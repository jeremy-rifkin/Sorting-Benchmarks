use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use crate::algos;
use crate::unchecked_tools::SliceUnchecked;

pub fn partition_end<T: Ord + Copy>(slice: &mut [T]) -> usize {
	unsafe {
		let pivot = *slice.get_unchecked(slice.len() - 1);
		let mut i = 0;
		for j in 0..(slice.len() - 1) {
			if *slice.get_unchecked(j) < pivot {
				slice.swap_unchecked(i, j);
				i += 1;
			}
		}
		slice.swap_unchecked(i, slice.len() - 1);
		return i;
	}
}

pub fn quicksort_end<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	let pivot = partition_end(array);
	// safety: 0 <= pivot < array.len()
	// TODO: no performance difference observed on x86, slight diff on arm
	let (l, r) = unsafe { array.split_at_unchecked_mut_excl(pivot) };
	quicksort_end(l);
	quicksort_end(r);
}

fn partition_random<T: Ord + Copy>(slice: &mut [T], rng: &mut SmallRng) -> usize {
	unsafe {
		slice.swap_unchecked(rng.gen_range(0..slice.len()), slice.len() - 1);
	}
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
	// safety: 0 <= pivot < array.len()
	// TODO: no performance difference observed on x86, slight diff on arm
	let pivot = partition_random(array, rng);
	let (l, r) = unsafe { array.split_at_unchecked_mut_excl(pivot) };
	quicksort_random_step(l, rng);
	quicksort_random_step(r, rng);
}

pub fn quicksort_hybrid<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 32 {
		algos::insertionsort(array);
		return;
	}
	// safety: 0 <= pivot < array.len()
	// TODO: no performance difference observed on x86, slight diff on arm
	let pivot = partition_end(array);
	let (l, r) = unsafe { array.split_at_unchecked_mut_excl(pivot) };
	quicksort_hybrid(l);
	quicksort_hybrid(r);
}
