use crate::algos::insertionsort;
use crate::algos::merge_single;

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

// wrapper for rust's sorting algorithm
pub fn rustsort<T: Ord>(slice: &mut [T]) {
	slice.sort();
}
