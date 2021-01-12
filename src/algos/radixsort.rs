use std::collections::VecDeque;

fn bucketsort<T: Copy>(array: &mut [T], buckets: &mut Vec<VecDeque<T>>, extract_bits: fn(&mut T) -> u8) {
    unsafe {
        for n in array.iter_mut() {
            (*buckets.get_unchecked_mut(extract_bits(n) as usize)).push_back(*n);
        }
        let mut i = 0;
        let mut j = 0;
        while i < buckets.len() {
            let n = (*buckets.get_unchecked_mut(i)).pop_front();
            if n.is_some() {
                *array.get_unchecked_mut(j) = n.unwrap();
                j += 1;
            } else {
                i += 1;
            }
        }
    }
}

pub fn radixsort(array: &mut [i32]) {
	// 1-time bucket allocation shared over all bucketsorts
	// correct because bucketsort will fully empty out each deque
	// on average each deque will expect array.len() / 256 items however over-allocating by a
	// factor of 2 should substantially reduce runtime allocations
	// any runtime allocations will also be carried across the bucketsort calls
	// note: linked list performance was also tested and deques have much improved performance
	let mut buckets: Vec<VecDeque<i32>> = vec![VecDeque::with_capacity(std::cmp::max(array.len() / 256 * 2, 8)); 256];
    bucketsort(array, &mut buckets, |x| *x as u8);
    bucketsort(array, &mut buckets, |x| (*x >> 8) as u8);
    bucketsort(array, &mut buckets, |x| (*x >> 16) as u8);
    bucketsort(array, &mut buckets, |x| ((*x >> 24) as u8) ^ 0x80);
}
