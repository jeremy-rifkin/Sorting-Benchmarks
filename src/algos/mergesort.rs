use crate::algos;

// TODO: get rid of
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

fn merge<T: Ord + Copy>(slice: &mut [T], buffer: &mut Vec<T>) {
	let middle = slice.len() / 2;
	let mut i = 0;
	let mut j = middle;
	let mut k = 0;
	while i < middle && j < slice.len() {
		if slice[i] < slice[j] {
			i += 1;
			buffer[k] = slice[i - 1];
		} else {
			j += 1;
			buffer[k] = slice[j - 1];
		}
		k += 1;
	}
	while i < middle {
		buffer[k] = slice[i];
		i += 1;
		k += 1;
	}
	while j < slice.len() {
		buffer[k] = slice[j];
		j += 1;
		k += 1;
	}
	slice.copy_from_slice(&buffer[..slice.len()]);
}

pub fn mergesort_repeated_alloc<T: Ord + Copy + Default>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	let middle = array.len() / 2;
	mergesort_repeated_alloc(&mut array[..middle]);
	mergesort_repeated_alloc(&mut array[middle..]);
	let mut buffer: Vec<T> = vec![T::default(); array.len()];
	merge(array, &mut buffer);
}

pub fn mergesort_pre_alloc<T: Ord + Copy + Default>(array: &mut [T]) {
	let mut buffer: Vec<T> = vec![T::default(); array.len()];
	mergesort_pre_alloc_r(array, &mut buffer);
}

fn mergesort_pre_alloc_r<T: Ord + Copy + Default>(array: &mut [T], buffer: &mut Vec<T>) {
	if array.len() <= 1 {
		return;
	}
	let middle = array.len() / 2;
	mergesort_pre_alloc_r(&mut array[..middle], buffer);
	mergesort_pre_alloc_r(&mut array[middle..], buffer);
	merge(array, buffer);
}

pub fn mergesort_hybrid<T: Ord + Copy + Default>(array: &mut [T]) {
	let mut buffer: Vec<T> = vec![T::default(); array.len()];
	mergesort_hybrid_r(array, &mut buffer);
}

fn mergesort_hybrid_r<T: Ord + Copy + Default>(array: &mut [T], buffer: &mut Vec<T>) {
	if array.len() <= 32 {
		algos::insertionsort_unsafe(array);
		return;
	}
	let middle = array.len() / 2;
	mergesort_hybrid_r(&mut array[..middle], buffer);
	mergesort_hybrid_r(&mut array[middle..], buffer);
	merge(array, buffer);
}

fn merge_in_place_naive<T: Ord + Copy>(array: &mut [T]) {
	let mut mid = array.len() / 2;
	let mut i = 0;
	let mut j = mid;
	// slice already sorted
	// bounds are safe because this method won't be called for any array.len() < 2
	// TODO: could also check if the sub-arrays are backwards (happens if the array is fully reversed)
	// TODO: is it actually worth it to perform this kind of check? the probability of something
	// paying off is low...
	if array[mid - 1] <= array[mid] {
		return;
	}
	while i < mid && j < array.len() {
		if array[i] <= array[j] {
			i += 1;
		} else {
			let v = array[j];
			let mut k = j;
			while k > i {
				array[k] = array[k - 1];
				k -= 1;
			}
			array[i] = v;
			i += 1;
			mid += 1;
			j += 1;
		}
	}
}

pub fn mergesort_in_place_naive<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	let middle = array.len() / 2;
	mergesort_in_place_naive(&mut array[..middle]);
	mergesort_in_place_naive(&mut array[middle..]);
	merge_in_place_naive(array);
}

// https://stackoverflow.com/questions/2571049/how-to-sort-in-place-using-the-merge-sort-algorithm
// https://github.com/liuxinyu95/AlgoXY/blob/algoxy/sorting/merge-sort/src/mergesort.c
pub fn mergesort_in_place<T: Ord + Copy>(array: &mut [T]) {
	imsort(array, 0, array.len());
}

fn imsort<T: Ord + Copy>(xs: &mut [T], l: usize, u: usize) {
	let mut m: usize;
	let mut n: usize;
	let mut w: usize;
	if u - l > 1 {
		m = l + (u - l) / 2;
		w = l + u - m;
		wsort(xs, l, m, w); /* the last half contains sorted elements */
		while w - l > 2 {
			n = w;
			w = l + (n - l + 1) / 2;
			wsort(xs, w, n, l);  /* the first half of the previous working area contains sorted elements */
			wmerge(xs, l, l + n - w, n, u, w);
		}
		n = w;
		while n > l {
			m = n;
			while m < u && xs[m] < xs[m-1] {
				xs.swap(m, m - 1);
				m += 1;
			}
			n -= 1;
		}
	}
}

fn wsort<T: Ord + Copy>(xs: &mut [T], mut l: usize, u: usize, mut w: usize) {
	let m: usize;
	if u - l > 1 {
		m = l + (u - l) / 2;
		imsort(xs, l, m);
		imsort(xs, m, u);
		wmerge(xs, l, m, m, u, w);
	} else {
		while l < u {
			xs.swap(l, w);
			l += 1;
			w += 1;
		}
	}
}

