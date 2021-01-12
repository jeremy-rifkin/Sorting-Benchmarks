use crate::swap_unsafe::SwapUnsafe;

pub fn bubblesort<T: Ord>(array: &mut[T]) {
	unsafe {
		let mut swapped = true;
		while swapped {
			swapped = false;
			for i in 1..array.len() {
				if array.get_unchecked(i - 1) > array.get_unchecked(i) {
					array.swap_unchecked(i - 1, i);
					swapped = true;
				}
			}
		}
	}
}
