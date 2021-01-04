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
#[derive(Debug, PartialEq)]
pub struct QuartileDescriptor {
	pub q1: f64,
	pub q2: f64,
	pub q3: f64,
	pub iqr: f64
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

// everything below is for implementing welch's t-test
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
fn gamma(x: f64) -> f64 {
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
/// Constant value for `ln(pi)`
const LN_PI: f64 = 1.1447298858494001741434273513530587116472948129153;
/// Constant value for `ln(2 * sqrt(e / pi))`
const LN_2_SQRT_E_OVER_PI: f64 = 0.6207822376352452223455184457816472122518527279025978;
// https://docs.rs/statrs/0.13.0/src/statrs/function/gamma.rs.html#33-57
/// Computes the logarithm of the gamma function
/// with an accuracy of 16 floating point digits.
/// The implementation is derived from
/// "An Analysis of the Lanczos Gamma Approximation",
/// Glendon Ralph Pugh, 2004 p. 116
fn ln_gamma(x: f64) -> f64 {
	if x < 0.5 {
		let s = GAMMA_DK
			.iter()
			.enumerate()
			.skip(1)
			.fold(GAMMA_DK[0], |s, t| s + t.1 / (t.0 as f64 - x));
		LN_PI
			- (f64::consts::PI * x).sin().ln()
			- s.ln()
			- LN_2_SQRT_E_OVER_PI
			- (0.5 - x) * ((0.5 - x + GAMMA_R) / f64::consts::E).ln()
	} else {
		let s = GAMMA_DK
			.iter()
			.enumerate()
			.skip(1)
			.fold(GAMMA_DK[0], |s, t| s + t.1 / (x + t.0 as f64 - 1.0));
		s.ln()
			+ LN_2_SQRT_E_OVER_PI
			+ (x - 0.5) * ((x - 0.5 + GAMMA_R) / f64::consts::E).ln()
	}
}

// helper for large values of n
// computes gamma(a) / gamma(b) with e^(lngamma(a) - lngamma(b))
fn large_gamma(a: f64, b: f64) -> f64 {
	f64::consts::E.powf(ln_gamma(a) - ln_gamma(b))
}

// beta functions implemented based off of mpmath
// https://github.com/fredrik-johansson/mpmath/blob/77c4c5e0ce37a2acca27bbf059e508bcb9579005/mpmath/functions/factorials.py#L5-L59
// BSD license
fn isnpint(x: f64) -> bool {
	return x <= 0.0 && x.round() == x;
}

fn gammaprod(a: &[f64], b: &[f64]) -> f64 {
	let mut poles_num = vec![];
	let mut poles_den = vec![];
	let mut regular_num = vec![];
	let mut regular_den = vec![];
	for x in a {
		if isnpint(*x) {
			poles_num.push(*x);
		} else {
			regular_num.push(*x);
		}
	}
	for x in b {
		if isnpint(*x) {
			poles_den.push(*x);
		} else {
			regular_den.push(*x);
		}
	}
	if poles_num.len() < poles_den.len() {
		return 0.0;
	}
	if poles_num.len() > poles_den.len() {
		return f64::INFINITY;
	}
	let mut p = 1.0;
	while poles_num.len() > 0 {
		let i = poles_num.pop().unwrap();
		let j = poles_den.pop().unwrap();
		p *= (-1.0f64).powf(i + j) * gamma(1.0 - j) / gamma(1.0 - i);
	}
	regular_num.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
	regular_den.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
	for i in 0..std::cmp::max(regular_den.len(), regular_num.len()) {
		if i < regular_num.len() && i < regular_den.len() {
			p *= large_gamma(regular_num[i], regular_den[i]);
		} else if i < regular_num.len() {
			p *= gamma(regular_num[i]);
		} else { // i < regular_den.len()
			p /= gamma(regular_den[i]);
		}
	}
	p
}

fn beta(x: f64, y: f64) -> f64 {
	gammaprod(&[x, y], &[x + y])
}

// a and b are the parameters of the beta function
// x1 is the lower bound of the integral, x2 is the upper bound
// we're looking for x1 = 0, x2 = x
fn betainc(a: f64, b: f64, x1: f64, x2: f64) -> f64 {
	let a = a;
	if x1 == x2 {
		0.0
	} else if x1 == 0.0 {
		// this is the only branch currently taken by the program
		if x1 == 0.0 && x2 == 1.0 {
			beta(a, b)
		} else {
			x2.powf(a) * hypergeometric2F1(a, 1.0 - b, a + 1.0, x2) / a
		}
	} else {
		let s1 = x2.powf(a) * hypergeometric2F1(a, 1.0 - b, a + 1.0, x2);
		let s2 = x1.powf(a) * hypergeometric2F1(a, 1.0 - b, a + 1.0, x1);
		(s1 - s2) / a
	}
}

fn betainc_regularized(a: f64, b: f64, x: f64) -> f64 {
	betainc(a, b, 0.0, x) / beta(a, b)
}

// computes part of the hypergeometric function while avoiding overflow from the rising factorials
// and factorials on both the top and bottom
//   (a)_n (b)_n	1
//   ----------- * ---
//      (c)_n	    n!
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
fn hypergeometric2F1(a: f64, b: f64, c: f64, z: f64) -> f64 {
	assert!(c >= 0.0);
	let mut sum = 0.0;
	let mut n = 0;
	loop {
		let v = pochhammer_factorial(a, b, c, n) * z.powi(n);
		sum += v;
		if v.abs() < 0.00001 {
			break;
		}
		if v >= 1e10 { // substantial loss of precision
			return f64::NAN;
		}
		if n >= 10_000 || v.is_infinite() || v.is_nan() {
			println!("{} {}", v, n);
			panic!("at the disco");
		}
		n += 1;
	}
	sum
}

fn t_cdf(x: f64, v: f64) -> f64 {
	// Attempt to use the simple formula when x^2 < v
	if x.powi(2) < v {
		let res = 0.5 + x * large_gamma((v + 1.0) / 2.0, v / 2.0) * hypergeometric2F1(0.5, (v + 1.0) / 2.0, 1.5, -x * x / v)
		/ (v * f64::consts::PI).sqrt();
		// round to 5 decimal places to deal with precision limits
		let res = (res * 10000.0).round() / 10000.0;
		// res returns NaN if there's an issue with 2F1 (i.e. precision overflow)
		// TODO: more sanity checks on the result?
		if !res.is_nan() {
			return res;
		}
	}
	// else: x^2 >= v or 2F1 in the simple formula wasn't successful
	let res = 1.0 - 0.5 * betainc_regularized(v / 2.0, 0.5, v / (x.powi(2) + v));
	let res = (res * 10000.0).round() / 10000.0;
	res
}

// Welch's t-test
pub fn two_sample_t_test(mean1: f64, mean2: f64, s1: f64, s2: f64, n1: usize, n2: usize, two_tailed: bool) -> f64 {
	let (v1, v2) = ((n1 - 1) as f64, (n2 - 1) as f64);
	let (n1, n2) = (n1 as f64, n2 as f64);
	let t = (mean1 - mean2).abs() / (s1.powi(2) / n1 + s2.powi(2) / n2).sqrt();
	if t == 0.0 {
		return 0.5 * if two_tailed { 2.0 } else { 1.0 }; // ?
	}
	let v = (s1.powi(2) / n1 + s2.powi(2) / n2).powi(2) / (s1.powi(4) / (n1.powi(2) * v1) + s2.powi(4) / (n2.powi(2) * v2));
	(1.0 - t_cdf(t, v)) * if two_tailed { 2.0 } else { 1.0 }
}

use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	static ref T_TABLE: HashMap<i32, (f64, f64, f64, f64, f64, f64)> = {
		let entries = [
			//   0      1      2      3       4       5
			//   50%    80%    90%    95%     98%     99%
			(1, (1.000, 3.078, 6.314, 12.706, 31.821, 63.657)),
			(2, (0.816, 1.886, 2.920, 4.303, 6.965, 9.925)),
			(3, (0.765, 1.638, 2.353, 3.182, 4.541, 5.841)),
			(4, (0.741, 1.533, 2.132, 2.776, 3.747, 4.604)),
			(5, (0.727, 1.476, 2.015, 2.571, 3.365, 4.032)),
			(6, (0.718, 1.440, 1.943, 2.447, 3.143, 3.707)),
			(7, (0.711, 1.415, 1.895, 2.365, 2.998, 3.499)),
			(8, (0.706, 1.397, 1.860, 2.306, 2.896, 3.355)),
			(9, (0.703, 1.383, 1.833, 2.262, 2.821, 3.250)),
			(10, (0.700, 1.372, 1.812, 2.228, 2.764, 3.169)),
			(11, (0.697, 1.363, 1.796, 2.201, 2.718, 3.106)),
			(12, (0.695, 1.356, 1.782, 2.179, 2.681, 3.055)),
			(13, (0.694, 1.350, 1.771, 2.160, 2.650, 3.012)),
			(14, (0.692, 1.345, 1.761, 2.145, 2.624, 2.977)),
			(15, (0.691, 1.341, 1.753, 2.131, 2.602, 2.947)),
			(16, (0.690, 1.337, 1.746, 2.120, 2.583, 2.921)),
			(17, (0.689, 1.333, 1.740, 2.110, 2.567, 2.898)),
			(18, (0.688, 1.330, 1.734, 2.101, 2.552, 2.878)),
			(19, (0.688, 1.328, 1.729, 2.093, 2.539, 2.861)),
			(20, (0.687, 1.325, 1.725, 2.086, 2.528, 2.845)),
			(21, (0.686, 1.323, 1.721, 2.080, 2.518, 2.831)),
			(22, (0.686, 1.321, 1.717, 2.074, 2.508, 2.819)),
			(23, (0.685, 1.319, 1.714, 2.069, 2.500, 2.807)),
			(24, (0.685, 1.318, 1.711, 2.064, 2.492, 2.797)),
			(25, (0.684, 1.316, 1.708, 2.060, 2.485, 2.787)),
			(26, (0.684, 1.315, 1.706, 2.056, 2.479, 2.779)),
			(27, (0.684, 1.314, 1.703, 2.052, 2.473, 2.771)),
			(28, (0.683, 1.313, 1.701, 2.048, 2.467, 2.763)),
			(29, (0.683, 1.311, 1.699, 2.045, 2.462, 2.756)),
			(30, (0.683, 1.310, 1.697, 2.042, 2.457, 2.750)),
			(40, (0.681, 1.303, 1.684, 2.021, 2.423, 2.704)),
			(50, (0.679, 1.299, 1.676, 2.009, 2.403, 2.678)),
			(60, (0.679, 1.296, 1.671, 2.000, 2.390, 2.660)),
			(70, (0.678, 1.294, 1.667, 1.994, 2.381, 2.648)),
			(80, (0.678, 1.292, 1.664, 1.990, 2.374, 2.639)),
			(90, (0.677, 1.291, 1.662, 1.987, 2.368, 2.632)),
			(100, (0.677, 1.290, 1.660, 1.984, 2.364, 2.626)),
			(0, (0.674, 1.282, 1.645, 1.960, 2.326, 2.576))
		];
		let mut table = HashMap::new();
		for entry in entries.iter() {
			table.insert(entry.0, entry.1);
		}
		table
	};
}

// returns an estimate for a confidence interval critical t value
// right it's hard-coded for a 98% confidence interval (because tuple)
// estimation should be such that the returned value yields at least a 98% interval
// TODO: use regression or method other than a lookup table?
pub fn t_lookup(df: i32) -> f64 {
	assert!(df >= 1);
	if df <= 30 {
		T_TABLE.get(&df).unwrap().4
	} else if df <= 100 {
		let df = ((df + 9) / 10) * 10; // round df up to the nearest power of 10
		T_TABLE.get(&df).unwrap().4
	} else {
		T_TABLE.get(&0).unwrap().4
	}
}
