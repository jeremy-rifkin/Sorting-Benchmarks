use std::time::Instant;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use regex::Regex;
use prettytable::*;

mod algos;
mod sort;
mod utils;

const SEED: u64 = 2222;

#[derive(Clone)]
#[derive(Debug)]
struct BenchmarkResult {
	mean: f64,
	stdev: f64,
	count: usize
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
	let mut rng = SmallRng::seed_from_u64(SEED);
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
		// flush cpu cache
		//destroy_cache();
		// perform test
		let start = Instant::now();
		sort(&mut test_vectors[i]);
		results.push(start.elapsed().as_nanos() as u64);
		// update counters
		completed += 1;
		running_sum += results[i];
		// if runtime exceeds 10 seconds...
		if running_sum >= 10e9 as u64 {
			break;
		}
	}
	// if not enough tests completed, return None
	if completed < 30 {
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
	let q = utils::quartiles(&results);
	//let __results = results.clone(); // debug stuff
	let results: Vec<u64> = results.into_iter()
							.filter(|item| utils::tukey(*item, &q, 3.0))
							.collect();
	let mean = results.iter().sum::<u64>() as f64 / results.len() as f64;
	let stdev = utils::stdev(&results, mean);

	// debug stuff
	//let mean = running_sum as f64 / completed as f64;
	//let v = mean / 1e6;
	//// +/- 1.96 standard deviations = 95% CI
	//let ci = 1.96 * utils::stdev(&__results[..completed], mean) / 1e6 / (completed as f64).sqrt();
	//if ci / v * 100.0 >= 10.0 {
	//	println!("{:.0}% {:?}", ci / v * 100.0, __results);
	//	println!("{:?}", q);
	//	println!("{} {} {:?}", mean, stdev, results);
	//}

	Option::Some(BenchmarkResult {
		mean,
		stdev,
		count: results.len()
	})
}

fn main() {
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
		sfn!(algos::quicksort_end::<i32>,            "O(n log n)"),
		sfn!(algos::quicksort_end_unsafe::<i32>,     "O(n log n)"),
		sfn!(algos::quicksort_random::<i32>,         "O(n log n)"),
		sfn!(algos::quicksort_hybrid::<i32>,         "O(n log n)"),
		sfn!(sort::weird::<i32>,                     "O(n^(3/2))"),
		sfn!(sort::rustsort::<i32>,                  "O(n log n)")
	];
	// run tests
	let mut results = vec![Vec::new(); algorithms.len()]; // 2d matrix of results
	let mut header = vec![String::from("")]; // start building the header now
	let mut test_size = 10;
	let n_tests = 200;
	// This flag array is to prevent algorithms from being benchmarked after a certain point - don't
	// want to run bubblesort on a million items.
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= 1_000_000 {
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
				let result = result.as_ref().unwrap();
				let v = result.mean / 1e6;
				// +/- 1.96 standard deviations = 95% CI
				let ci = 1.96 * result.stdev / 1e6 / (result.count as f64).sqrt();
				let s = format!("{:.5} ± {:.5} ({:.0}%)", v, ci, ci / v * 100.0);
				row.push(Cell::new(&s));
			}
		}
		table.add_row(Row::new(row));
	}
	table.printstd();
	println!("└ Values in ms; 95% confidence interval displayed");
}
