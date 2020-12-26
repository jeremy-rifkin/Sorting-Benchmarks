// node's children are at 2i + 1 and 2i + 2
// parent is at (i - 1) / 2
// TODO: some of this is not optimally written
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
		let mut j = 0;
		while (2 * j + 1 < i && array[j] < array[2 * j + 1]) || (2 * j + 2 < i && array[j] < array[2 * j + 2]) {
			// figure out how to sink
			if 2 * j + 2 < i {
				// sink with larger child
				if array[2 * j + 1] > array[2 * j + 2] {
					array.swap(j, 2 * j + 1);
					j = 2 * j + 1;
				} else {
					array.swap(j, 2 * j + 2);
					j = 2 * j + 2;
				}
			} else {
				// sink with the left child
				array.swap(j, 2 * j + 1);
				j = 2 * j + 1;
			}
		}
	}
}
pub fn heapsort_bottom_up<T: Ord>(array: &mut[T]) {
	// TODO
}

