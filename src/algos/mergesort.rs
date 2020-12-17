use crate::algos::insertion;

pub fn merge_single<T: Ord + Copy>(slice: &mut [T], middle: usize) {
	let mut i = 0;
	let mut j = middle;
	let mut k = 0;
	let mut merged: Vec<T> = vec![slice[0]; slice.len()];
	while i < middle && j < slice.len() {
		merged[k] = if slice[i] < slice[j] {i += 1; slice[i - 1]} else {j += 1; slice[j - 1]};
		k += 1;
	}
	while i < middle {
		merged[k] = slice[i];
		i += 1;
		k += 1;
	}
	while j < slice.len() {
		merged[k] = slice[j];
		j += 1;
		k += 1;
	}
	slice.copy_from_slice(&merged);
}

pub fn merge<T: Ord + Copy>(slice: &mut [T]) {
	if slice.len() <= 32 {
		insertion(slice);
		return;
	}
	let slice_len = slice.len();
	let middle = slice_len / 2;
	merge(&mut slice[0..middle]);
	merge(&mut slice[middle..slice_len]);
	merge_single(slice, middle);
}
