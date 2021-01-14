pub trait SliceUnchecked<T> {
	unsafe fn swap_unchecked(&mut self, i: usize, j: usize);
	unsafe fn set_unchecked(&mut self, i: usize, value: T);
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
}
