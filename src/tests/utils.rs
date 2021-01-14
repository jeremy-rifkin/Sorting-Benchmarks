use std::time::Duration;

use crate::utils;

#[test]
fn test_commafy() {
	assert_eq!(utils::commafy(123), "123");
	assert_eq!(utils::commafy(123456), "123,456");
	assert_eq!(utils::commafy(123456789), "123,456,789");
}

#[test]
fn test_duration_to_human() {
	assert_eq!(utils::duration_to_human(Duration::from_nanos(500)), "500.00ns");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(1_500)), "__1.50μs");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(1_500_000)), "__1.50ms");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(1_500_000_000)), "_1.50s");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(30 * 1_000_000_000)), "30.00s");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(90 * 1_000_000_000)), "_1m 30.00s");
	assert_eq!(utils::duration_to_human(Duration::from_nanos(95 * 60 * 1_000_000_000 + 500_000_000)), "1h 35m _0.50s");
}

#[test]
fn test_verify_sorted() {
	utils::verify_sorted(&[1,2,3,4,5]);
	utils::verify_sorted(&[1,1,1,1,1]);
}

#[test]
#[should_panic]
fn test_verify_unsorted() {
	utils::verify_sorted(&[1,1,1,1,0]);
}

#[test]
fn test_fmin() {
	assert_eq!(utils::fmin(1.0, 2.0), 1.0);
	assert_eq!(utils::fmin(2.0, 1.0), 1.0);
}

#[test]
fn test_fmax() {
	assert_eq!(utils::fmax(1.0, 2.0), 2.0);
	assert_eq!(utils::fmax(2.0, 1.0), 2.0);
}
