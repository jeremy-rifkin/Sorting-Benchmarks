// node's children are at 2i + 1 and 2i + 2
// parent is at (i - 1) / 2
// TODO: some of this is not optimally written
// presumably sink calls will be inlined
fn sink<T: Ord>(array: &mut[T], i: usize, heap_size: usize) {
	let mut i = i;
	while (2 * i + 1 < heap_size && array[i] < array[2 * i + 1]) || (2 * i + 2 < heap_size && array[i] < array[2 * i + 2]) {
		// figure out how to sink
		if 2 * i + 2 < heap_size {
			// sink with larger child
			if array[2 * i + 1] > array[2 * i + 2] {
				array.swap(i, 2 * i + 1);
				i = 2 * i + 1;
			} else {
				array.swap(i, 2 * i + 2);
				i = 2 * i + 2;
			}
		} else {
			// sink with the left child
			array.swap(i, 2 * i + 1);
			i = 2 * i + 1;
		}
	}
}

pub fn heapsort_top_down<T: Ord>(array: &mut[T]) {
	// build heap
	for mut i in 0..array.len() {
		// swim
		while i > 0 && array[(i - 1) / 2] < array[i] {
			array.swap(i, (i - 1) / 2);
			i = (i - 1) / 2;
		}
	}
	// extraction
	for i in (0..array.len()).rev() {
		// take max
		array.swap(0, i);
		// sink
		sink(array, 0, i);
	}
}

pub fn heapsort_bottom_up<T: Ord>(array: &mut[T]) {
	// build heap
	for i in (0..(array.len() / 2)).rev() {
		sink(array, i, array.len());
	}
	// extraction
	for i in (0..array.len()).rev() {
		// take max
		array.swap(0, i);
		// sink
		sink(array, 0, i);
	}
}
