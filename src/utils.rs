use rand::RngCore;
use rand::rngs::SmallRng;

#[macro_export] macro_rules! pair {
	($f:expr) => {
		($f, &Regex::new("::<.+>$").unwrap().replace(stringify!($f), ""))
	};
}

pub fn create_sorted_array(size: usize) -> Vec<i32> {
	let mut array = vec![0; size];
	for i in 0..size {
		array[i] = i as i32;
	}
	return array;
}

pub fn create_reversed_array(size: usize) -> Vec<i32> {
	let mut array = vec![0; size];
	for (i, v) in (0..size).rev().enumerate() {
		array[i] = v as i32;
	}
	return array;
}

pub fn create_random_array(size: usize, rng: &mut SmallRng) -> Vec<i32> {
	let mut array = vec![0; size];
	for i in (0..size).rev() {
		array[i] = rng.next_u32() as i32;
	}
	return array;
}

pub fn verify_sorted(array: &[i32]) {
	assert!(array.windows(2).all(|slice| slice[0] <= slice[1]));
}
