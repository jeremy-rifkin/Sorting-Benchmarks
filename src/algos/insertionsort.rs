pub fn insertionsort<T: Ord>(array: &mut [T]) {
	for mut i in 1..array.len() {
		while i > 0 && array[i - 1] > array[i] {
			array.swap(i, i - 1);
			i -= 1;
		}
	}
}

pub fn insertionsort_unsafe<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		let ptr = array.as_mut_ptr();
		for mut i in 1..array.len() {
			while i > 0 && *ptr.add(i - 1) > *ptr.add(i) {
				let tmp = *ptr.add(i - 1);
				*ptr.add(i - 1) = *ptr.add(i);
				*ptr.add(i) = tmp;
				i -= 1;
			}
		}
	}
}
