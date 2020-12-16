pub fn shell_sequence<'a, T: Ord>(slice: &mut [T], gap_sequence: impl Iterator<Item = &'a usize>) {
	for gap in gap_sequence {
		if *gap < slice.len() {
			insertion_gap(slice, *gap);
		}
	}
}

pub fn shell_function<T: Ord>(slice: &mut [T], gap_function: impl Fn(usize) -> usize) {
	let mut i = 0;
	while gap_function(i) < slice.len() {
		i += 1;
	}
	for j in (0..i).rev() {
		insertion_gap(slice, gap_function(j));
	}
}

pub fn shell<T: Ord>(slice: &mut [T]) {
	const DEFAULT_SEQUENCE: [usize; 12] = [
		20622, 8855, 3802, 1633,
		701, 301, 132, 57,
		23, 10, 4, 1
	];
	shell_sequence(slice, DEFAULT_SEQUENCE.iter());
}
