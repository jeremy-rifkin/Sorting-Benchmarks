use crate::algos;
use crate::unchecked_tools::SliceUnchecked;

// TODO: get rid of
pub fn merge_single<T: Ord + Copy>(slice: &mut [T], middle: usize) {
	unsafe {
		let mut i = 0;
		let mut j = middle;
		let mut k = 0;
		let mut merged: Vec<T> = Vec::with_capacity(slice.len());
		merged.set_len(slice.len());
		while i < middle && j < slice.len() {
			*merged.get_unchecked_mut(k) = if *slice.get_unchecked(i) < *slice.get_unchecked(j)
				{i += 1; *slice.get_unchecked(i - 1)} else {j += 1; *slice.get_unchecked(j - 1)};
			k += 1;
		}
		while i < middle {
			*merged.get_unchecked_mut(k) = *slice.get_unchecked(i);
			i += 1;
			k += 1;
		}
		while j < slice.len() {
			*merged.get_unchecked_mut(k) = *slice.get_unchecked(j);
			j += 1;
			k += 1;
		}
		slice.copy_from_slice(&merged);
	}
}

fn merge<T: Ord + Copy>(slice: &mut [T], buffer: &mut Vec<T>) {
	unsafe {
		let middle = slice.len() / 2;
		let mut i = 0;
		let mut j = middle;
		let mut k = 0;
		while i < middle && j < slice.len() {
			if slice.get_unchecked(i) < slice.get_unchecked(j) {
				i += 1;
				*buffer.get_unchecked_mut(k) = *slice.get_unchecked(i - 1);
			} else {
				j += 1;
				*buffer.get_unchecked_mut(k) = *slice.get_unchecked(j - 1);
			}
			k += 1;
		}
		while i < middle {
			*buffer.get_unchecked_mut(k) = *slice.get_unchecked(i);
			i += 1;
			k += 1;
		}
		while j < slice.len() {
			*buffer.get_unchecked_mut(k) = *slice.get_unchecked(j);
			j += 1;
			k += 1;
		}
		slice.copy_from_slice(&buffer[..slice.len()]);
	}
}

pub fn mergesort<T: Ord + Copy>(array: &mut [T]) {
	let mut buffer: Vec<T> = Vec::with_capacity(array.len());
	unsafe { buffer.set_len(array.len()); }
	mergesort_step(array, &mut buffer);
}

fn mergesort_step<T: Ord + Copy>(array: &mut [T], buffer: &mut Vec<T>) {
	if array.len() <= 1 {
		return;
	}
	let middle = array.len() / 2;
	// these array slices will have their boundary checks optimized out
	mergesort_step(&mut array[..middle], buffer);
	mergesort_step(&mut array[middle..], buffer);
	merge(array, buffer);
}

pub fn mergesort_hybrid<T: Ord + Copy>(array: &mut [T]) {
	let mut buffer: Vec<T> = Vec::with_capacity(array.len());
	unsafe { buffer.set_len(array.len()); }
	mergesort_hybrid_r(array, &mut buffer);
}

fn mergesort_hybrid_r<T: Ord + Copy>(array: &mut [T], buffer: &mut Vec<T>) {
	if array.len() <= algos::INSERTIONSORT_THRESHOLD {
		algos::insertionsort(array);
		return;
	}
	let middle = array.len() / 2;
	// these array slices will have their boundary checks optimized out
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
	unsafe {
		if array.get_unchecked(mid - 1) <= array.get_unchecked(mid) {
			return;
		}
		while i < mid && j < array.len() {
			if array.get_unchecked(i) <= array.get_unchecked(j) {
				i += 1;
			} else {
				let v = *array.get_unchecked(j);
				let mut k = j;
				while k > i {
					*array.get_unchecked_mut(k) = *array.get_unchecked(k - 1);
					k -= 1;
				}
				*array.get_unchecked_mut(i) = v;
				i += 1;
				mid += 1;
				j += 1;
			}
		}
	}
}

pub fn mergesort_in_place_naive<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 1 {
		return;
	}
	let middle = array.len() / 2;
	// these array slices will have their boundary checks optimized out
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
			unsafe {
				while m < u && xs.get_unchecked(m) < xs.get_unchecked(m-1) {
					xs.swap_unchecked(m, m - 1);
					m += 1;
				}
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
			unsafe {
				xs.swap_unchecked(l, w);
			}
			l += 1;
			w += 1;
		}
	}
}

fn wmerge<T: Ord + Copy>(xs: &mut [T], mut i: usize, m: usize, mut j: usize, n: usize, mut w: usize) {
	unsafe {
		while i < m && j < n {
			let k;
			if xs.get_unchecked(i) < xs.get_unchecked(j) {
				k = i;
				i += 1;
			} else {
				k = j;
				j += 1;
			}
			xs.swap_unchecked(w, k);
			w += 1;
		}
		while i < m {
			xs.swap_unchecked(w, i);
			w += 1;
			i += 1;
		}
		while j < n {
			xs.swap_unchecked(w, j);
			w += 1;
			j += 1;
		}
	}
}
