pub fn bubblesort<T: Ord>(array: &mut[T]) {
	let mut swapped = true;
	while swapped {
		swapped = false;
		for i in 1..array.len() {
			if array[i - 1] > array[i] {
				array.swap(i - 1, i);
				swapped = true;
			}
		}
	}
}
