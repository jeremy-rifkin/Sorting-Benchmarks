use crate::algos::heapsort;
use crate::algos::insertionsort;
use crate::algos::quicksort;

pub fn introsort<T: Ord + Copy>(array: &mut [T]) {
	// max_depth = floor(log_2(array.len())) * 2
	// floor(log_2(array.len())) = number of times we can bitshift right and still be >= 1
	// we can build-in the *2 by just checking >= 0
	// so what we do here to calculate the log is binary shift by 1 each recursion
	// TODO: is this optimal? or would it be better to just use floats to calculate the log upfront...
	introsort_step(array, array.len());
}

pub fn introsort_step<T: Ord + Copy>(array: &mut [T], max_depth_intermediate: usize) {
	if array.len() <= 32 {
		insertionsort(array);
	} else if max_depth_intermediate == 0 {
		heapsort::heapsort_top_down(array);
	} else {
		let pivot = quicksort::partition_end(array);
		introsort_step(&mut array[..pivot], max_depth_intermediate >> 1);
		introsort_step(&mut array[(pivot + 1)..], max_depth_intermediate >> 1);
	}
}
