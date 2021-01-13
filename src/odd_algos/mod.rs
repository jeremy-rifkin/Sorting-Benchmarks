//
// This module contains a number of odd/non-standard/experimental sorting algorithms implemented
// while working on the project
// mostly out out of curiosity
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

pub fn selectionsort_cocktail<T: Ord>(array: &mut [T]) {
	unsafe {
		let mut finding_max = true;
		let mut edge_space = 0;
		while edge_space * 2 + (!finding_max as usize) < array.len() {
			if finding_max {
				let mut max = array.get_unchecked(edge_space);
				let mut max_index = edge_space;
				for i in (edge_space + 1)..(array.len() - edge_space) {
					if array.get_unchecked(i) > max {
						max = array.get_unchecked(i);
						max_index = i;
					}
				}
				array.swap_unchecked(array.len() - 1 - edge_space, max_index);
				finding_max = false;
			} else {
				let mut min = array.get_unchecked(array.len() - 1 - edge_space);
				let mut min_index = array.len() - 1 - edge_space;
				for i in edge_space..(array.len() - 1 - edge_space) {
					if array.get_unchecked(i) < min {
						min = array.get_unchecked(i);
						min_index = i;
					}
				}
				array.swap_unchecked(edge_space, min_index);
				finding_max = true;
				edge_space += 1;
			}
		}
	}
}

pub fn selectionsort_minmax<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		for i in 0..(array.len() / 2) {
			let mut min = *array.get_unchecked(i);
			let mut min_index = i;
			let mut max = *array.get_unchecked(i);
			let mut max_index = i;
			for j in (i + 1)..(array.len() - i) {
				if *array.get_unchecked(j) < min {
					min = *array.get_unchecked(j);
					min_index = j;
				} else if *array.get_unchecked(j) > max {
					max = *array.get_unchecked(j);
					max_index = j;
				}
			}
			if i == max_index {
				max_index = min_index;
			}
			array.swap_unchecked(i, min_index);
			array.swap_unchecked(array.len() - 1 - i, max_index);
		}
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

// Merge sort that keeps sorted subarrays and flips reversed subarrays.
use std::collections::VecDeque;

fn reverse<T>(slice: &mut [T]) {
	unsafe {
		for i in 0..(slice.len() / 2) {
			slice.swap_unchecked(i, slice.len() - 1 - i);
		}
	}
}

fn merge_to_buffer<T: Ord + Copy>(array: &mut [T], start: usize, split: usize, end: usize, buffer: &mut [T]) {
	unsafe {
		let mut i = start;
		let mut j = split;
		let mut k = start;
		while i < split && j < end {
			if array.get_unchecked(i) < array.get_unchecked(j) {
				*buffer.get_unchecked_mut(k) = *array.get_unchecked(i);
				i += 1;
			} else {
				*buffer.get_unchecked_mut(k) = *array.get_unchecked(j);
				j += 1;
			}
			k += 1;
		}

		while i < split {
			*buffer.get_unchecked_mut(k) = *array.get_unchecked(i);
			i += 1;
			k += 1;
		}

		while j < end {
			*buffer.get_unchecked_mut(k) = *array.get_unchecked(j);
			j += 1;
			k += 1;
		}
	}
}

pub fn mergesort_adaptive<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		let mut start = 0;
		let mut end = 1;
		let mut sorted = true;
		let mut merge_queue: VecDeque<usize> = VecDeque::new();
		while end < array.len() {
			if end - start == 1 {
				sorted = array.get_unchecked(start) < array.get_unchecked(end);
			} else if (array.get_unchecked(end - 1) <= array.get_unchecked(end)) != sorted { // Keeps the algorithm stable.
				if !sorted {
					reverse(&mut array[start..end]);
				}
				merge_queue.push_back(end);
				start = end;
			}
			end += 1;
		}
		if !sorted {
			reverse(&mut array[start..end]);
		}
		merge_queue.push_back(end);

		start = 0;
		let mut in_array = true;
		let mut new_merge_queue: VecDeque<usize> = VecDeque::new();
		let mut buffer: Vec<T> = vec![*array.get_unchecked(0); array.len()];
		while merge_queue.len() > 1 {
			let split = merge_queue.pop_front().unwrap();
			let end = merge_queue.pop_front().unwrap();
			if in_array {
				merge_to_buffer(array, start, split, end, &mut buffer);
			} else {
				merge_to_buffer(&mut buffer, start, split, end, array);
			}
			new_merge_queue.push_back(end);
			start = end;

			if merge_queue.len() <= 1 {
				if merge_queue.len() == 1 {
					// TODO consider merging this here instead of postponing this.
					let end = merge_queue.pop_front().unwrap();
					if in_array {
						buffer[start..end].clone_from_slice(&array[start..end]);
					} else {
						array[start..end].clone_from_slice(&buffer[start..end]);
					}
					new_merge_queue.push_back(end);
				}

				merge_queue = new_merge_queue;
				start = 0;
				in_array = !in_array;
				new_merge_queue = VecDeque::new();
			}
		}

		if !in_array {
			array.clone_from_slice(&buffer);
		}
	}
}

pub fn mergesort_double_hybrid<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		let mut merge_queue: VecDeque<usize> = VecDeque::new();
		let mut i = 0;
		while i + 32 < array.len() {
			insertionsort(&mut array[i..(i + 32)]);
			merge_queue.push_back(i + 32);
			i += 32;
		}
		insertionsort(&mut array[i..]);
		merge_queue.push_back(array.len());

		let mut start = 0;
		let mut in_array = true;
		let mut new_merge_queue: VecDeque<usize> = VecDeque::new();
		let mut buffer: Vec<T> = vec![*array.get_unchecked(0); array.len()];
		while merge_queue.len() > 1 {
			let split = merge_queue.pop_front().unwrap();
			let end = merge_queue.pop_front().unwrap();
			if in_array {
				merge_to_buffer(array, start, split, end, &mut buffer);
			} else {
				merge_to_buffer(&mut buffer, start, split, end, array);
			}
			new_merge_queue.push_back(end);
			start = end;

			if merge_queue.len() <= 1 {
				if merge_queue.len() == 1 {
					// TODO consider merging this here instead of postponing this.
					let end = merge_queue.pop_front().unwrap();
					if in_array {
						buffer[start..end].clone_from_slice(&array[start..end]);
					} else {
						array[start..end].clone_from_slice(&buffer[start..end]);
					}
					new_merge_queue.push_back(end);
				}

				merge_queue = new_merge_queue;
				start = 0;
				in_array = !in_array;
				new_merge_queue = VecDeque::new();
			}
		}

		if !in_array {
			array.clone_from_slice(&buffer);
		}
	}
}
