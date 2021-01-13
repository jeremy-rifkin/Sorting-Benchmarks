#![cfg(not(tarpaulin_include))] // this file should be excluded from test coverage

use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use lazy_static::lazy_static;
use num_cpus;
use prettytable::*;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng, seq::SliceRandom};
use regex::Regex;

mod algos;
mod odd_algos;
mod statistics;
mod utils;
mod swap_unsafe;
mod tests;

//
// This is the driver code for the algorithm benchmarking.
//
// Benchmarking is hard. As Emery Berger points out in his talk "Performance Matters"
// (https://www.youtube.com/watch?v=r-TLSBdHe1A) there are a ton of complex hardware and software
// factors that effect application performance. These include caching, branch prediction, random
// layout of executable binary in memory.
//
// These factors are really hard to address. Berger presented a tool called Coz which is aimed at
// application profiling and identifying targets for performance improvement. Berger also presented
// a tool called stabilizer which modifies executable layout during runtime to eliminate the effect
// of linker layout. The effects of code layout on performance are largely due to cache / branch
// predictor conflict misses (if frequently used pieces of code happen to conflict). I don't know
// how substantially this effects modern cpus (which tend to use associative caches), however, it
// would explain some anomalies we've observed while working on this. I'd like to use stabilizer,
// however, it is a compile-time plugin for C++ and I think getting it to work with Rust is outside
// the scope of this project.
//
// Here's some of the issues we've encountered:
//  - In 200 runs with mean runtime ~2,200ns, there was an outlier of 112,600ns. This result blew up
//    the standard deviation calculation. This happened on a couple algorithms on a couple
//    test-sizes a couple runs but there was no pattern to which algorithms or when it would happen.
//    Because of the extreme nature of these outliers, the outliers are simply discarded.
//  - We'd get really tight distributions (98% CI == +/- 0%) for the performance of some algorithms
//    however, on the next execution of the program, even without code changes, we'd get
//    substantially different results. The p-value of a 2-sample t-test for these instances is 0.
//
// Here are some of the factors contributing to benchmarking challenges:
//  - Cache
//  - Branch prediction
//  - Linker layout
//  - Os task scheduling
//  - Locations of objects in the heap?
//  - CPU throttling
//
// Here are some of the techniques used for mitigation:
//  - Instead of running all insertion sorts size=1,000 then all selection sorts size=1,000 etc. and
//    everything sequentially, every single individual algorithm call is setup and shuffled. Then a
//    thread pool will begin performing benchmarks from the problem pool. This is an attempt to
//    improve independence between each algorithm runs.
//  - OS calls are made to request preferential scheduling. This should improve consistency.
//  - Threads sleep between benchmark runs and only N_Cores / 2 threads are spun up. This is to help
//    prevent thermal throttling and improve consistency of cache performance.
//
// We experimented with running a cache buster between every benchmark execution (writing to a
// massive block of memory to flush out the cache). This has been discarded because it was not
// highly effective at addressing benchmarking issues, was very slow, and would be problematic in a
// multi-threaded context.
//
// Rust performs boundary checks on every array access. This causes a substantial performance hit
// (and may effect branch prediction). The various sorting algorithms we've implemented are much
// slower than rust's built-in sorting algorithms. This is partially due to unsafe array accesses
// (rust's built-in algorithms use unsafe accesses to disable boundary checks), and we aim to have
// everything implemented with unsafe accesses. Rust's sorting algorithms are also faster because
// they are advanced hybrid sorting algorithms and have had much more work put into optimizing them
// than we've put into optimizing ours. We hope to get our algorithms to have comparable performance
// to rust's builtin, but it isn't strictly necessary for our program: we just want to look at how
// algorithms compare to each other on real hardware.
//
// Note: The benchmarking takes a while to run, but it's thorough
//

// this flag will disable a lot of algorithms and reduce the number of tests performed
// this is for use during development
const TEST_MODE: bool = false;

const MIN_TEST_SIZE: usize = 10;
#[cfg(not(arm11))]
const MAX_TEST_SIZE: usize = if !TEST_MODE { 1_000_000 } else { 10_000 };
#[cfg(arm11)]
const MAX_TEST_SIZE: usize = if !TEST_MODE { 100_000   } else { 10_000 };
const N_TESTS: usize =       if !TEST_MODE { 200       } else { 50     };
const ALPHA: f64 = 0.001;
const DIFF_THRESHOLD: f64 = 0.05;

