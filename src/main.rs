use std::thread;
use std::time::{Duration, Instant};

use prettytable::*;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use regex::Regex;

mod algos;
mod statistics;
mod utils;

const MIN_TEST_SIZE: usize = 10;
const MAX_TEST_SIZE: usize = 1000; //1_000_000;
const N_TESTS: usize = 200;
const ALPHA: f64 = 0.001;

const RNG_SEED: u64 = 2222;
const RUNTIME_LIMIT: u64 = 10e9 as u64;
const MIN_ACCEPTABLE_TESTS: usize = 30;
const OUTLIER_COEFFICIENT: f64 = 3.0;

#[derive(Clone, Debug)]
struct BenchmarkResult {
	mean: f64,
	stdev: f64,
	count: usize,
	is_fastest: bool
}

impl std::fmt::Display for BenchmarkResult {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let v = self.mean / 1e6;
		let ci = self.t_ci();
		write!(f, "{:.5} ± {:.5} ({:.0}%){}", v, ci, ci / v * 100.0, if self.is_fastest {" *"} else {""})
	}
}

impl BenchmarkResult {
	// returns a p-value
	fn compare(&self, other: &BenchmarkResult) -> f64 {
		let p = statistics::two_sample_t_test(self.mean, other.mean, self.stdev, other.stdev, self.count, other.count, true);
		//println!("{}", p);
		assert!(!p.is_nan(), "problematic value: {}", p);
		assert!(!p.is_infinite(), "problematic value: {}", p);
		assert!(!p.is_sign_negative(), "problematic value: {}", p);
		assert!(p <= 1.0, "problematic value: {}", p);
		p
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

// flush cpu cache between benchmarks
// 20M / 4 bytes (i32)
const DESTROYER_SIZE: usize = 20_000_000 / 4;
#[allow(dead_code)]
fn destroy_cache() {
	let mut a = vec![0; DESTROYER_SIZE];
	for i in 0..DESTROYER_SIZE {
		a[i] = i as i32;
	}
}

// benchmarks a sorting algorithm
// args
//  sort: function pointer for the algorithm to use
//  size: array size to test
//  n_tests: maximum number of tests to perform
// return
//  None if less than 30 tests could be completed within 10seconds (excluding cache buster and setup)
//  BenchmarkResult otherwise
// None return is used as a flag in the driver code to stop benchmarking an algorithm after it
// becomes unbearably slow.
fn bench(sort: fn(&mut [i32]), size: usize, n_tests: usize) -> Option<BenchmarkResult> {
	// setup tests
	let mut test_vectors: Vec<Vec<i32>> = vec![vec![0; size]; n_tests];
	let mut rng = SmallRng::seed_from_u64(RNG_SEED);
	for i in 0..n_tests {
		for j in 0..size {
			test_vectors[i][j] = rng.next_u32() as i32;
		}
	}
	// run tests
	let mut results: Vec<u64> = Vec::with_capacity(n_tests);
	let mut running_sum = 0u64; // running sum of the results
	let mut completed = 0; // tests completed counter
	for i in 0..n_tests {
		thread::sleep(Duration::from_millis(10));
		// flush cpu cache
		destroy_cache();
		// perform test
		let start = Instant::now();
		sort(&mut test_vectors[i]);
		results.push(start.elapsed().as_nanos() as u64);
		// update counters
		completed += 1;
		running_sum += results[i];
		// if runtime exceeds 10 seconds...
		if running_sum >= RUNTIME_LIMIT {
			break;
		}
	}
	// if not enough tests completed, return None
	if completed < MIN_ACCEPTABLE_TESTS {
		return Option::None;
	}
	// verify correctness just for good measure
	for i in 0..completed {
		utils::verify_sorted(&test_vectors[i]);
	}
	// compute stats
	// We had an issue with a few benchmarks randomly having massive standard deviations every time
	// we'd run the benchmark just a couple results would have anomalies and there wasn't any
	// consistency or pattern to which benchmarks would have anomalies.
	// The reason for the massive standard deviations was due to just a couple (usually just 1)
	// extraneous results in the results vector (e.g. in a benchmark whose result's mean was
	//  ~2,200ns, there was an outlier of 112,600ns blowing up the standard deviation calculation).
	// This filter here uses Tukey's method to filter out outliers.
	let q = statistics::quartiles(&results);
	let results: Vec<u64> = results.into_iter()
							.filter(|item| statistics::tukey(*item, &q, OUTLIER_COEFFICIENT))
							.collect();
	let mean = results.iter().sum::<u64>() as f64 / results.len() as f64;
	let stdev = statistics::stdev(&results, mean);
	Option::Some(BenchmarkResult {
		mean,
		stdev,
		count: results.len(),
		is_fastest: false // field will be used for display
	})
}

fn find_limits(algorithms: &Vec<(fn(&mut [i32]), String, &str)>) -> Vec<usize> {
	let mut rng = SmallRng::seed_from_u64(RNG_SEED ^ 0xF00D);
	let mut test_size = MIN_TEST_SIZE;
	let n_tests = 4; // we'll take the best of 4
	let mut test_vectors: Vec<Vec<i32>> = vec![Vec::new(); n_tests];
	let mut limits = vec![0; algorithms.len()];
	// This flag array is to prevent algorithms from being benchmarked after a certain point - don't
	// want to run bubblesort on a million items.
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= MAX_TEST_SIZE {
		// update test vectors
		for i in 0..n_tests {
			while test_vectors[i].len() < test_size {
				test_vectors[i].push(rng.next_u32() as i32);
			}
		}
		for (i, item) in algorithms.iter().enumerate() {
			if algorithm_enable_flags[i] {
				println!("{} {}", item.1, utils::commafy(test_size));
				// run 4 tests
				let mut min = u64::MAX;
				for j in 0..n_tests {
					print!(".");
					thread::sleep(Duration::from_millis(10)); // TODO
					let start = Instant::now();
					item.0(&mut test_vectors[j].clone());
					min = std::cmp::min(min, start.elapsed().as_nanos() as u64);
					if min > RUNTIME_LIMIT / MIN_ACCEPTABLE_TESTS as u64 {
						break;
					}
				}
				print!("\n");
				if min <= RUNTIME_LIMIT / MIN_ACCEPTABLE_TESTS as u64 {
					limits[i] = test_size;
				} else {
					// don't test any further
					algorithm_enable_flags[i] = false;
				}
			}
		}
		test_size *= 10;
	}
	limits
}

fn main() {
	utils::set_priority();
	let algorithms: Vec<(fn(&mut [i32]), String, &str)> = vec![
		sfn!(algos::bubblesort::<i32>,               "O(n^2)"),
		sfn!(algos::bubblesort_unsafe::<i32>,        "O(n^2)"),
		sfn!(algos::cocktail_shaker::<i32>,          "O(n^2)"),
		sfn!(algos::selectionsort::<i32>,            "O(n^2)"),
		sfn!(algos::insertionsort::<i32>,            "O(n^2)"),
		sfn!(algos::insertionsort_unsafe::<i32>,     "O(n^2)"),
		sfn!(algos::shell::<i32>,                    "O(n^(4/3))"),
		sfn!(algos::mergesort_pre_alloc::<i32>,      "O(n log n)"),
		sfn!(algos::mergesort_repeated_alloc::<i32>, "O(n log n)"),
		sfn!(algos::mergesort_hybrid::<i32>,         "O(n log n)"),
		sfn!(algos::mergesort_in_place_naive::<i32>, "O(n^2)"),
		sfn!(algos::mergesort_in_place::<i32>,       "O(n log n)"),
		sfn!(algos::heapsort_bottom_up::<i32>,       "O(n log n)"),
		sfn!(algos::heapsort_top_down::<i32>,        "O(n log n)"),
		sfn!(algos::quicksort_end::<i32>,            "O(n log n)"),
		sfn!(algos::quicksort_end_unsafe::<i32>,     "O(n log n)"),
		sfn!(algos::quicksort_random::<i32>,         "O(n log n)"),
		sfn!(algos::quicksort_hybrid::<i32>,         "O(n log n)"),
		sfn!(algos::weird::<i32>,                    "O(n^(3/2))"),
		sfn!(algos::rustsort::<i32>,                 "O(n log n)")
	];

	let limits = find_limits(&algorithms);
	for (i, entry) in algorithms.iter().enumerate() {
		println!("{} {}", entry.1, utils::commafy(limits[i]));
	}

	// run tests
	let mut results = vec![Vec::new(); algorithms.len()]; // 2d matrix of results
	let mut header = vec![String::from("")]; // start building the header now
	let mut test_size = MIN_TEST_SIZE;
	let n_tests = N_TESTS;
	// This flag array is to prevent algorithms from being benchmarked after a certain point - don't
	// want to run bubblesort on a million items.
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= MAX_TEST_SIZE {
		header.push(utils::commafy(test_size));
		// test every algorithm for this test size
		for (i, a) in algorithms.iter().enumerate() {
			println!("{} {}", utils::commafy(test_size), a.1);
			if algorithm_enable_flags[i] {
				let b = bench(a.0, test_size, n_tests);
				if b.is_none() {
					algorithm_enable_flags[i] = false;
				}
				results[i].push(b);
			} else {
				results[i].push(Option::<BenchmarkResult>::None);
			}
		}
		test_size *= 10;
	}
	// mins
	for i in 0..results[0].len() {
		let mut min_mean = Option::<f64>::None;
		let mut min_result = Option::<usize>::None;
		for j in 0..algorithms.len() {
			let ar = &results[j][i];
			if ar.is_some() {
				let ar = ar.as_ref().unwrap();
				if min_mean.is_none() || ar.mean < min_mean.unwrap() {
					min_mean = Option::Some(ar.mean);
					min_result = Option::Some(j);
				}
			}
		}
		if min_result.is_some() {
			let min_j = min_result.unwrap();
			results[min_j][i].as_mut().unwrap().is_fastest = true;
			let min = results[min_j][i].clone().unwrap();
			for j in 0..algorithms.len() {
				if results[j][i].is_some() {
					let ar = results[j][i].as_mut().unwrap();
					if min.compare(ar) >= ALPHA {
						ar.is_fastest = true;
					}
				}
			}
		}
	}
	// print a delimited table (helpful for pasting into excel)
	for cell in header.iter() {
		print!("{} ", cell);
	}
	print!("\n");
	for (i, a) in algorithms.iter().enumerate() {
		print!("{}", a.1);
		for result in results[i].iter() {
			if result.is_none() {
				print!(" -");
			} else {
				let result = result.as_ref().unwrap();
				let v = result.mean / 1e6;
				print!(" {:.5}", v);
			}
		}
		print!("\n");
	}
	// make pretty table
	let mut table = Table::new();
	table.add_row(Row::new(header.iter().map(|x| Cell::new(x)).collect()));
	for (i, a) in algorithms.iter().enumerate() {
		let mut row = vec![Cell::new(&a.1)];
		for result in results[i].iter() {
			if result.is_none() {
				row.push(Cell::new("-"));
			} else {
				row.push(Cell::new(&format!("{}", result.as_ref().unwrap())));
			}
		}
		table.add_row(Row::new(row));
	}
	table.printstd();
	println!("└ Values in ms; 95% confidence interval displayed");
}
