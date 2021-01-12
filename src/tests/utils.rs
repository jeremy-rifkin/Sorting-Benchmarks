use crate::utils;

#[test]
fn test_commafy() {
	assert_eq!(utils::commafy(123), "123");
	assert_eq!(utils::commafy(123456), "123,456");
	assert_eq!(utils::commafy(123456789), "123,456,789");
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
