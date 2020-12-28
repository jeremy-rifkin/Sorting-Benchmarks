use crate::algos;

fn partition<T: Ord + Copy>(slice: &mut [T]) -> usize {
    let pivot = slice[slice.len() - 1];
    let mut i = 0;
    for j in 0..(slice.len() - 1) {
        if slice[j] < pivot {
            slice.swap(i, j);
            i += 1;
        }
    }
    slice.swap(i, slice.len() - 1);
    return i;
}

pub fn quicksort<T: Ord + Copy>(array: &mut [T]) {
	if array.len() <= 32 {
        algos::insertionsort(array);
        return;
    }
    let pivot = partition(array);
    let array_len = array.len();
    quicksort(&mut array[0..pivot]);
    quicksort(&mut array[pivot..array_len]);
}
