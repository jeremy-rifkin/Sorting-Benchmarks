#[macro_use] extern crate prettytable;
use std::time::Instant;
use rand::prelude::*;
use rand::Rng;
mod sort;

fn bench(mut sort: impl FnMut(&mut [i32]), size: usize, count: usize, seed: u64) -> u128 {
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
    sum / count as u128
}

fn main() {
    const SLOW_GOAL: usize = 16384;
    const FAST_GOAL: usize = SLOW_GOAL * 4;
    const MIN_TESTS: usize = 4;
    let seed: u64 = thread_rng().gen();
    println!("Seed: {}", seed);

    // Slower sorting algorithms.
    let mut table = table!(["", "sort::insertion", "sort::bubble", "sort::selection"]);
    let mut test_size = 3;
    while test_size <= SLOW_GOAL / MIN_TESTS {
        table.add_row(row![test_size,
            bench(sort::insertion, test_size, SLOW_GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::bubble, test_size, SLOW_GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::selection, test_size, SLOW_GOAL / test_size, seed) as f64 / 1000000.0]);
        test_size = (test_size as f64 * 1.5) as usize;
    }
    table.printstd();

    // Faster sorting algorithms.
    table = table!(["", "Rust's sort", "sort::shell", "sort::weird", "sort::merge"]);
    let mut test_size = 3;
    while test_size <= FAST_GOAL / MIN_TESTS {
        table.add_row(row![test_size,
            bench(|slice| slice.sort(), test_size, FAST_GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::shell, test_size, FAST_GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::weird, test_size, FAST_GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::merge, test_size, FAST_GOAL / test_size, seed) as f64 / 1000000.0]);
        test_size = (test_size as f64 * 1.5) as usize;
    }
    table.printstd();
}
