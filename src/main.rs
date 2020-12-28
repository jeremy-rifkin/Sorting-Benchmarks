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
	stdev: f64
}

// computes the sample standard deviation of a Vec<u64>
// mean passed as a parameter to take advantage of pre-computed value
fn stdev(array: &Vec<u64>, completed: usize, mean: f64) -> f64 {
	let mut sum = 0.0;
	for i in 0..completed {
		sum += (array[i] as f64 - mean).powi(2);
	}
	sum /= (completed - 1) as f64;
	sum.sqrt()
}

// flush cpu cache between benchmarks
// 20M / 4 bytes (i32)
const DESTROYER_SIZE: usize = 20_000_000 / 4;
fn destroy_cache() {
	let mut a = vec![0; DESTROYER_SIZE];
	for i in 0..DESTROYER_SIZE {
		a[i] = i as i32;
	}
}

// benchmarks a sorting algorithm
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
	let mut results: Vec<u64> = vec![0; n_tests];
	let mut running_sum = 0.0;
	let mut completed = 0;
	for i in 0..n_tests {
		destroy_cache();
		let start = Instant::now();
		sort(&mut test_vectors[i]);
		completed += 1;
		results[i] = start.elapsed().as_nanos() as u64;
		running_sum += results[i] as f64;
		if running_sum >= 10e9 {
			break;
			//return Option::None;
		}
	}
	if completed < 30 {
		return Option::None;
	}
	// verify correctness
	for i in 0..completed {
		utils::verify_sorted(&test_vectors[i]);
	}
	// mean and stdev
	let mean = running_sum / completed as f64;
	Option::Some(BenchmarkResult {
		mean,
		stdev: stdev(&results, completed, mean)
	})
}

fn main() {
	let algorithms: Vec<(fn(&mut [i32]), String)> = vec![
		pair!(algos::bubblesort::<i32>),
		pair!(algos::cocktail_shaker::<i32>),
		pair!(algos::selectionsort::<i32>),
		pair!(algos::insertionsort::<i32>),
		pair!(algos::shell::<i32>),
		pair!(algos::mergesort_pre_alloc::<i32>),
		pair!(algos::mergesort_repeated_alloc::<i32>),
		pair!(algos::mergesort_hybrid::<i32>),
		pair!(algos::mergesort_in_place_naive::<i32>),
		pair!(algos::mergesort_in_place::<i32>),
		pair!(algos::heapsort_bottom_up::<i32>),
		pair!(algos::heapsort_top_down::<i32>),
		pair!(sort::weird::<i32>),
		pair!(sort::rustsort::<i32>)
	];
	// run tests
	let mut results = vec![Vec::new(); algorithms.len()];
	let mut header = vec![String::from("")];
	let mut test_size = 10;
	let n_tests = 200;
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= 1_000_000 {
		header.push(utils::commafy(test_size));
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
	// delimited table
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
	// make table
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
				let ci = 2.0 * result.stdev / 1e6;
				let s = format!("{:.5} ± {:.5} ({:.0}%)", v, ci, ci / v * 100.0);
				row.push(Cell::new(&s));
			}
		}
		table.add_row(Row::new(row));
	}
	table.printstd();
	println!("└ Values in ms; 95% confidence interval displayed");
}
