use crate::swap_unsafe::SwapUnsafe;

pub fn insertionsort<T: Ord>(array: &mut [T]) {
	unsafe {
		for mut i in 1..array.len() {
			while i > 0 && array.get_unchecked(i - 1) > array.get_unchecked(i) {
				array.swap_unchecked(i, i - 1);
				i -= 1;
			}
		}
	}
}

pub fn insertionsort_boundary_checked<T: Ord>(array: &mut [T]) {
	for mut i in 1..array.len() {
		while i > 0 && array[i - 1] > array[i] {
			array.swap(i, i - 1);
			i -= 1;
		}
	}
}
