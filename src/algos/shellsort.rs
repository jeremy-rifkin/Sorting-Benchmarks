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

pub fn shell_function<T: Ord>(slice: &mut [T], gap_function: fn(u32) -> usize, max: usize) {
	// shell functions are typically ""almost" geometric sequences" so this step should take
	// logarithmic time and be fairly inconsequential
	let mut i = 0;
	while gap_function(i) <= max {
		i += 1;
	}
	for j in (0..=i).rev() {
		insertion_gap(slice, gap_function(j));
	}
}

pub fn shell_function_known<T: Ord>(slice: &mut [T], gap_function: fn(u32) -> usize, k0: u32, kf: u32) {
	for j in (kf..=k0).rev() {
		insertion_gap(slice, gap_function(j));
	}
}

pub fn shell_function_recursive<T: Ord>(slice: &mut [T], gap_function: fn(usize) -> usize, h0: usize, hmin: usize) {
	let mut h = h0;
	while h >= hmin {
		insertion_gap(slice, h);
		let newh = gap_function(h);
		if newh == h {
			break;
		}
		h = newh;
	}
}

#[allow(dead_code)]
pub fn shellsort_knuth<T: Ord>(slice: &mut [T]) {
	// (3^k - 1) / 2 not exceeding ceil(n / 3)
	shell_function_known(
		slice,
		|k: u32| (3usize.pow(k) - 1) / 2,
		(2.0 * (slice.len() as f64 / 3.0).ceil() + 1.0).log(3.0).floor() as u32,
		1
	);
}

#[allow(dead_code)]
pub fn shellsort_sedgewick82<T: Ord>(slice: &mut [T]) {
	// TODO: test k upper-bound
	shell_function(
		slice,
		|k: u32| if k == 0 { 1 } else { 4usize.pow(k) + 3 * 2usize.pow(k - 1) + 1 },
		slice.len() / 2
	);
}

#[allow(dead_code)]
pub fn shellsort_sedgewick86<T: Ord>(slice: &mut [T]) {
	// TODO: test k upper-bound
	shell_function(
		slice,
		|k: u32| if k % 2 == 0
		{9 * (2usize.pow(k) - 2usize.pow(k / 2)) + 1} else
		{8 * 2usize.pow(k) - 6 * 2usize.pow((k + 1) / 2) + 1},
		slice.len() / 2
	);
}

#[allow(dead_code)]
pub fn shellsort_gonnet_baeza<T: Ord>(slice: &mut [T]) {
	// TODO: test k upper-bound
	shell_function_recursive(
		slice,
		|h: usize| std::cmp::max(5 * h / 11, 1),
		slice.len(),
		1
	);
}

#[allow(dead_code)]
pub fn shellsort_tokuda<T: Ord>(slice: &mut [T]) {
	// TODO: test k upper-bound
	shell_function(
		slice,
		|k: u32| ((9.0 * 2.25f64.powi(k as i32 - 1) - 4.0) / 5.0).ceil() as usize,
		slice.len() / 2
	);
}

pub fn shellsort_ciura<T: Ord>(slice: &mut [T]) {
	const DEFAULT_SEQUENCE: [usize; 12] = [
		20622, 8855, 3802, 1633,
		701, 301, 132, 57,
		23, 10, 4, 1
	];
	shell_sequence(slice, &DEFAULT_SEQUENCE[..]);
}
