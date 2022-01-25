use crate::algos;
use crate::utils::compare_and_swap;
use crate::unchecked_tools::SliceUnchecked;

// node's children are at 2i + 1 and 2i + 2
// parent is at (i - 1) / 2
// sink and swim should be inlined
// I think swim and sink are pretty well optimized...

fn sink<T: Ord + Copy>(array: &mut[T], mut i: usize) {
	unsafe {
		let value = *array.get_unchecked(i);
		while 2 * i + 1 < array.len() {
			let l = 2 * i + 1;
			let r = 2 * i + 2;
			let lv = *array.get_unchecked(l);
			let (target_i, target_v) = if r < array.len() && *array.get_unchecked(r) > lv
			                           { (r, *array.get_unchecked(r)) } else { (l, lv) };
			if target_v > value {
				*array.get_unchecked_mut(i) = target_v;
				i = target_i;
			} else {
				break;
			}
		}
		*array.get_unchecked_mut(i) = value;
	}
}

fn swim<T: Ord + Copy>(array: &mut[T], mut i: usize) {
	unsafe {
		let value = *array.get_unchecked(i);
		while i != 0 {
			let parent = *array.get_unchecked((i - 1) / 2);
			if parent < value {
				*array.get_unchecked_mut(i) = *array.get_unchecked((i - 1) / 2);
				i = (i - 1) / 2;
			} else {
				break;
			}
		}
		*array.get_unchecked_mut(i) = value;
	}
}

fn sort_down<T: Ord + Copy>(array: &mut[T]) {
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

pub fn heapsort_top_down<T: Ord + Copy>(array: &mut[T]) {
	// build heap
	for i in 0..array.len() {
		swim(array, i);
	}
	// extract sorted array
	sort_down(array);
}

pub fn heapsort_bottom_up<T: Ord + Copy>(array: &mut[T]) {
	// build heap
	for i in (0..array.len() / 2).rev() {
		sink(array, i);
	}
	// extract sorted array
	sort_down(array);
}

fn sink_guaranteed_right_child<T: Ord + Copy>(array: &mut[T], mut i: usize) {
	unsafe {
		let value = *array.get_unchecked(i);
		while i <= (array.len() - 3) / 2 {
			let l = 2 * i + 1;
			let r = 2 * i + 2;
			let rv = *array.get_unchecked(r);
			let lv = *array.get_unchecked(l);
			let (target_i, target_v) = if rv > lv { (r, rv) } else { (l, lv) };
			assert!(target_i < array.len());
			if target_v > value {
				*array.get_unchecked_mut(i) = target_v;
				i = target_i;
			} else {
				break;
			}
		}
		*array.get_unchecked_mut(i) = value;
	}
}

fn sort_down_guaranteed_right_child<T: Ord + Copy>(array: &mut[T]) {
	unsafe {
		// extraction
		for i in (3..array.len()).rev() {
			// take max
			array.swap_unchecked(0, i);
			// sink
			sink_guaranteed_right_child(&mut array[..(if i % 2 == 0 { i - 1 } else { i })], 0);
		}
		// sorting network for bottom 3 items
		match array {
			[a, b, c, ..] => {
				// because we know [a, b, c] are a max-heap we can sort with just one comparison
				std::mem::swap(a, c);
				compare_and_swap(a, b);
			}
			[a, b] => {
				std::mem::swap(a, b);
			}
			_ => {}
		}
		//let max = std::cmp::min(array.len(), 3);
		//algos::insertionsort(&mut array[..max]);
	}
}

pub fn heapsort_bottom_up_optimized<T: Ord + Copy>(array: &mut[T]) {
	// build heap
	for i in (0..=(array.len() - 3) / 2).rev() {
		sink_guaranteed_right_child(array, i);
	}
	if array.len() % 2 == 0 {
		swim(array, array.len() - 1);
	}
	// extract sorted array
	sort_down_guaranteed_right_child(array);
}
