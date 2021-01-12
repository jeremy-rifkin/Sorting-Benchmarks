//
// This module contains a number of experimental sorting algorithms we implemented in testing
// just out of curiosity
// some of them are cool, some of them are just memes
// most of these are courtesy of Robochu
//

use crate::algos::insertionsort;
use crate::algos::merge_single;
use crate::swap_unsafe::SwapUnsafe;

// Robochu's sorting algorithm
pub fn weird<T: Ord + Copy>(slice: &mut [T]) {
	// Sort chunks of size sqrt(N) using an O(N^2) algorithm.
	// This has O(N^1.5) time complexity.
	let chunk_size = (slice.len() as f64).sqrt() as usize;
	for chunk in slice.chunks_mut(chunk_size) {
		insertionsort(chunk);
	}
	// Merge these chunks together. Using a naive implementation, this would also be
	// O(N^1.5) time complexity, but I decided to improve it using basic divide-and-conquer.
	let mut step = 1;
	let mut merge_count = 1;
	while merge_count > 0 {
		merge_count = 0;
		for i in ((step * chunk_size)..slice.len()).step_by(step * chunk_size * 2) {
			let slice_len = slice.len();
			if i + step * chunk_size < slice_len {
				merge_single(&mut slice[(i - step * chunk_size)..(i + step * chunk_size)],
					step * chunk_size);
			} else {
				merge_single(&mut slice[(i - step * chunk_size)..slice_len], step * chunk_size);
			}
			merge_count += 1;
		}
		step *= 2;
	}
}

fn insertion_gap_sequence<T: Ord>(array: &mut [T], gap_sequence: &[usize]) {
	unsafe {
		let mut j = 0;
		for mut i in gap_sequence[j]..array.len() {
			while j < gap_sequence.len() {
				while i >= *gap_sequence.get_unchecked(j) && array.get_unchecked(i - gap_sequence.get_unchecked(j)) > array.get_unchecked(i) {
					array.swap_unchecked(i, i - gap_sequence.get_unchecked(j));
					i -= gap_sequence.get_unchecked(j);
				}
				j += 1;
			}
			j = 0;
		}
	}
}

fn shell_alternative_sequence<T: Ord>(slice: &mut [T], gap_sequence: &[usize]) {
	unsafe {
		for i in 0..gap_sequence.len() {
			if *gap_sequence.get_unchecked(i) < slice.len() {
				insertion_gap_sequence(slice, &gap_sequence[i..]);
			}
		}
	}
}

pub fn shellsort_alternative_ciura<T: Ord>(slice: &mut [T]) {
	const DEFAULT_SEQUENCE: [usize; 12] = [
		20622, 8855, 3802, 1633,
		701, 301, 132, 57,
		23, 10, 4, 1
	];
	shell_alternative_sequence(slice, &DEFAULT_SEQUENCE);
}

