use sorting_benchmarks::statistics;

fn mean(array: &[u64]) -> f64 {
	array.iter().sum::<u64>() as f64 / array.len() as f64
}

fn is_within_error(value: f64, expected: f64, margin: f64) -> bool {
	(value - expected).abs() <= margin
}

macro_rules! asserte {
	($a:expr, $b:expr) => {
		let a = $a;
		let b = $b;
		assert!(is_within_error(a, b, 0.005), "{} vs. {}", a, b);
	};
}

#[test]
fn test_mean() {
	let array = [6, 2, 3, 1];
	assert_eq!(mean(&array), 3.0);
}

#[test]
fn test_stdev() {
	let array = [6, 2, 3, 1];
	asserte!(statistics::stdev(&array, mean(&array)), 2.16);
	let array = [2, 2, 5, 7];
	asserte!(statistics::stdev(&array, mean(&array)), 2.45);
	let array = [2, 4, 4, 4, 5, 5, 7, 9];
	asserte!(statistics::stdev(&array, mean(&array)), 2.14);
}

#[test]
fn test_quartiles() {
	let array = vec![1, 2, 5, 6, 7, 9, 12, 15, 18, 19, 27];
	assert_eq!(statistics::quartiles(&array), statistics::QuartileDescriptor {
		q1: 5.0,
		q2: 9.0,
		q3: 18.0,
		iqr: 13.0
	});
	let array = vec![3, 5, 7, 8, 9, 11, 15, 16, 20, 21];
	assert_eq!(statistics::quartiles(&array), statistics::QuartileDescriptor {
		q1: 7.0,
		q2: 10.0,
		q3: 16.0,
		iqr: 9.0
	});
}

#[test]
fn test_tukey() {
	let q = statistics::QuartileDescriptor {
		q1: -2.0,
		q2: 0.0,
		q3: 2.0,
		iqr: 4.0
	};
	assert!(statistics::tukey(3, &q, 3.0));
	assert!(!statistics::tukey(15, &q, 3.0));
}

#[test]
fn test_two_sample_t_test() {
	asserte!(statistics::two_sample_t_test(10.0, 11.0, 0.5, 0.5, 5, 4, true), 0.022);
	asserte!(statistics::two_sample_t_test(10.0, 12.0, 4.0, 3.0, 5, 4, true), 0.42);
	asserte!(statistics::two_sample_t_test(10.0, 10.0, 0.5, 0.5, 5, 4, true), 1.0);
	let a = (2962.4365482233502, 121.26102846652408, 197);
	let b = (1323.7373737373737, 110.33944725932848, 198);
	assert_eq!(statistics::two_sample_t_test(a.0, b.0, a.1, b.1, a.2, b.2, true), 0.0);
	let a = (1036.6834170854272, 84.751897427476393, 199);
	let b = (978.28282828282829, 85.395947359712380, 198);
	assert_eq!(statistics::two_sample_t_test(a.0, b.0, a.1, b.1, a.2, b.2, true), 0.0);
	let a = (1018.0, 121.86160698218863, 200);
	let b = (904.52261306532660, 114.28145408898557, 199);
	assert_eq!(statistics::two_sample_t_test(a.0, b.0, a.1, b.1, a.2, b.2, true), 0.0);
}

#[test]
fn test_t_lookup() {
	assert_eq!(statistics::t_lookup(4), 3.747);
	assert_eq!(statistics::t_lookup(12), 2.681);
	assert_eq!(statistics::t_lookup(35), 2.423);
	assert_eq!(statistics::t_lookup(72), 2.374);
	assert_eq!(statistics::t_lookup(124124), 2.326);
}
