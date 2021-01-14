use crate::unchecked_tools::SliceUnchecked;

// node's children are at 2i + 1 and 2i + 2
// parent is at (i - 1) / 2
// TODO: some of this is not optimally written
// presumably sink calls will be inlined
fn sink<T: Ord>(array: &mut[T], i: usize, heap_size: usize) {
	unsafe {
		let mut i = i;
		while (2 * i + 1 < heap_size
			&& array.get_unchecked(i) < array.get_unchecked(2 * i + 1)) ||
			(2 * i + 2 < heap_size
			&& array.get_unchecked(i) < array.get_unchecked(2 * i + 2)) {
			// figure out how to sink
			if 2 * i + 2 < heap_size {
				// sink with larger child
				if array.get_unchecked(2 * i + 1) > array.get_unchecked(2 * i + 2) {
					array.swap_unchecked(i, 2 * i + 1);
					i = 2 * i + 1;
				} else {
					array.swap_unchecked(i, 2 * i + 2);
					i = 2 * i + 2;
				}
			} else {
				// sink with the left child
				array.swap_unchecked(i, 2 * i + 1);
				i = 2 * i + 1;
			}
		}
	}
}

pub fn heapsort_top_down<T: Ord>(array: &mut[T]) {
	unsafe {
		// build heap
		for mut i in 0..array.len() {
			// swim
			while i > 0 && array.get_unchecked((i - 1) / 2) < array.get_unchecked(i) {
				array.swap_unchecked(i, (i - 1) / 2);
				i = (i - 1) / 2;
			}
		}
		// extraction
		for i in (0..array.len()).rev() {
			// take max
			array.swap_unchecked(0, i);
			// sink
			sink(array, 0, i);
		}
	}
}

pub fn heapsort_bottom_up<T: Ord>(array: &mut[T]) {
	unsafe {
		// build heap
		for i in (0..(array.len() / 2)).rev() {
			sink(array, i, array.len());
		}
		// extraction
		for i in (0..array.len()).rev() {
			// take max
			array.swap_unchecked(0, i);
			// sink
			sink(array, 0, i);
		}
	}
}
