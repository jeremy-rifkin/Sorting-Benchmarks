use crate::swap_unsafe::SwapUnsafe;

pub fn selectionsort<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		for i in 0..(array.len() - 1) {
			let mut min = array.get_unchecked(i);
			let mut min_index = i;
			for j in (i + 1)..array.len() {
				if array.get_unchecked(j) < min {
					min = array.get_unchecked(j);
					min_index = j;
				}
			}
			array.swap_unchecked(i, min_index);
		}
	}
}

pub fn drugsort<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		let mut finding_max = true;
		let mut edge_space = 0;
		while edge_space * 2 + (!finding_max as usize) < array.len() {
			if finding_max {
				let mut max = array.get_unchecked(edge_space);
				let mut max_index = edge_space;
				for i in (edge_space + 1)..(array.len() - edge_space) {
					if array.get_unchecked(i) > max {
						max = array.get_unchecked(i);
						max_index = i;
					}
				}
				array.swap_unchecked(array.len() - 1 - edge_space, max_index);
				finding_max = false;
			} else {
				let mut min = array.get_unchecked(array.len() - 1 - edge_space);
				let mut min_index = array.len() - 1 - edge_space;
				for i in edge_space..(array.len() - 1 - edge_space) {
					if array.get_unchecked(i) < min {
						min = array.get_unchecked(i);
						min_index = i;
					}
				}
				array.swap_unchecked(edge_space, min_index);
				finding_max = true;
				edge_space += 1;
			}
		}
	}
}

pub fn methsort<T: Ord + Copy>(array: &mut [T]) {
	unsafe {
		for i in 0..(array.len() / 2) {
			let mut min = *array.get_unchecked(i);
			let mut min_index = i;
			let mut max = *array.get_unchecked(i);
			let mut max_index = i;
			for j in (i + 1)..(array.len() - i) {
				if *array.get_unchecked(j) < min {
					min = *array.get_unchecked(j);
					min_index = j;
				} else if *array.get_unchecked(j) > max {
					max = *array.get_unchecked(j);
					max_index = j;
				}
			}
			if i == max_index {
				max_index = min_index;
			}
			array.swap_unchecked(i, min_index);
			array.swap_unchecked(array.len() - 1 - i, max_index);
		}
	}
}
