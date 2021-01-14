pub fn insertionsort<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		for mut i in 1..array.len() {
			let v = *array.get_unchecked(i);
			while i > 0 && *array.get_unchecked(i - 1) > v {
				*array.get_unchecked_mut(i) = *array.get_unchecked(i - 1);
				i -= 1;
			}
			*array.get_unchecked_mut(i) = v;
		}
	}
}

pub fn insertionsort_boundary_checked<T: Ord + Copy>(array: &mut [T]) {
	for mut i in 1..array.len() {
		let v = array[i];
		while i > 0 && array[i - 1] > v {
			array[i] = array[i - 1];
			i -= 1;
		}
		array[i] = v;
	}
}