fn wmerge<T: Ord + Copy>(xs: &mut [T], mut i: usize, m: usize, mut j: usize, n: usize, mut w: usize) {
	while i < m && j < n {
		#[allow(unused_assignments)]
		let mut k: usize = 0;
		if xs[i] < xs[j] {
			k = i;
			i += 1;
		} else {
			k = j;
			j += 1;
		}
		xs.swap(w, k);
		w += 1;
	}
	while i < m {
		xs.swap(w, i);
		w += 1;
		i += 1;
	}
	while j < n {
		xs.swap(w, j);
		w += 1;
		j += 1;
	}
}





// Merge sort that keeps sorted subarrays and flips reversed subarrays.
use std::collections::VecDeque;

fn reverse<T>(slice: &mut [T]) {
	for i in 0..(slice.len() / 2) {
		slice.swap(i, slice.len() - 1 - i);
	}
}

fn merge_to_buffer<T: Ord + Copy>(array: &mut [T], start: usize, split: usize, end: usize, buffer: &mut [T]) {
	let mut i = start;
	let mut j = split;
	let mut k = start;
	while i < split && j < end {
		if array[i] < array[j] {
			buffer[k] = array[i];
			i += 1;
		} else {
			buffer[k] = array[j];
			j += 1;
		}
		k += 1;
	}

	while i < split {
		buffer[k] = array[i];
		i += 1;
		k += 1;
	}

	while j < end {
		buffer[k] = array[j];
		j += 1;
		k += 1;
	}
}

pub fn mergesort_adaptive<T: Ord + Copy>(array: &mut [T]) {
	let mut start = 0;
	let mut end = 1;
	let mut sorted = true;
	let mut merge_queue: VecDeque<usize> = VecDeque::new();
	while end < array.len() {
		if end - start == 1 {
			sorted = array[start] < array[end];
		} else if (array[end - 1] <= array[end]) != sorted { // Keeps the algorithm stable.
			if !sorted {
				reverse(&mut array[start..end]);
			}
			merge_queue.push_back(end);
			start = end;
		}
		end += 1;
	}
	if !sorted {
		reverse(&mut array[start..end]);
	}
	merge_queue.push_back(end);

	start = 0;
	let mut in_array = true;
	let mut new_merge_queue: VecDeque<usize> = VecDeque::new();
	let mut buffer: Vec<T> = vec![array[0]; array.len()];
	while merge_queue.len() > 1 {
		let split = merge_queue.pop_front().unwrap();
		let end = merge_queue.pop_front().unwrap();
		if in_array {
			merge_to_buffer(array, start, split, end, &mut buffer);
		} else {
			merge_to_buffer(&mut buffer, start, split, end, array);
		}
		new_merge_queue.push_back(end);
		start = end;

		if merge_queue.len() <= 1 {
			if merge_queue.len() == 1 {
				// TODO consider merging this here instead of postponing this.
				let end = merge_queue.pop_front().unwrap();
				if in_array {
					buffer[start..end].clone_from_slice(&array[start..end]);
				} else {
					array[start..end].clone_from_slice(&buffer[start..end]);
				}
				new_merge_queue.push_back(end);
			}

			merge_queue = new_merge_queue;
			start = 0;
			in_array = !in_array;
			new_merge_queue = VecDeque::new();
		}
	}

	if !in_array {
		array.clone_from_slice(&buffer);
	}
}

pub fn mergesort_double_hybrid<T: Ord + Copy>(array: &mut [T]) {
	use algos::insertionsort_unsafe;
	let mut merge_queue: VecDeque<usize> = VecDeque::new();
	let mut i = 0;
	while i + 32 < array.len() {
		insertionsort_unsafe(&mut array[i..(i + 32)]);
		merge_queue.push_back(i + 32);
		i += 32;
	}
	insertionsort_unsafe(&mut array[i..]);
	merge_queue.push_back(array.len());

	let mut start = 0;
	let mut in_array = true;
	let mut new_merge_queue: VecDeque<usize> = VecDeque::new();
	let mut buffer: Vec<T> = vec![array[0]; array.len()];
	while merge_queue.len() > 1 {
		let split = merge_queue.pop_front().unwrap();
		let end = merge_queue.pop_front().unwrap();
		if in_array {
			merge_to_buffer(array, start, split, end, &mut buffer);
		} else {
			merge_to_buffer(&mut buffer, start, split, end, array);
		}
		new_merge_queue.push_back(end);
		start = end;

		if merge_queue.len() <= 1 {
			if merge_queue.len() == 1 {
				// TODO consider merging this here instead of postponing this.
				let end = merge_queue.pop_front().unwrap();
				if in_array {
					buffer[start..end].clone_from_slice(&array[start..end]);
				} else {
					array[start..end].clone_from_slice(&buffer[start..end]);
				}
				new_merge_queue.push_back(end);
			}

			merge_queue = new_merge_queue;
			start = 0;
			in_array = !in_array;
			new_merge_queue = VecDeque::new();
		}
	}

	if !in_array {
		array.clone_from_slice(&buffer);
	}
}
