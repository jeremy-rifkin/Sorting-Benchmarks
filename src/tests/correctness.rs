#[allow(dead_code)]

use rand::rngs::SmallRng;
use rand::{RngCore, SeedableRng};

use crate::algos;
use crate::utils::*;

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

fn test_sorting_algorithm_size(mut algorithm: impl FnMut(&mut [i32]), size: usize) {
	let mut array = create_sorted_array(size);
	algorithm(&mut array);
	verify_sorted(&array);

	let mut array = create_reversed_array(size);
	algorithm(&mut array);
	verify_sorted(&array);

	let mut rng = SmallRng::seed_from_u64(FIXED_SEED);
	for _i in 0..5 {
		let mut array = create_random_array(size, &mut rng);
		algorithm(&mut array);
		verify_sorted(&array);
	}
}

fn test_sorting_algorithm(algorithm: impl FnMut(&mut [i32])) {
	test_sorting_algorithm_size(algorithm, TEST_ARRAY_SIZE);
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
fn test_selectionsort() {
	test_sorting_algorithm(algos::selectionsort::selectionsort);
}

#[test]
fn test_insertionsort() {
	test_sorting_algorithm(algos::insertionsort::insertionsort);
}

#[test]
fn test_insertionsort_boundary_checked() {
	test_sorting_algorithm(algos::insertionsort::insertionsort_boundary_checked);
}

#[test]
fn test_insertionsort_c() {
	test_sorting_algorithm(algos::insertionsort_c);
}

#[test]
fn test_shellsort_knuth() {
	test_sorting_algorithm(algos::shellsort::shellsort_knuth);
}

#[test]
fn test_shellsort_sedgewick82() {
	test_sorting_algorithm(algos::shellsort::shellsort_sedgewick82);
}

#[test]
fn test_shellsort_sedgewick86() {
	test_sorting_algorithm(algos::shellsort::shellsort_sedgewick86);
}

#[test]
fn test_shellsort_gonnet_baeza() {
	test_sorting_algorithm(algos::shellsort::shellsort_gonnet_baeza);
}

#[test]
fn test_shellsort_tokuda() {
	test_sorting_algorithm(algos::shellsort::shellsort_tokuda);
}

#[test]
fn test_shellsort_ciura() {
	test_sorting_algorithm(algos::shellsort::shellsort_ciura);
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
fn test_mergesort_adaptive() {
	test_sorting_algorithm(exp_algos::mergesort_adaptive);
}

#[test]
fn test_mergesort_double_hybrid() {
	test_sorting_algorithm(exp_algos::mergesort_double_hybrid);
	// coverage for an edge case:
	test_sorting_algorithm_size(exp_algos::mergesort_double_hybrid, 10_000);
}

#[test]
fn test_quicksort_end() {
	test_sorting_algorithm(algos::quicksort_end);
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
fn test_radixsort() {
	test_sorting_algorithm(algos::radixsort);
}

#[test]
fn test_cpp_std_sort() {
	test_sorting_algorithm(algos::cpp_std_sort);
}

//#[test]
//fn test_cpp_std_stable_sort() {
//	test_sorting_algorithm(algos::cpp_std_stable_sort);
//}

// ---------------------  experimental algos  ---------------------
use crate::exp_algos;
#[test]
fn test_weird() {
	test_sorting_algorithm(exp_algos::weird);
}

#[test]
fn test_alternative() {
	test_sorting_algorithm(exp_algos::shellsort_alternative_ciura);
}
