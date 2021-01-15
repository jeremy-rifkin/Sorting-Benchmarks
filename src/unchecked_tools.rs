use std::slice::from_raw_parts_mut;

pub trait SliceUnchecked<T> {
	unsafe fn swap_unchecked(&mut self, i: usize, j: usize);
	unsafe fn set_unchecked(&mut self, i: usize, value: T);
	// because slice.split_at_mut_unchecked is not public... have to re-implement here
	//unsafe fn split_at_unchecked(&mut self, i: usize) -> (&mut [T], &mut [T]);
	unsafe fn split_at_unchecked_mut(&mut self, i: usize) -> (&mut [T], &mut [T]);
	//unsafe fn split_at_unchecked_excl(&mut self, i: usize) -> (&mut [T], &mut [T]);
	unsafe fn split_at_unchecked_mut_excl(&mut self, i: usize) -> (&mut [T], &mut [T]);
}

//impl<T: Copy> SwapUnsafe for [T] {
impl<T> SliceUnchecked<T> for [T] {
	unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
		//let tmp = *self.get_unchecked(j);
		//*self.get_unchecked_mut(j) = *self.get_unchecked(i);
		//*self.get_unchecked_mut(i) = tmp;
		let pa: *mut T = self.get_unchecked_mut(i);
		let pb: *mut T = self.get_unchecked_mut(j);
		std::ptr::swap(pa, pb);
	}
	unsafe fn set_unchecked(&mut self, i: usize, value: T) {
		*self.get_unchecked_mut(i) = value;
	}
	// because slice.split_at_mut_unchecked is not public... have to re-implement here
	// returns [0, i), [i, len)
	unsafe fn split_at_unchecked_mut(&mut self, i: usize) -> (&mut [T], &mut [T]) {
		let len = self.len();
		let ptr = self.as_mut_ptr();
		(from_raw_parts_mut(ptr, i), from_raw_parts_mut(ptr.add(i), len - i))
	}
	// returns [0, i), (i, len)
	unsafe fn split_at_unchecked_mut_excl(&mut self, i: usize) -> (&mut [T], &mut [T]) {
		let len = self.len();
		let ptr = self.as_mut_ptr();
		(from_raw_parts_mut(ptr, i), from_raw_parts_mut(ptr.add(i + 1), len - i - 1))
	}
}
