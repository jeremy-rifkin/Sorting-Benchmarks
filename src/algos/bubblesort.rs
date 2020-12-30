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

pub fn bubblesort_unsafe<T: Ord + Copy>(array: &mut[T]) {
	unsafe {
		let mut swapped = true;
		let ptr = array.as_mut_ptr();
		while swapped {
			swapped = false;
			for i in 1..array.len() {
				if *ptr.add(i - 1) > *ptr.add(i) {
					let tmp = *ptr.add(i - 1);
					*ptr.add(i - 1) = *ptr.add(i);
					*ptr.add(i) = tmp;
					swapped = true;
				}
			}
		}
	}
}