const RNG_SEED: u64 = 2222;
const RUNTIME_LIMIT: u64 = 10e9 as u64;
const MIN_ACCEPTABLE_TESTS: usize = 30;
const OUTLIER_COEFFICIENT: f64 = 3.0;

lazy_static! {
	// don't want to run bubblesort on a million items (or 100,000 items for that matter)
	static ref LIMIT_TABLE: HashMap<&'static str, usize> = {
		let mut table = HashMap::new();
		table.insert("O(n^2)", 10_000);
		table.insert("O(n^(4/3))", usize::MAX);
		table.insert("O(n^(3/2))", usize::MAX);
		table.insert("O(n log n)", usize::MAX);
		table.insert("O(n)", usize::MAX); // radixsort
		table
	};
}

lazy_static! {
	static ref TEST_SIZES: Vec<usize> = {
		let mut v = Vec::new();
		let mut test_size = MIN_TEST_SIZE;
		while test_size <= MAX_TEST_SIZE {
			v.push(test_size);
			test_size *= 10;
		}
		v
	};
}

lazy_static! {
	static ref N_WORKERS: usize = {
		//num_cpus::get_physical() - 1
		num_cpus::get_physical() / 2
	};
}

#[derive(Clone, Copy, Debug, Default)]
struct BenchmarkResult {
	mean: f64,
	stdev: f64,
	count: usize,
	is_stat_fastest: bool,
	is_fastest: bool
}

impl std::fmt::Display for BenchmarkResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		// TODO: show within 5% of min as well?
		let v = self.mean / 1e6;
		let ci = self.t_ci();
		write!(f,
			"{:.5} ± {:.5} ({:.0}%) {} {}",
				v,     ci, ci / v * 100.0,
				if self.is_stat_fastest {"s"} else {" "},
				if self.is_fastest      {"*"} else {" "}
			)
	}
}

impl BenchmarkResult {
	// returns a p-value and a percent difference (based off of the smaller mean)
	fn compare(&self, other: &BenchmarkResult) -> (f64, f64) {
		// percent diff
		let min = utils::fmin(self.mean, other.mean);
		let max = utils::fmax(self.mean, other.mean);
		let diff = (max - min) / min;
		// p value
		if self.stdev == 0.0 || other.stdev == 0.0 { // will cause problems / infinite / nan values
			return (0.0, diff); // I guess?
		}
		let p = statistics::two_sample_t_test(self.mean, other.mean,
											  self.stdev, other.stdev,
											  self.count, other.count, true);
		//println!("{}", p);
		assert!(!p.is_nan(), "problematic value: {}", p);
		assert!(!p.is_infinite(), "problematic value: {}", p);
		assert!(!p.is_sign_negative(), "problematic value: {}", p);
		assert!(p <= 1.0, "problematic value: {}", p);
		(p, diff)
	}
	fn update_display(&mut self, other: &BenchmarkResult) {
		let (p, diff) = self.compare(other);
		if p >= ALPHA {
			self.is_stat_fastest = true;
		}
		if diff <= DIFF_THRESHOLD {
			self.is_fastest = true;
		}
	}
	fn reset_display(&mut self) {
		self.is_stat_fastest = false;
		self.is_fastest = false;
	}
	#[allow(dead_code)]
	fn z_ci(&self) -> f64 {
		// +/- 1.96 standard deviations = 95% CI
		1.96 * self.stdev / 1e6 / (self.count as f64).sqrt()
	}
	fn t_ci(&self) -> f64 {
		// returns 98% confidence interval
		statistics::t_lookup(self.count as i32 - 1) * self.stdev / 1e6 / (self.count as f64).sqrt()
	}
}

#[derive(PartialEq)]
enum MType {
	IdAssignment,
	WorkAssignment
}

#[derive(Clone, Copy)]
struct WorkDescriptor {
	algorithm_i: usize,
	size: usize,
	test_i: usize
}

union MContents {
	id: usize,
	work: WorkDescriptor
}

struct MPMessage {
	m_type: MType,
	contents: MContents
}

