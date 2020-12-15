
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

pub fn merge<T: Ord + Copy>(slice: &mut [T]) {
    if slice.len() <= 32 {
        insertion(slice);
        return;
    }
    let slice_len = slice.len();
    let middle = slice_len / 2;
    merge(&mut slice[0..middle]);
    merge(&mut slice[middle..slice_len]);
    merge_single(slice, middle);
}

pub fn weird<T: Ord + Copy + std::fmt::Debug>(slice: &mut [T]) {
    let chunk_size = (slice.len() as f64).sqrt() as usize;
    for chunk in slice.chunks_mut(chunk_size) {
        insertion(chunk);
    }
    let mut step = 1;
    let mut merge_count = 1;
    while merge_count > 0 {
        merge_count = 0;
        for i in ((step * chunk_size)..slice.len()).step_by(step * chunk_size * 2) {
            let slice_len = slice.len();
            if i + step * chunk_size < slice_len {
                merge_single(&mut slice[(i - step * chunk_size)..(i + step * chunk_size)],
                    step * chunk_size);
            } else {
                merge_single(&mut slice[(i - step * chunk_size)..slice_len], step * chunk_size);
            }
            merge_count += 1;
        }
        step *= 2;
    }
}
