use std::collections::LinkedList;

pub fn bucketsort<T>(array: &mut [T], extract_bits: &dyn Fn(T) -> u8) {
    let mut buckets: Vec<LinkedList<u8>> = vec![LinkedList::new(); 256];
}
