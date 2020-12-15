
pub fn insertion_gap<T: Ord>(slice: &mut [T], gap: usize) {
    if gap == 0 {
        panic!("Invalid argument! gap has to be at least 1.");
    }

    for i in 1..(gap + 1) {
        for j in (i..slice.len()).step_by(gap) {
            let mut k = j;
            while k >= gap && slice[k] < slice[k - gap] {
                slice.swap(k, k - gap);
                k -= gap;
            }
        }
    }
}

pub fn insertion<T: Ord>(slice: &mut [T]) {
    insertion_gap(slice, 1);
}

pub fn shell_sequence<'a, T: Ord>(slice: &mut [T], gap_sequence: impl Iterator<Item = &'a usize>) {
    for gap in gap_sequence {
        if *gap < slice.len() {
            insertion_gap(slice, *gap);
        }
    }
}

pub fn shell_function<T: Ord>(slice: &mut [T], gap_function: impl Fn(usize) -> usize) {
    let mut i = 0;
    while gap_function(i) < slice.len() {
        i += 1;
    }
    for j in (0..i).rev() {
        insertion_gap(slice, gap_function(j));
    }
}

pub fn shell<T: Ord>(slice: &mut [T]) {
    const DEFAULT_SEQUENCE: [usize; 8] = [701, 301, 132, 57, 23, 10, 4, 1];
    shell_sequence(slice, DEFAULT_SEQUENCE.iter());
}

pub fn merge_single<T: Ord + Copy>(slice: &mut [T], middle: usize) {
    let mut i = 0;
    let mut j = middle;
    let mut k = 0;
    let mut merged: Vec<T> = vec![slice[0]; slice.len()];
    while i < middle && j < slice.len() {
        merged[k] = if slice[i] < slice[j] {i += 1; slice[i - 1]} else {j += 1; slice[j - 1]};
        k += 1;
    }
    while i < middle {
        merged[k] = slice[i];
        i += 1;
        k += 1;
    }
    while j < slice.len() {
        merged[k] = slice[j];
        j += 1;
        k += 1;
    }
    slice.copy_from_slice(&merged);
}

pub fn weird<T: Ord + Copy>(slice: &mut [T]) {
    let chunk_size = (slice.len() as f64).sqrt() as usize;
    for chunk in slice.chunks_mut(chunk_size) {
        insertion(chunk);
    }
    for i in (chunk_size..slice.len()).step_by(chunk_size) {
        let slice_len = slice.len();
        if i + chunk_size < slice_len {
            merge_single(&mut slice[0..(i + chunk_size)], i)
        } else {
            merge_single(&mut slice[0..slice_len], i)
        }
    }
}
