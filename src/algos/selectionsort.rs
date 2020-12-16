pub fn selection<T: Ord + Copy>(slice: &mut [T]) {
	for i in 0..(slice.len() - 1) {
		let mut min = slice[i];
		let mut min_index = i;
		for j in (i + 1)..slice.len() {
			if slice[j] < min {
				min = slice[j];
				min_index = j;
			}
		}
		slice.swap(i, min_index);
	}
}