pub trait SwapUnsafe {
	unsafe fn swap_unchecked(&mut self, i: usize, j: usize);
}

//impl<T: Copy> SwapUnsafe for [T] {
impl<T> SwapUnsafe for [T] {
	#[inline]
	unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
		//let tmp = *self.get_unchecked(j);
		//*self.get_unchecked_mut(j) = *self.get_unchecked(i);
		//*self.get_unchecked_mut(i) = tmp;
		let pa: *mut T = self.get_unchecked_mut(i);
		let pb: *mut T = self.get_unchecked_mut(j);
		std::ptr::swap(pa, pb);
	}
}
