use crate::unchecked_tools::SliceUnchecked;

// node's children are at 2i + 1 and 2i + 2
// parent is at (i - 1) / 2
// sink and swim should be inlined
// I think swim and sink are pretty well optimized...

fn sink<T: Ord>(array: &mut[T], mut i: usize) {
	unsafe {
		loop {
			let l = 2 * i + 1;
			let r = 2 * i + 2;
			let target_i = if r < array.len() && array.get_unchecked(r) > array.get_unchecked(l)
							{ r } else { l };
			if target_i < array.len() && array.get_unchecked(target_i) > array.get_unchecked(i) {
				array.swap_unchecked(i, target_i);
				i = target_i;
			} else {
				break;
			}
		}
	}
}

fn swim<T: Ord>(array: &mut[T], mut i: usize) {
	unsafe {
		loop {
			if i != 0 && array.get_unchecked((i - 1) / 2) < array.get_unchecked(i) {
				array.swap_unchecked(i, (i - 1) / 2);
				i = (i - 1) / 2;
			} else {
				break;
			}
		}
	}
}

fn extract_from_heap<T: Ord>(array: &mut[T]) {
	unsafe {
		// extraction
		for i in (1..array.len()).rev() {
			// take max
			array.swap_unchecked(0, i);
			// sink
			sink(&mut array[..i], 0);
		}
	}
}

pub fn heapsort_top_down<T: Ord>(array: &mut[T]) {
	// build heap
	for i in 0..array.len() {
		swim(array, i);
	}
	// extract sorted array
	extract_from_heap(array);
}

pub fn heapsort_bottom_up<T: Ord>(array: &mut[T]) {
	// build heap
	for i in (0..array.len() / 2).rev() {
		sink(array, i);
	}
	// extract sorted array
	extract_from_heap(array);
}
