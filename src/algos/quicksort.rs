use rand::rngs::SmallRng;
use rand::{Rng, RngCore, SeedableRng};
use crate::algos;

fn partition<T: Ord + Copy>(slice: &mut [T], rng: &mut SmallRng) -> usize {
    let pivot_index = rng.gen_range(0..slice.len());
    //let pivot_index = slice.len() / 2;
    slice.swap(pivot_index, slice.len() - 1);
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

pub fn quicksort_rng<T: Ord + Copy>(array: &mut [T], rng: &mut SmallRng) {
	if array.len() <= 32 {
        algos::insertionsort(array);
        return;
    }
    let pivot = partition(array, rng);
    quicksort_rng(&mut array[..pivot], rng);
    quicksort_rng(&mut array[(pivot + 1)..], rng);
}

pub fn quicksort<T: Ord + Copy>(array: &mut [T]) {
    let mut rng = SmallRng::from_entropy();
    quicksort_rng(array, &mut rng);
}
