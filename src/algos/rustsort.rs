// wrapper for rust's sorting algorithm
#[cfg(not(tarpaulin_include))]
pub fn rustsort<T: Ord>(slice: &mut [T]) {
	slice.sort();
}

// wrapper for rust's sorting algorithm
#[cfg(not(tarpaulin_include))]
pub fn rustsort_unsable<T: Ord>(slice: &mut [T]) {
	slice.sort_unstable();
}
