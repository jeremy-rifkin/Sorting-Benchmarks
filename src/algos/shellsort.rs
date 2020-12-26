fn insertion_gap<T: Ord>(array: &mut [T], gap: usize) {
	for mut i in gap..array.len() {
		while i >= gap && array[i - gap] > array[i] {
			array.swap(i, i - gap);
			i -= gap;
		}
	}
}

pub fn shell_sequence<T: Ord>(slice: &mut [T], gap_sequence: &[usize]) {
	for gap in gap_sequence {
		if *gap < slice.len() {
			insertion_gap(slice, *gap);
		}
	}
}

#[allow(dead_code)]
pub fn shell_function<T: Ord>(slice: &mut [T], gap_function: impl Fn(usize) -> usize) {
	// shell functions are typically ""almost" geometric sequences" so this step should take
	// logarithmic time and be fairly inconsequential
	let mut i = 0;
	while gap_function(i) < slice.len() {
		i += 1;
	}
	for j in (0..i).rev() {
		insertion_gap(slice, gap_function(j));
	}
}

// TODO: compare multiple functions / sequences
pub fn shell<T: Ord>(slice: &mut [T]) {
	const DEFAULT_SEQUENCE: [usize; 12] = [
		20622, 8855, 3802, 1633,
		701, 301, 132, 57,
		23, 10, 4, 1
	];
	shell_sequence(slice, &DEFAULT_SEQUENCE[..]);
}
