pub fn insertionsort<T: Ord>(array: &mut [T]) {
	for mut i in 1..array.len() {
		while i > 0 && array[i - 1] > array[i] {
			array.swap(i, i - 1);
			i -= 1;
		}
	}
}
