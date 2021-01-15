use crate::algos::insertionsort;
use std::num::Wrapping;
use std::mem::size_of;
use std::cmp::min;

// Binary search for where the element should end up in a sorted array.
// This version of binary search always returns a valid index, even if the exact element is not
// found.
unsafe fn binary_search<T: Ord>(slice: &[T], element: T) -> usize {
    let mut left = 0;
    let mut right = slice.len();
    while left < right {
        let middle = (left + right) / 2;
        if *slice.get_unchecked(middle) < element {
            left = middle + 1;
        } else if *slice.get_unchecked(middle) > element {
            right = middle;
        } else {
            return middle;
        }
    }
    left
}

// Merge 2 slices into destination. This merging algorithm uses galloping in order to hopefully
// optimize merging even more. short_merge_left() merges from left to right, while
// short_merge_right() merges from right to left.
unsafe fn short_merge_left<T: Ord + Copy + std::fmt::Debug>(first: &[T], second: &[T], destination: &mut [T]) {
    let mut i = 0;
	let mut j = 0;
	let mut k = 0;
	while i < first.len() && j < second.len() {
		if *first.get_unchecked(i) < *second.get_unchecked(j) {
			*destination.get_unchecked_mut(k) = *first.get_unchecked(i);
			i += 1;
		} else {
			*destination.get_unchecked_mut(k) = *second.get_unchecked(j);
			j += 1;
		}
        k += 1;
        // TODO implement galloping.
    }
    
	while i < first.len() {
        *destination.get_unchecked_mut(k) = *first.get_unchecked(i);
        i += 1;
        k += 1;
    }

    while j < second.len() {
        *destination.get_unchecked_mut(k) = *second.get_unchecked(j);
        j += 1;
        k += 1;
    }
}

unsafe fn short_merge_right<T: Ord + Copy + std::fmt::Debug>(first: &[T], second: &[T], destination: &mut [T]) {
	// Exactly the same implementation as short_merge_left(), but mirrored.
    let mut i = first.len() as isize - 1;
	let mut j = second.len() as isize - 1;
	let mut k = destination.len() as isize - 1;
	while i >= 0 && j >= 0 {
		if *first.get_unchecked(i as usize) > *second.get_unchecked(j as usize) {
			*destination.get_unchecked_mut(k as usize) = *first.get_unchecked(i as usize);
			i -= 1;
		} else {
			*destination.get_unchecked_mut(k as usize) = *second.get_unchecked(j as usize);
			j -= 1;
		}
        k -= 1;
        // TODO implement galloping.
    }
    
	while i >= 0 {
        *destination.get_unchecked_mut(k as usize) = *first.get_unchecked(i as usize);
        i -= 1;
        k -= 1;
    }
    
    while j >= 0 {
        *destination.get_unchecked_mut(k as usize) = *second.get_unchecked(j as usize);
        j -= 1;
        k -= 1;
    }
}

// A quick merging algorithm that uses binary search in order to reduce the number of elements
// needed to be merged and temporary memory in order to reduce the amount of out-of-space memory.
// The merging itself is done by short_merge().
unsafe fn quick_merge<T: Ord + Copy + std::fmt::Debug>(slice: &mut [T], split: usize, buffer: &mut Vec<T>) {
    let left = binary_search(&slice[..split], *slice.get_unchecked(split));
    let right = binary_search(&slice[split..], *slice.get_unchecked(split - 1)) + split;
    if split - left < right - split {
        buffer.clear();
        for element in &slice[left..split] {
            buffer.push(*element);
        }
        let destination = std::slice::from_raw_parts_mut(&mut slice[left] as *mut T, right - left); // TODO undo
        short_merge_left(buffer, &slice[split..right], destination);
    } else {
        buffer.clear();
        for element in &slice[split..right] {
            buffer.push(*element);
        }
        let destination = std::slice::from_raw_parts_mut(&mut slice[left] as *mut T, right - left); // TODO undo
        short_merge_right(buffer, &slice[left..split], destination);
    }
}

pub fn timsort<T: Ord + Copy + std::fmt::Debug>(slice: &mut [T]) {
    unsafe {
        let left_shift_distance = size_of::<usize>() * 8 + 6;
        let right_shift_distance = (size_of::<usize>() * 8 - 6) as isize - slice.len().leading_zeros() as isize;
        let mut min_run_len = if right_shift_distance > 0 {(Wrapping(slice.len()) >> right_shift_distance as usize).0} else {slice.len()};
		min_run_len += (((Wrapping(slice.len()) << left_shift_distance) >> left_shift_distance).0 != 0) as usize;

        let mut run_stack = vec![0];
        let mut start = 0;
        let mut end = 1; // TODO handle slice of size 1 or 0.
        let mut buffer = Vec::new();
        while end < slice.len() || run_stack.len() > 2 {
            // Add a new run to the stack.
            if end < slice.len() {
                end = min(start + min_run_len, slice.len());
                insertionsort(&mut slice[start..end]);
                run_stack.push(end);
                start = end;
            }

            // Merge runs on the stack as long as the requirements are satisfied.
            if run_stack.len() > 2 {
                let mut x = *run_stack.get_unchecked(run_stack.len() - 1) - *run_stack.get_unchecked(run_stack.len() - 2);
                let mut y = *run_stack.get_unchecked(run_stack.len() - 2) - *run_stack.get_unchecked(run_stack.len() - 3);
                if run_stack.len() > 3 {
                    let mut z = *run_stack.get_unchecked(run_stack.len() - 3) - *run_stack.get_unchecked(run_stack.len() - 4);
                    while run_stack.len() > 3 && ((z <= y + x || y <= x) || end == slice.len()) {
                        if z <= x {
                            quick_merge(&mut slice[*run_stack.get_unchecked(run_stack.len() - 4)..*run_stack.get_unchecked(run_stack.len() - 2)], z, &mut buffer);
                            run_stack.remove(run_stack.len() - 3);
                            y = y + z;
                        } else {
                            quick_merge(&mut slice[*run_stack.get_unchecked(run_stack.len() - 3)..*run_stack.get_unchecked(run_stack.len() - 1)], y, &mut buffer);
                            run_stack.remove(run_stack.len() - 2);
                            x = x + y;
                            y = z;
                        }

                        if run_stack.len() > 3 {
                            z = *run_stack.get_unchecked(run_stack.len() - 3) - *run_stack.get_unchecked(run_stack.len() - 4);
                        }
                    }
                }

                while run_stack.len() > 2 && (y <= x || end == slice.len()) {
                    quick_merge(&mut slice[*run_stack.get_unchecked(run_stack.len() - 3)..*run_stack.get_unchecked(run_stack.len() - 1)], y, &mut buffer);
                    run_stack.remove(run_stack.len() - 2);
                    x = x + y;

                    if run_stack.len() > 2 {
                        y = *run_stack.get_unchecked(run_stack.len() - 2) - *run_stack.get_unchecked(run_stack.len() - 3);
                    }
                }
            }
        }
    }
}
