pub fn insertion_gap<T: Ord>(slice: &mut [T], gap: usize) {
	if gap == 0 {
		panic!("Invalid argument! gap has to be at least 1.");
	}

	for i in 1..(gap + 1) {
		for j in (i..slice.len()).step_by(gap) {
			let mut k = j;
			while k >= gap && slice[k] < slice[k - gap] {
				slice.swap(k, k - gap);
				k -= gap;
			}
		}
	}
}

pub fn insertion<T: Ord>(slice: &mut [T]) {
	insertion_gap(slice, 1);
}