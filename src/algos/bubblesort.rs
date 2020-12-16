pub fn bubble<T: Ord>(slice: &mut [T]) {
	let mut swapped = true;
	while swapped {
		swapped = false;
		for i in 1..slice.len() {
			if slice[i] < slice[i - 1] {
				slice.swap(i, i - 1);
				swapped = true;
			}
		}
	}
}
