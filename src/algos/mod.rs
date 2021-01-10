pub mod bubblesort;
pub use bubblesort::*;

pub mod cocktail_shaker;
pub use cocktail_shaker::*;

pub mod heapsort;
pub use heapsort::*;

pub mod insertionsort;
pub use insertionsort::*;

pub mod mergesort;
pub use mergesort::*;

pub mod misc;
pub use misc::*;

pub mod quicksort;
pub use quicksort::*;

pub mod radixsort;
pub use radixsort::*;

pub mod selectionsort;
pub use selectionsort::*;

pub mod shellsort;
pub use shellsort::*;

pub mod timsort;
pub use timsort::*;

// It would be elegant to just pace a rust slice to the c code. Turns out it's pretty
// straightforward to do too. It would allow the c functions to match the signature of the other
// sorting algorithms. Unfortunately, we can't just plug in the c functions into the rust code
// because c functions are unsafe and require a wrapper regardless.
// I'm hoping LTO can optimize out the wrapper. TODO: verify this happens.
//#[allow(dead_code)]
//#[repr(C)] pub struct slice_t { pub ptr: usize, pub size: i32 }
#[link(name="insertionsort", kind="static")]
extern "C" {
	//#[allow(improper_ctypes)]
	//pub fn c_insertionsort(array: &mut [i32]);
	pub fn c_insertionsort(array: libc::intptr_t, length: i32);
	pub fn std_sort(array: libc::intptr_t, length: i32);
	//pub fn std_stable_sort(array: libc::intptr_t, length: i32);
}

//pub fn insertionsort_c(array: &mut [i32]) {
//	unsafe {
//		c_insertionsort(array);
//	}
//}

pub fn insertionsort_c(array: &mut [i32]) {
	unsafe {
		c_insertionsort(array.as_ptr() as isize, array.len() as i32);
	}
}

pub fn cpp_std_sort(array: &mut [i32]) {
	unsafe {
		std_sort(array.as_ptr() as isize, array.len() as i32);
	}
}

//pub fn cpp_std_stable_sort(array: &mut [i32]) {
//	unsafe {
//		std_stable_sort(array.as_ptr() as isize, array.len() as i32);
//	}
//}
