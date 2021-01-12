use crate::swap_unsafe::SwapUnsafe;

pub fn selectionsort<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		for i in 0..(array.len() - 1) {
			let mut min = array.get_unchecked(i);
			let mut min_index = i;
			for j in (i + 1)..array.len() {
				if array.get_unchecked(j) < min {
					min = array.get_unchecked(j);
					min_index = j;
				}
			}
			array.swap_unchecked(i, min_index);
		}
	}
}
