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

#[allow(dead_code)]
#[repr(C)] pub struct slice_t { pub ptr: usize, pub size: i32 }
#[link(name="insertionsort", kind="static")]
extern "C" {
	#[allow(dead_code)]
	pub fn c_test();
	#[allow(improper_ctypes)]
	//pub fn c_insertionsort(array: &mut [i32]);
	pub fn c_insertionsort(array: libc::intptr_t, length: i32);
	//pub fn c_slice_test(array: &mut [i32]) -> slice_t;
	//pub fn c_insertionsort_s(array: &[i32]);
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
