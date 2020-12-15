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
    const GOAL: usize = 16384;
    let seed: u64 = thread_rng().gen();
    println!("Seed: {}", seed);

    let mut table = table!(["", "Rust's sort", "sort::insertion", "sort::shell", "sort::weird"]);
    let mut test_size = 4;
    while test_size <= GOAL {
        table.add_row(row![test_size,
            bench(|slice| slice.sort(), test_size, GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::insertion, test_size, GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::shell, test_size, GOAL / test_size, seed) as f64 / 1000000.0,
            bench(sort::weird, test_size, GOAL / test_size, seed) as f64 / 1000000.0]);
        test_size *= 4;
    }
    table.printstd();
}
