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

// struct used for describing the quartiles of a data set
#[derive(Debug)]
pub struct QuartileDescriptor {
	q1: f64,
	q2: f64,
	q3: f64,
	iqr: f64
}

// returns the median value of an array
fn median(array: &[u64]) -> f64 {
	if array.len() % 2 == 0 {
		(array[array.len() / 2 - 1] + array[array.len() / 2]) as f64 / 2.0
	} else {
		array[array.len() / 2] as f64
	}
}

// finds the quartiles and iqr of an array
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

// filter function to filter out outliers in a data set
pub fn tukey(item: u64, q: &QuartileDescriptor, threshold: f64) -> bool {
	let item = item as f64;
	!(item > q.q3 + threshold * q.iqr || item < q.q1 - threshold * q.iqr)
}

// gamma function implementation from the statrs crate
// https://docs.rs/statrs/0.7.0/src/statrs/function/gamma.rs.html#54-72
// https://github.com/boxtown/statrs/blob/8518fe7fc3e3de64ecbd45e3ad05ae98d68b945b/src/consts.rs
// MIT license
use std::f64;
/// Constant value for `2 * sqrt(e / pi)`
const TWO_SQRT_E_OVER_PI: f64 = 1.8603827342052657173362492472666631120594218414085755;
/// Auxiliary variable when evaluating the `gamma_ln` function
const GAMMA_R: f64 = 10.900511;
/// Polynomial coefficients for approximating the `gamma_ln` function
const GAMMA_DK: &'static [f64] = &[2.48574089138753565546e-5,
                                   1.05142378581721974210,
                                   -3.45687097222016235469,
                                   4.51227709466894823700,
                                   -2.98285225323576655721,
                                   1.05639711577126713077,
                                   -1.95428773191645869583e-1,
                                   1.70970543404441224307e-2,
                                   -5.71926117404305781283e-4,
                                   4.63399473359905636708e-6,
								   -2.71994908488607703910e-9];
/// Computes the gamma function with an accuracy
/// of 16 floating point digits. The implementation
/// is derived from "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
pub fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        let s = GAMMA_DK.iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));

        f64::consts::PI /
        ((f64::consts::PI * x).sin() * s * TWO_SQRT_E_OVER_PI *
         ((0.5 - x + GAMMA_R) / f64::consts::E).powf(0.5 - x))
    } else {
        let s = GAMMA_DK.iter()
            .enumerate()
            .skip(1)
            .fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));

        s * TWO_SQRT_E_OVER_PI * ((x - 0.5 + GAMMA_R) / f64::consts::E).powf(x - 0.5)
    }
}


// implementation of a two sample t test
fn pochhammer(q: f64, n: i32) -> f64 {
	if n == 0 {
		1.0
	} else {
		let mut acc = 1.0;
		for m in 0..n {
			acc *= q + m as f64;
		}
		acc
	}
}

fn factorial(n: i32) -> f64 {
	if n <= 1 {
		1.0
	} else {
		let mut acc = 1.0;
		for i in 2..=n {
			acc *= i as f64;
		}
		acc
	}
}

// computes part of the hypergeometric function while avoiding overflow from the rising factorials
// and factorials on both the top and bottom
//   (a)_n (b)_n    1
//   ----------- * ---
//      (c)_n       n!
fn pochhammer_factorial(a: f64, b: f64, c: f64, n: i32) -> f64 {
	let a_gen = |i: i32| a + i as f64;
	let b_gen = |i: i32| b + i as f64;
	let c_gen = |i: i32| c + i as f64;
	let f_gen = |i: i32| (i + 1) as f64;
	let mut acc = 1.0;
	for i in 0..n {
		acc *= a_gen(i) * b_gen(i) / c_gen(i) / f_gen(i);
	}
	acc
}

#[allow(non_snake_case)]
fn hypergeometric2F1___(a: f64, b: f64, c: f64, z: f64) -> f64 {
	assert!(c >= 0.0);
	assert!(z.abs() < 1.0);
	let mut sum = 0.0;
	let mut n = 0;
	loop {
		let _a = pochhammer(a, n);
		let _b = pochhammer(b, n);
		let _c = pochhammer(c, n);
		let _p = z.powi(n);
		let _f = factorial(n);
		//let v = pochhammer(a, n) * pochhammer(b, n) / pochhammer(c, n) * z.powi(n) / factorial(n);
		let v = pochhammer(a, n) / pochhammer(c, n) * z.powi(n) * pochhammer(b, n) / factorial(n);
		sum += v;
		if v.abs() < 0.0001 {
			break;
		}
		if n >= 10_000 || v.is_infinite() {
			println!("{} {}", v, n);
			panic!("at the disco");
		}
		n += 1;
	}
	sum
}
#[allow(non_snake_case)]
fn hypergeometric2F1(a: f64, b: f64, c: f64, z: f64) -> f64 {
	assert!(c >= 0.0);
	assert!(z.abs() < 1.0);
	let mut sum = 0.0;
	let mut n = 0;
	loop {
		let v = pochhammer_factorial(a, b, c, n) * z.powi(n);
		sum += v;
		if v.abs() < 0.0001 {
			break;
		}
		if n >= 10_000 || v.is_infinite() || v.is_nan() {
			println!("{} {}", v, n);
			panic!("at the disco");
		}
		n += 1;
	}
	sum
}

pub fn t_cdf(x: f64, v: f64) -> f64 {
	//println!("t_cdf({}, {})", x, v);
	//println!("gamma({}) = {}", (v + 1.0) / 2.0, gamma((v + 1.0) / 2.0));
	//println!("hypergeometric2F1({}, {}, {}, {}) = {}", 0.5, (v + 1.0) / 2.0, 1.5, -x*x/v, hypergeometric2F1(0.5, (v + 1.0) / 2.0, 1.5, -x*x/v));
	//println!("/ {}", (v * f64::consts::PI).sqrt() * gamma(v / 2.0));
	0.5 + x * gamma((v + 1.0) / 2.0) * hypergeometric2F1(0.5, (v + 1.0) / 2.0, 1.5, -x*x/v) / ((v * f64::consts::PI).sqrt() * gamma(v / 2.0))
}

// Welch's t-test
pub fn two_sample_t_test(mean1: f64, mean2: f64, s1: f64, s2: f64, n1: i32, n2: i32, two_tailed: bool) -> f64 {
	let (v1, v2) = ((n1 - 1) as f64, (n2 - 1) as f64);
	let (n1, n2) = (n1 as f64, n2 as f64);
	let t = (mean1 - mean2) / (s1.powi(2)/n1 + s2.powi(2)/n2).sqrt();
	let v = (s1.powi(2)/n1 + s2.powi(2)/n2).powi(2) / (s1.powi(4) / (n1.powi(2) * v1) + s2.powi(4) / (n2.powi(2) * v2));
	assert!(t.abs().powi(2) < v);
	t_cdf(t, v) * if two_tailed { 2.0 } else { 1.0 }
}
