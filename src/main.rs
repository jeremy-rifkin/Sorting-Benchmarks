use std::time::Instant;
use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};
use regex::Regex;
use prettytable::*;

mod algos;
mod sort;
mod utils;

const SEED: u64 = 2222;

struct BenchmarkResult {
	mean: f64,
	stdev: f64
}

// computes the sample standard deviation of a Vec<u64>
// mean passed as a parameter to take advantage of pre-computed value
fn stdev(array: &Vec<u64>, mean: f64) -> f64 {
	let mut sum = 0.0;
	for x in array {
		sum += (*x as f64 - mean).powi(2);
	}
	sum /= (array.len() - 1) as f64;
	sum.sqrt()
}

// returns number with comma separators (i.e. 1000000 -> "1,000,000")
fn commafy(mut num: usize) -> String {
	let log = (num as f64).log10() as usize;
	let len = log + log / 3 + 1;
	let mut s = vec![b'x'; len];
	let mut i = 0;
	let mut count = 0;
	while num > 0 {
		if count > 0 && count % 3 == 0 {
			s[len - i - 1] = b',';
			i += 1;
		}
		s[len - i - 1] = b'0' + (num % 10) as u8;
		i += 1;
		num /= 10;
		count += 1;
	}
	return String::from_utf8(s).unwrap();
}

// wrapper for rust's sorting algorithm
fn rustsort<T: Ord>(slice: &mut [T]) {
	slice.sort();
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
fn bench(sort: fn(&mut [i32]), size: usize, n_tests: usize) -> BenchmarkResult {
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
	for i in 0..n_tests {
		destroy_cache();
		let now = Instant::now();
		sort(&mut test_vectors[i]);
		results[i] = now.elapsed().as_nanos() as u64;
	}
	// verify validity
	for i in 0..n_tests {
		assert!(test_vectors[i].windows(2).all(|slice| slice[0] <= slice[1]));
	}
	// mean and stdev
	let mean = results.iter().sum::<u64>() as f64 / n_tests as f64;
	BenchmarkResult {
		mean,
		stdev: stdev(&results, mean)
	}
}

fn main() {
	let algorithms: [(fn(&mut [i32]), &str); 6] = [
		pair!(algos::mergesort_pre_alloc::<i32>),
		pair!(algos::mergesort_repeated_alloc::<i32>),
		pair!(algos::mergesort_hybrid::<i32>),
		pair!(algos::mergesort_in_place_naive::<i32>),
		pair!(algos::mergesort_in_place::<i32>),
		pair!(rustsort::<i32>)
	];
	let mut table = Table::new();
	let mut header = vec![""];
	for a in algorithms.iter() {
		header.push(a.1);
	}
	table.add_row(Row::new(header.iter().map(|x| Cell::new(x)).collect()));
	let mut test_size = 10;
	let n_tests = 30;
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= 1_000_000 {
		let mut row = vec![Cell::new(&commafy(test_size))];
		for (i, a) in algorithms.iter().enumerate() {
			println!("{} {}", test_size, a.1);
			if algorithm_enable_flags[i] {
				let b = bench(a.0, test_size, n_tests);
				// ns / 1e6 -> ms
				// *2 for 95% CI
				let s = format!("{:.5} ± {:.5} ({:.0}%)", b.mean / 1e6, 2.0 * b.stdev / 1e6, (2.0 * b.stdev / 1e6) / (b.mean / 1e6) * 100.0);
				row.push(Cell::new(&s));
				// disable algorithm if it was exceedingly slow for this test
				if b.mean / 1e6 >= 1000.0 {
					algorithm_enable_flags[i] = false;
				}
			} else {
				row.push(Cell::new("-"));
			}
		}
		table.add_row(Row::new(row));
		test_size *= 10;
	}
	table.printstd();
	println!("values in ms; 95% confidence interval displayed");
}
