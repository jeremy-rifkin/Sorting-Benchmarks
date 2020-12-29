// this macro is shorthand for adding algorithms to to benchmark
// input: function pointer, string
// output: tuple (fn pointer, fn name, string)
#[macro_export] macro_rules! sfn {
	($f:expr, $c:literal) => {
		($f, (&Regex::new("::<.+>$").unwrap().replace(stringify!($f), "")).to_string(), $c)
	};
}

// returns number with comma separators (i.e. 1000000 -> "1,000,000")
pub fn commafy(mut num: usize) -> String {
	let log = (num as f64).log10() as usize;
	let len = log + log / 3 + 1;
	let mut s = vec![b'x'; len];
	let mut i = 0;
	let mut count = 0;
	while num > 0 {
		if count > 0 && count % 3 == 0 {
			s[len - i - 1] = b',';
			i += 1;
		}
		s[len - i - 1] = b'0' + (num % 10) as u8;
		i += 1;
		num /= 10;
		count += 1;
	}
	return String::from_utf8(s).unwrap();
}

pub fn verify_sorted(array: &[i32]) {
	assert!(array.windows(2).all(|slice| slice[0] <= slice[1]));
}

// computes the sample standard deviation of a Vec<u64>
// mean passed as a parameter to take advantage of pre-computed value
pub fn stdev(array: &[u64], mean: f64) -> f64 {
	let mut sum = 0.0;
	for i in 0..array.len() {
		sum += (array[i] as f64 - mean).powi(2);
	}
	sum /= (array.len() - 1) as f64;
	sum.sqrt()
}

#[derive(Debug)]
pub struct QuartileDescriptor {
	q1: f64,
	q2: f64,
	q3: f64,
	iqr: f64
}

fn median(array: &[u64]) -> f64 {
	if array.len() % 2 == 0 {
		(array[array.len() / 2 - 1] + array[array.len() / 2]) as f64 / 2.0
	} else {
		array[array.len() / 2] as f64
	}
}

pub fn quartiles(array: &Vec<u64>) -> QuartileDescriptor {
	let mut array = array.clone();
	array.sort();
	if array.len() % 2 == 0 {
		let q1 = median(&array[..array.len() / 2]) as f64;
		let q2 = median(&array);
		let q3 = median(&array[array.len() / 2..]) as f64;
		QuartileDescriptor {q1, q2, q3, iqr: q3 - q1}
	} else {
		let q1 = median(&array[..array.len() / 2]) as f64;
		let q2 = median(&array);
		let q3 = median(&array[array.len() / 2 + 1..]) as f64;
		QuartileDescriptor {q1, q2, q3, iqr: q3 - q1}
	}
}

pub fn tukey(item: u64, q: &QuartileDescriptor, threshold: f64) -> bool {
	let item = item as f64;
	!(item > q.q3 + threshold * q.iqr || item < q.q1 - threshold * q.iqr)
}
