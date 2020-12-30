use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use sorting_benchmarks::*;
use utils::*;

const TEST_ARRAY_SIZE: usize = 1000;
const FIXED_SEED: u64 = 0xB000000; // spooky

fn create_sorted_array(size: usize) -> Vec<i32> {
	let mut array = vec![0; size];
	for i in 0..size {
		array[i] = i as i32;
	}
	return array;
}

fn create_reversed_array(size: usize) -> Vec<i32> {
	let mut array = vec![0; size];
	for (i, v) in (0..size).rev().enumerate() {
		array[i] = v as i32;
	}
	return array;
}

fn create_random_array(size: usize, rng: &mut SmallRng) -> Vec<i32> {
	let mut array = vec![0; size];
	for i in 0..size {
		array[i] = rng.next_u32() as i32;
	}
	return array;
}

fn test_sorting_algorithm(mut algorithm: impl FnMut(&mut [i32])) {
	let mut array = create_sorted_array(TEST_ARRAY_SIZE);
	algorithm(&mut array);
	verify_sorted(&array);

	let mut array = create_reversed_array(TEST_ARRAY_SIZE);
	algorithm(&mut array);
	verify_sorted(&array);

	let mut rng = SmallRng::seed_from_u64(FIXED_SEED);
	for _i in 0..5 {
		let mut array = create_random_array(TEST_ARRAY_SIZE, &mut rng);
		algorithm(&mut array);
		verify_sorted(&array);
	}
}

#[test]
fn test_bubblesort() {
	test_sorting_algorithm(algos::bubblesort::bubblesort);
}

#[test]
fn test_bubblesort_unsafe() {
	test_sorting_algorithm(algos::bubblesort::bubblesort_unsafe);
}

#[test]
fn test_cocktail_shaker() {
	test_sorting_algorithm(algos::cocktail_shaker::cocktail_shaker);
}

#[test]
fn test_selectionsort() {
	test_sorting_algorithm(algos::selectionsort::selectionsort);
}

#[test]
fn test_insertionsort() {
	test_sorting_algorithm(algos::insertionsort::insertionsort);
}

#[test]
fn test_insertionsort_unsafe() {
	test_sorting_algorithm(algos::insertionsort::insertionsort_unsafe);
}

#[test]
fn test_shellsort() {
	test_sorting_algorithm(algos::shellsort::shell);
}

#[test]
fn test_heapsort_top_down() {
	test_sorting_algorithm(algos::heapsort::heapsort_top_down);
}

#[test]
fn test_heapsort_bottom_up() {
	test_sorting_algorithm(algos::heapsort::heapsort_bottom_up);
}

#[test]
fn test_mergesort_repeated_alloc() {
	test_sorting_algorithm(algos::mergesort::mergesort_repeated_alloc);
}

#[test]
fn test_mergesort_pre_alloc() {
	test_sorting_algorithm(algos::mergesort::mergesort_pre_alloc);
}

#[test]
fn test_mergesort_hybrid() {
	test_sorting_algorithm(algos::mergesort::mergesort_hybrid);
}

#[test]
fn test_mergesort_in_place_naive() {
	test_sorting_algorithm(algos::mergesort::mergesort_in_place_naive);
}

#[test]
fn test_mergesort_in_place() {
	test_sorting_algorithm(algos::mergesort::mergesort_in_place);
}

#[test]
fn test_quicksort_end() {
	test_sorting_algorithm(algos::quicksort_end);
}

#[test]
fn test_quicksort_end_unsafe() {
	test_sorting_algorithm(algos::quicksort_end_unsafe);
}

#[test]
fn test_quicksort_random() {
	test_sorting_algorithm(algos::quicksort_random);
}

#[test]
fn test_quicksort_hybrid() {
	test_sorting_algorithm(algos::quicksort_hybrid);
}

#[test]
fn test_weird() {
	test_sorting_algorithm(sort::weird);
}
