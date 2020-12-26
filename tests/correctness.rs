use rand::{RngCore, SeedableRng};
use rand::rngs::SmallRng;

use sorting_benchmarks::*;

const TEST_ARRAY_SIZE: usize = 1000;
const FIXED_SEED: u64 = 0xB000000; // spooky

fn create_sorted_array() -> [i32; TEST_ARRAY_SIZE] {
	let mut array: [i32; TEST_ARRAY_SIZE] = [0; TEST_ARRAY_SIZE];
	for i in 0..TEST_ARRAY_SIZE {
		array[i] = i as i32;
	}
	return array;
}

fn create_reversed_array() -> [i32; TEST_ARRAY_SIZE] {
	let mut array: [i32; TEST_ARRAY_SIZE] = [0; TEST_ARRAY_SIZE];
	for i in (0..TEST_ARRAY_SIZE).rev() {
		array[i] = i as i32;
	}
	return array;
}

fn create_random_array(rng: &mut SmallRng) -> [i32; TEST_ARRAY_SIZE] {
	let mut array: [i32; TEST_ARRAY_SIZE] = [0; TEST_ARRAY_SIZE];
	for i in (0..TEST_ARRAY_SIZE).rev() {
		array[i] = rng.next_u32() as i32;
	}
	return array;
}

fn verify_sorted(array: &[i32]) {
	assert!(array.windows(2).all(|slice| slice[0] <= slice[1]));
}

fn test_sorting_algorithm(mut algorithm: impl FnMut(&mut [i32])) {
	let mut array = create_sorted_array();
	algorithm(&mut array);
	verify_sorted(&array);

	let mut array = create_reversed_array();
	algorithm(&mut array);
	verify_sorted(&array);

	let mut rng = SmallRng::seed_from_u64(FIXED_SEED);
	for _i in 0..5 {
		let mut array = create_random_array(&mut rng);
		algorithm(&mut array);
		verify_sorted(&array);
	}
}

#[test]
fn test_bubblesort() {
	test_sorting_algorithm(algos::bubblesort::bubblesort);
}

#[test]
fn test_cocktail_shaker() {
	test_sorting_algorithm(algos::cocktail_shaker::cocktail_shaker);
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
fn test_insertionsort() {
	test_sorting_algorithm(algos::insertionsort::insertionsort);
}

#[test]
fn test_mergesort() {
	test_sorting_algorithm(algos::mergesort::merge);
}

#[test]
fn test_selectionsort() {
	test_sorting_algorithm(algos::selectionsort::selection);
}

#[test]
fn test_shellsort() {
	test_sorting_algorithm(algos::shellsort::shell);
}
