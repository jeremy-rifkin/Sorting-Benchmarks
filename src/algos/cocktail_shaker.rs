use crate::unchecked_tools::SliceUnchecked;

pub fn cocktail_shaker<T: Ord>(array: &mut[T]) {
	unsafe {
		let mut lower = 0 as usize; // first unsorted element
		let mut upper = array.len() - 1; // last unsorted element
		while lower < upper {
			let mut last_swap = 0;
			for i in lower..upper {
				if array.get_unchecked(i) > array.get_unchecked(i + 1) {
					last_swap = i;
					array.swap_unchecked(i, i + 1);
				}
			}
			upper = last_swap;
			for i in (lower..upper).rev() {
				if array.get_unchecked(i) > array.get_unchecked(i + 1) {
					last_swap = i + 1;
					array.swap_unchecked(i, i + 1);
				}
			}
			lower = last_swap;
		}
	}
}
