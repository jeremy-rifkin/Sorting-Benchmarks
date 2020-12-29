use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use crate::algos;

fn partition_end<T: Ord + Copy>(slice: &mut [T]) -> usize {
	let pivot = slice[slice.len() - 1];
	let mut i = 0;
	for j in 0..(slice.len() - 1) {
		if slice[j] < pivot {
			slice.swap(i, j);
			i += 1;
		}
	}
	slice.swap(i, slice.len() - 1);
	return i;
}

pub fn quicksort_end<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	let pivot = partition_end(array);
	quicksort_end(&mut array[..pivot]);
	quicksort_end(&mut array[(pivot + 1)..]);
}

fn partition_random<T: Ord + Copy>(slice: &mut [T], rng: &mut SmallRng) -> usize {
	slice.swap(rng.gen_range(0..slice.len()), slice.len() - 1);
	partition_end(slice)
}

pub fn quicksort_random<T: Ord + Copy>(array: &mut [T]) {
	let mut rng = SmallRng::from_entropy();
	quicksort_random_step(array, &mut rng);
}

fn quicksort_random_step<T: Ord + Copy>(array: &mut [T], rng: &mut SmallRng) {
	if array.len() <= 1 {
		return;
	}
	let pivot = partition_random(array, rng);
	quicksort_random_step(&mut array[..pivot], rng);
	quicksort_random_step(&mut array[(pivot + 1)..], rng);
}

pub fn quicksort_hybrid<T: Ord + Copy>(array: &mut [T]) {
	let mut rng = SmallRng::from_entropy();
	quicksort_hybrid_step(array, &mut rng);
}

fn quicksort_hybrid_step<T: Ord + Copy>(array: &mut [T], rng: &mut SmallRng) {
	if array.len() <= 32 {
		algos::insertionsort(array);
		return;
	}
	let pivot = partition_random(array, rng);
	quicksort_hybrid_step(&mut array[..pivot], rng);
	quicksort_hybrid_step(&mut array[(pivot + 1)..], rng);
}

