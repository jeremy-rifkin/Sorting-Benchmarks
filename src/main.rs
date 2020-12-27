use std::time::Instant;
use rand::prelude::*;
use rand::Rng;
use regex::Regex;
use prettytable::*;

mod algos;
mod sort;

macro_rules! pair {
	($f:expr) => {
		($f, &Regex::new("::<.+>$").unwrap().replace(stringify!($f), ""))
	};
}

fn bench(mut sort: impl FnMut(&mut [i32]), size: usize, count: usize, seed: u64) -> f64 {
	let mut rng = StdRng::seed_from_u64(seed);
	let mut test_vectors: Vec<Vec<i32>> = vec![vec![0; size]; count];
	for i in 0..count {
		for j in 0..size {
			test_vectors[i][j] = rng.gen();
		}
	}

	let mut results: Vec<u128> = vec![0; count];
	for i in 0..count {
		let now = Instant::now();
		sort(&mut test_vectors[i]);
		results[i] = now.elapsed().as_nanos();
	}

	let mut sum = 0;
	for i in 0..count {
		assert!(test_vectors[i].windows(2).all(|slice| slice[0] <= slice[1]));
		sum += results[i];
	}
	sum as f64 / count as f64
}

fn main() {
	let seed: u64 = 0x2BAD;
	let algorithms: [(for<'r> fn(&'r mut [i32]), &str); 5] = [
		pair!(algos::mergesort_pre_alloc::<i32>),
		pair!(algos::mergesort_repeated_alloc::<i32>),
		pair!(algos::mergesort_hybrid::<i32>),
		pair!(algos::mergesort_in_place_naive::<i32>),
		pair!(algos::mergesort_in_place::<i32>)
	];
	let mut table = Table::new();
	let mut header = vec![""];
	for a in algorithms.iter() {
		header.push(a.1);
	}
	header.push("rust::sort");
	table.add_row(Row::new(header.iter().map(|x| Cell::new(x)).collect()));
	let mut test_size = 10;
	let mut algorithm_enable_flags = vec![true; algorithms.len()];
	while test_size <= 1_000_000 {
		let mut row = vec![Cell::new(&test_size.to_string())];
		for (i, a) in algorithms.iter().enumerate() {
			println!("{} {}", test_size, a.1);
			if algorithm_enable_flags[i] {
				let b = bench(a.0, test_size, 10, seed) / 1000000.0;
				row.push(Cell::new(&(b.to_string())));
				// disable algorithm if it was exceedingly slow for this test
				if b >= 1000.0 {
					algorithm_enable_flags[i] = false;
				}
			} else {
				row.push(Cell::new("-"));
			}
		}
		row.push(Cell::new(&(bench(|slice| slice.sort(), test_size, 10, seed) / 1000000.0).to_string()));
		table.add_row(Row::new(row));
		test_size *= 10;
	}
	table.printstd();
}
