use crate::algos;
use crate::unchecked_tools::SliceUnchecked;

const fn num_bits<T>() -> usize { std::mem::size_of::<T>() * 8 }

pub fn introsort<T: Ord + Copy>(array: &mut [T]) {
	// max recursion = 2 * floor(log_2(array.len()))
	introsort_step(array, (num_bits::<usize>() - array.len().leading_zeros() as usize - 1) * 2);
}

fn introsort_step<T: Ord + Copy>(mut array: &mut [T], mut r_height: usize) {
	// loop creates something along the lines of a tail-call recursion
	// TODO: no performance difference observed on x86 :/
	loop {
		if array.len() <= algos::INSERTIONSORT_THRESHOLD {
			algos::insertionsort(array);
			return;
		} else if r_height == 0 {
			algos::heapsort::heapsort_bottom_up_optimized(array);
			return;
		} else {
			let pivot = algos::quicksort::partition_end(array);
			// safety: 0 <= pivot < array.len()
			let (l, r) = unsafe { array.split_at_unchecked_mut_excl(pivot) };
			if l.len() < r.len() {
				introsort_step(l, r_height - 1);
				array = r;
			} else {
				introsort_step(r, r_height - 1);
				array = l;
			}
			r_height -= 1;
		}
	}
}