impl MPMessage {
	pub fn new_id_message(id: usize) -> MPMessage {
		MPMessage {
			m_type: MType::IdAssignment,
			contents: MContents { id }
		}
	}
	pub fn new_work_message(work: WorkDescriptor) -> MPMessage {
		MPMessage {
			m_type: MType::WorkAssignment,
			contents: MContents { work }
		}
	}
	pub fn get_id_message(&self) -> usize {
		assert!(self.m_type == MType::IdAssignment);
		unsafe { self.contents.id }
	}
	pub fn get_work_message(&self) -> WorkDescriptor {
		assert!(self.m_type == MType::WorkAssignment);
		unsafe { self.contents.work }
	}
}

struct BenchmarkManager {
	algorithms: Vec<(fn(&mut [i32]), String, &'static str)>,
	results_table: Vec<Vec<Option<BenchmarkResult>>>
}

impl BenchmarkManager {
	pub fn new() -> BenchmarkManager {
		let algorithms: Vec<(fn(&mut [i32]), String, &str)> =
		if !TEST_MODE {
			 vec![
				sfn!(algos::bubblesort::<i32>,               "O(n^2)"),
				sfn!(algos::cocktail_shaker::<i32>,          "O(n^2)"),
				sfn!(algos::selectionsort::<i32>,            "O(n^2)"),
				sfn!(odd_algos::selectionsort_cocktail::<i32>,"O(n^2)"),
				sfn!(odd_algos::selectionsort_minmax::<i32>, "O(n^2)"),
				sfn!(algos::insertionsort::<i32>,            "O(n^2)"),
				sfn!(algos::insertionsort_boundary_checked::<i32>,"O(n^2)"),
				sfn!(algos::insertionsort_c,                 "O(n^2)"),
				sfn!(algos::shellsort_knuth::<i32>,          "O(n^(4/3))"),
				sfn!(algos::shellsort_sedgewick82::<i32>,    "O(n^(4/3))"),
				sfn!(algos::shellsort_sedgewick86::<i32>,    "O(n^(4/3))"),
				sfn!(algos::shellsort_gonnet_baeza::<i32>,   "O(n^(4/3))"),
				sfn!(algos::shellsort_tokuda::<i32>,         "O(n^(4/3))"),
				sfn!(algos::shellsort_ciura::<i32>,          "O(n^(4/3))"),
				sfn!(odd_algos::shellsort_alternative_ciura::<i32>,"O(n^(4/3))"),
				sfn!(algos::mergesort_pre_alloc::<i32>,      "O(n log n)"),
				sfn!(algos::mergesort_repeated_alloc::<i32>, "O(n log n)"),
				sfn!(algos::mergesort_hybrid::<i32>,         "O(n log n)"),
				sfn!(algos::mergesort_in_place_naive::<i32>, "O(n^2)"),
				sfn!(algos::mergesort_in_place::<i32>,       "O(n log n)"),
				sfn!(odd_algos::mergesort_adaptive::<i32>,   "O(n log n)"),
				sfn!(odd_algos::mergesort_double_hybrid::<i32>,"O(n log n)"),
				sfn!(algos::heapsort_bottom_up::<i32>,       "O(n log n)"),
				sfn!(algos::heapsort_top_down::<i32>,        "O(n log n)"),
				sfn!(algos::quicksort_end::<i32>,            "O(n log n)"),
				sfn!(algos::quicksort_random::<i32>,         "O(n log n)"),
				sfn!(algos::quicksort_hybrid::<i32>,         "O(n log n)"),
				sfn!(odd_algos::btreesort::<i32>,            "O(n log n)"),
				sfn!(odd_algos::weird::<i32>,                "O(n^(3/2))"),
				sfn!(algos::radixsort,                       "O(n)"),
				sfn!(algos::rustsort::<i32>,                 "O(n log n)"),
				sfn!(algos::rustsort_unsable::<i32>,         "O(n log n)"),
				sfn!(algos::cpp_std_sort,                    "O(n log n)")
			]
		} else {
			vec![
				sfn!(algos::bubblesort::<i32>,               "O(n^2)"),
				sfn!(algos::cocktail_shaker::<i32>,          "O(n^2)"),
				sfn!(algos::selectionsort::<i32>,            "O(n^2)"),
				sfn!(algos::insertionsort::<i32>,            "O(n^2)"),
				sfn!(algos::insertionsort_boundary_checked::<i32>,"O(n^2)"),
				sfn!(algos::insertionsort_c,                 "O(n^2)"),
				sfn!(algos::shellsort_knuth::<i32>,          "O(n^(4/3))"),
				sfn!(algos::shellsort_sedgewick82::<i32>,    "O(n^(4/3))"),
				sfn!(algos::mergesort_pre_alloc::<i32>,      "O(n log n)"),
				sfn!(algos::mergesort_hybrid::<i32>,         "O(n log n)"),
				sfn!(algos::mergesort_in_place::<i32>,       "O(n log n)"),
				sfn!(algos::heapsort_top_down::<i32>,        "O(n log n)"),
				sfn!(algos::quicksort_end::<i32>,            "O(n log n)"),
				sfn!(algos::quicksort_hybrid::<i32>,         "O(n log n)"),
				sfn!(algos::radixsort,                       "O(n)"),
				sfn!(algos::rustsort::<i32>,                 "O(n log n)"),
				sfn!(algos::cpp_std_sort,                    "O(n log n)")
			]
		};
		// TODO: single vec serving as 2D array? algorithms[i][j] = results[i * len + j]
		let results_table = vec![vec![Option::None; TEST_SIZES.len()]; algorithms.len()];
		BenchmarkManager {
			algorithms,
			results_table
		}
	}
	fn seedgen(n: usize) -> u64 {
		let n = n as u64;
		// in the old benchmark code we'd use an rng source with the fixed seed to generate N_TESTS
		// test vectors
		// this would give us N_TESTS unique (probably) test vectors, but the values were
		// predictable and constant across all tests
		// in order to get unique but consistent test vectors with this situation, we use a basic
		// method to create a seed based off of the current test we're on
		// could just do RNG_SEED + n but why do that when we can do it complicated
		// note: previously looked into creating a rng with RNG_SEED for every cell in the table
		// and passing those to threads with the various jobs, but, that would be complicated as far
		// as lifetimes go and would introduce concurrency problems
		// this way we don't have to avoid scheduling two sub-runs of (alg, size) at once
		let mut seed = RNG_SEED;
		for i in 0..n {
			seed = (seed.rotate_left(1) + n) ^ i ^ RNG_SEED;
		}
		seed
	}
	fn run_bench(sort: fn(&mut [i32]), size: usize, test_i: usize) -> u64 {
		// setup the test itself based off seed for this particular run
		let mut test_vector: Vec<i32> = vec![0; size];
		let mut rng = SmallRng::seed_from_u64(BenchmarkManager::seedgen(test_i));
		for i in 0..size {
			test_vector[i] = rng.next_u32() as i32;
		}
		// sleep briefly - this is an attempt to produce more constant results
		if !TEST_MODE { thread::sleep(Duration::from_millis(10)) };
		// test body:
		let start = Instant::now();
		sort(&mut test_vector);
		let r = start.elapsed().as_nanos() as u64;
		// this is covered in test cases but just to be sure...
		utils::verify_sorted(&test_vector);
		r
	}
	fn generate_benchmark_jobs(&self) -> Vec<(usize, usize, usize)> {
		let mut jobs = Vec::new();
		for size_i in 0..TEST_SIZES.len() {
			for (i, a) in self.algorithms.iter().enumerate() {
				if TEST_SIZES[size_i] <= *LIMIT_TABLE.get(&a.2).unwrap_or(&usize::MAX) {
					for n in 0..N_TESTS {
						jobs.push((i, size_i, n));
					}
				}
			}
		}
		jobs
	}
	fn get_next_job(&self, time_table: &Vec<Vec<u64>>, jobs: &mut Vec<(usize, usize, usize)>)
		-> Option<(usize, usize, usize)> {
		// fetch a new job while discarding any jobs whose predecessors have exceeded the runtime
		// limit
		while !jobs.is_empty() {
			let job = jobs.pop().unwrap();
			if time_table[job.0][job.1] >= RUNTIME_LIMIT {
				// discard job and continue
			} else {
				return Option::Some(job);
			}
		}
		return Option::None;
	}
	fn compute_results(&mut self, results: Vec<Vec<Vec<u64>>>) {
		for algorithm_i in 0..self.algorithms.len() {
			for size_i in 0..TEST_SIZES.len() {
				// compute stats
				// We had an issue with a few benchmarks randomly having massive standard deviations
				// every time we'd run the benchmark just a couple results would have anomalies and
				// there wasn't any consistency or pattern to which benchmarks would have anomalies.
				// The reason for the massive standard deviations was due to just a couple (usually
				// just 1) extraneous results in the results vector (e.g. in a benchmark whose
				// result's mean was ~2,200ns, there was an outlier of 112,600ns blowing up the
				// standard deviation calculation). Here we use Tukey's method to discard outliers.
				// note results is shadowed twice here
				let results = &results[algorithm_i][size_i];
				if results.len() != N_TESTS {
					println!("---------->> {} {} {}", self.algorithms[algorithm_i].1,
													  utils::commafy(TEST_SIZES[size_i]),
													  results.len());
				}
				if results.len() <= MIN_ACCEPTABLE_TESTS {
					// either not enough tests were performed within the runtime limit or no tests
					// were performed because of complexity limits
					self.results_table[algorithm_i][size_i] = Option::None;
					continue;
				}
				let q = statistics::quartiles(&results);
				let results: Vec<u64> = results.into_iter()
										.map(|item| *item)
										.filter(|item| statistics::tukey(*item, &q,
																			OUTLIER_COEFFICIENT))
										.collect();
				let mean = results.iter().sum::<u64>() as f64 / results.len() as f64;
				let stdev = statistics::stdev(&results, mean);
				self.results_table[algorithm_i][size_i] = Option::Some(BenchmarkResult {
					mean,
					stdev,
					count: results.len(),
					// fields will be used in display code
					is_fastest: false,
					is_stat_fastest: false
				});
			}
		}
	}
	pub fn run_benchmarks(&mut self) {
		// thread strategy:
		//  * spawn physical cores - 1 threads to perform benchmarking
		//  * treat as a worker pool
		//  * sleep between benchmarks
		// message passing strategy:
		//  * channel cloned for all threads to send data back to main / coordinator thread
		//  * thread-specific channels created to send messages from the coordinator to a specific
		//    thread
		//  * threads are given their thread ids through message passing on startup
		//  * when there is no more work to give to a thread, the tx will be dropped aborting the
		//    thread's rx loop
		//  * when all threads exit they'll drop their tx eventually ending the coordinator rx loop
		let (coordinator_tx, coordinator_rx) = mpsc::channel();
		let mut threads = Vec::new();
		let mut channels: Vec<Option<mpsc::Sender<MPMessage>>> = Vec::new();
		for i in 0..*N_WORKERS {
			// the shadowing here is weird
			let coordinator_tx = mpsc::Sender::clone(&coordinator_tx);
			// tx moved into the channels vector
			// rx is moved into the thread
			// the cloned coordinator_tx is also moved into the thread
			let (tx, rx) = mpsc::channel();
			channels.push(Option::Some(tx));
			// the thread needs a pointer to the struct instance and I can't pass &self because its
			// lifetime is not &'static. There's surely a better rustic way to do this but I'm just
			// going to cast away the lifetime constraint.
			// this is safe because we know these threads won't live past the end of this method
			let self_ptr = unsafe { u_extend_lifetime!(self) };
			threads.push(thread::spawn(move || {
				// get thread id via rx
				let id = rx.recv().unwrap().get_id_message();
				// request higher thread priority
				// TODO: this actually isn't making much difference...
				//utils::set_thread_priority_max();
				// kickstart the process by requesting work
				coordinator_tx.send((id, u64::MAX)).unwrap();
				// begin work loop
				for received in rx {
					if received.m_type == MType::WorkAssignment {
						let job = received.get_work_message();
						let result = BenchmarkManager::run_bench(
							self_ptr.algorithms[job.algorithm_i].0,
							TEST_SIZES[job.size],
							job.test_i
						);
						coordinator_tx.send((id, result)).unwrap();
					} else {
						panic!("unexpected non-WorkAssignment message received in worker");
					}
				}
			}));
			channels.last().unwrap().as_ref().unwrap().send(MPMessage::new_id_message(i)).unwrap();
		}
		// drop original coordinator_tx parent to allow detecting when all threads drop their clones
		drop(coordinator_tx);
		// setup the test cases
		// the size of this Vec is O(really big)
		// this vec is used like a stack - jobs are consumed from the top
		let mut jobs = self.generate_benchmark_jobs();
		println!("executing of jobs: {} on {} threads with max size = {}",
			utils::commafy(jobs.len()),
			*N_WORKERS,
			utils::commafy(MAX_TEST_SIZE));
		// shuffle jobs using seed
		let mut rng = SmallRng::seed_from_u64(RNG_SEED);
		jobs.shuffle(&mut rng);
		// we have to keep track of every thread's current job so we know how to assign its output
		// it's a little ugly and non-elegant. the alternative is to include job info in the thread
		// result return
		let mut assignments = vec![Option::<(usize, usize, usize)>::None; *N_WORKERS];
		// our final results will be Vec<Vec<Option<BenchmarkResult>>> but as we get the data needed
		// for these jobs, we have to store in a Vec<Vec<Vec<u64>>>
		let mut results = vec![
							   vec![Vec::<u64>::with_capacity(N_TESTS); TEST_SIZES.len()];
						  self.algorithms.len()];
		// keep track of time spent on each cell
		// could just sum results, but may as well keep running sums in this table
		let mut time_table = vec![vec![0u64; TEST_SIZES.len()]; self.algorithms.len()];
		// receive loop
		// goal:
		//   recieve results from the worker threads
		//   log
		//   dispatch new work
		//   handle teardown
		// loop will break when all threads have reported with their final results and had their
		// channels torn down
		for received in coordinator_rx {
			let (thread_id, result) = received;
			if result == u64::MAX /* && assignments[thread_id].is_none() */ {
				// handle initial work request / kickstart
				assert!(assignments[thread_id].is_none());
				// no action needed - just proceed to work dispatch
			} else {
				// else log result from worker
				let (algorithm_i, size_i, _) = assignments[thread_id].unwrap();
				results[algorithm_i][size_i].push(result);
				time_table[algorithm_i][size_i] += result;
			}
			// dispatch new work or teardown
			if let Option::Some(job) = self.get_next_job(&time_table, &mut jobs) {
				println!("{} {} {}", utils::commafy(jobs.len()),
									 self.algorithms[job.0].1,
									 utils::commafy(TEST_SIZES[job.1]));
				channels[thread_id].as_ref()
								   .unwrap()
								   .send(MPMessage::new_work_message(WorkDescriptor {
									   algorithm_i: job.0,
									   size: job.1,
									   test_i: job.2
								   })).unwrap();
				assignments[thread_id] = Option::Some(job);
			} else {
				let c = channels[thread_id].take().unwrap();
				channels[thread_id] = Option::None;
				drop(c);
				// could set thread's assignment to none but it doesn't matter
			}
		}
		// join all threads
		println!("joining");
		for thread in threads {
			thread.join().unwrap();
		}
		// compute final results
		self.compute_results(results);
	}
	pub fn run_benchmarks_single_threaded(&mut self) {
		// for systems like the raspberry pi zero
		// TODO: this actually isn't making much difference...
		//utils::set_thread_priority_max();
		// setup the test cases
		// the size of this Vec is O(really big)
		// this vec is used like a stack - jobs are consumed from the top
		let mut jobs = self.generate_benchmark_jobs();
		println!("executing of jobs: {} on single-threaded with max size = {}",
			utils::commafy(jobs.len()),
			utils::commafy(MAX_TEST_SIZE));
		// shuffle jobs using seed
		let mut rng = SmallRng::seed_from_u64(RNG_SEED);
		jobs.shuffle(&mut rng);
		// our final results will be Vec<Vec<Option<BenchmarkResult>>> but as we get the data needed
		// for these jobs, we have to store in a Vec<Vec<Vec<u64>>>
		let mut results = vec![
							   vec![Vec::<u64>::with_capacity(N_TESTS); TEST_SIZES.len()];
						  self.algorithms.len()];
		// keep track of time spent on each cell
		// could just sum results, but may as well keep running sums in this table
		let mut time_table = vec![vec![0u64; TEST_SIZES.len()]; self.algorithms.len()];
		// job loop
		while let Option::Some(job) = self.get_next_job(&time_table, &mut jobs) {
			println!("{} {} {}", utils::commafy(jobs.len()),
								 self.algorithms[job.0].1,
								 utils::commafy(TEST_SIZES[job.1]));
			let result = BenchmarkManager::run_bench(
				self.algorithms[job.0].0,
				TEST_SIZES[job.1],
				job.2
			);
			results[job.0][job.1].push(result);
			time_table[job.0][job.1] += result;
		}
		// compute final results
		self.compute_results(results);
	}
	pub fn print(&mut self, filter: fn(&String, &str) -> bool) {
		// mins
		for i in 0..TEST_SIZES.len() {
			let mut min_mean = Option::<f64>::None;
			let mut min_result = Option::<usize>::None;
			for j in 0..self.algorithms.len() {
				if filter(&self.algorithms[j].1, self.algorithms[j].2) {
					let ar = &self.results_table[j][i];
					if ar.is_some() {
						let ar = ar.as_ref().unwrap();
						if min_mean.is_none() || ar.mean < min_mean.unwrap() {
							min_mean = Option::Some(ar.mean);
							min_result = Option::Some(j);
						}
					}
				}
			}
			if min_result.is_some() {
				let min_j = min_result.unwrap();
				self.results_table[min_j][i].as_mut().unwrap().is_fastest = true;
				let min = self.results_table[min_j][i].clone().unwrap();
				for j in 0..self.algorithms.len() {
					if filter(&self.algorithms[j].1, self.algorithms[j].2) {
						if self.results_table[j][i].is_some() {
							let ar = self.results_table[j][i].as_mut().unwrap();
							ar.update_display(&min);
						}
					}
				}
			}
		}
		// make pretty table
		let mut table = Table::new();
		table.add_row(Row::new(std::iter::once(String::from(""))
								.chain(TEST_SIZES
										.iter()
										.map(|x| utils::commafy(*x)))
								.map(|x| Cell::new(&x)).collect()));
		for (i, a) in self.algorithms.iter().enumerate() {
			if filter(&a.1, a.2) {
				let mut row = vec![Cell::new(&a.1)];
				for result in self.results_table[i].iter() {
					if result.is_none() {
						row.push(Cell::new("-"));
					} else {
						row.push(Cell::new(&format!("{}", result.as_ref().unwrap())));
					}
				}
				table.add_row(Row::new(row));
			}
		}
		table.printstd();
		println!("└ Values in ms; 98% confidence interval displayed; \
					s = statistically equal to fastest; * = within 5% of fastest");
		// reset mins / maxes
		for a in &mut self.results_table {
			for b in a {
				if b.is_some() {
					b.as_mut().unwrap().reset_display();
				}
			}
		}
	}
}

#[cfg(test)]
mod test {
	#[test]
	fn test_seedgen() {
		use std::collections::HashSet;
		use super::*;
		// This test ensures that seedgen properly generates unique seeds
		let mut set = HashSet::new();
		for n in 0..N_TESTS {
			assert!(set.insert(BenchmarkManager::seedgen(n)));
		}
		println!("{:?}", set);
		assert!(set.len() == N_TESTS);
	}
}

fn main() {
	let mut manager = BenchmarkManager::new();
	let start = Instant::now();
	if *N_WORKERS < 2 {
		manager.run_benchmarks_single_threaded();
	} else {
		manager.run_benchmarks();
	}
	let runtime = start.elapsed();

	println!("Bubble sorts:");
	manager.print(|n, _| n.contains("bubble") || n.contains("cocktail"));
	println!();

	println!("Insertion sorts:");
	manager.print(|n, _| n.contains("insertion")
						|| n.contains("selection")
						|| n.contains("cocktail"));
	println!();

	println!("Shell sorts:");
	manager.print(|n, _| n.contains("shellsort") || n.contains("insertionsort"));
	println!();

	println!("Merge sorts:");
	manager.print(|n, _| n.contains("mergesort"));
	println!();

	println!("Heap sorts:");
	manager.print(|n, _| n.contains("heapsort"));
	println!();

	println!("Quick sorts:");
	manager.print(|n, _| n.contains("quicksort"));
	println!();

	println!("Radix sort:");
	manager.print(|n, _| n.contains("radix") || n.contains("rustsort"));
	println!();

	println!("Totals:");
	manager.print(|n, _| !n.contains("radix"));

	println!("\nRuntime: {}", utils::duration_to_human(runtime));

	return;
}
