use std::time::Instant;
use rand::prelude::*;
use rand::Rng;
mod sort;

fn bench(mut sort: impl FnMut(&mut [i32]), size: usize, count: usize, seed: u64) -> u128 {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut test_vectors: Vec<Vec<i32>> = vec![vec![0; size]; count];
    for i in 0..count {
        test_vectors.push(Vec::new());
        for _ in 0..size {
            test_vectors[i].push(rng.gen());
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
        sum += results[i];
    }
    sum / count as u128
}

fn main() {
    let test_size = 20;
    let test_count = 1000;
    let seed: u64 = thread_rng().gen();
    println!("Seed: {}", seed);

    println!(
        "{} takes {} seconds on average to sort an array of size {}. {} tests run.",
        "Rust's sorting algorithm",
        bench(|slice| slice.sort(), test_size, test_count, seed) as f64 / 1000000000.0,
        test_size,
        test_count
    );

    println!(
        "{} takes {} seconds on average to sort an array of size {}. {} tests run.",
        "sort::insertion",
        bench(sort::insertion, test_size, test_count, seed) as f64 / 1000000000.0,
        test_size,
        test_count
    );
}
