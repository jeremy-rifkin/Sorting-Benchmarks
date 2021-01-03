use std::collections::VecDeque;

fn bucketsort<T: Copy>(array: &mut [T], extract_bits: fn(&mut T) -> u8) {
    let mut buckets: Vec<VecDeque<T>> = vec![VecDeque::new(); 256];
    for n in array.iter_mut() {
        buckets[extract_bits(n) as usize].push_back(*n);
    }
    let mut i = 0;
    let mut j = 0;
    while i < buckets.len() {
        let n = buckets[i].pop_front();
        if n.is_some() {
            array[j] = n.unwrap();
            j += 1;
        } else {
            i += 1;
        }
    }
}

pub fn radixsort(array: &mut [i32]) {
    bucketsort(array, |x| *x as u8);
    bucketsort(array, |x| (*x >> 8) as u8);
    bucketsort(array, |x| (*x >> 16) as u8);
    bucketsort(array, |x| ((*x >> 24) as u8) ^ 0x80);
}
