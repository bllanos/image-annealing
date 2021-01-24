use super::WorkgroupDimensions;
use std::primitive::u32;

#[test]
#[should_panic(expected = "Integer overflow")]
fn large_width() {
    WorkgroupDimensions::new(u32::MAX - 1, 1);
}

#[test]
#[should_panic(expected = "Integer overflow")]
fn large_height() {
    WorkgroupDimensions::new(1, u32::MAX - 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn zero_width() {
    WorkgroupDimensions::new(0, 1);
}

#[test]
#[should_panic(expected = "assertion failed")]
fn zero_height() {
    WorkgroupDimensions::new(1, 0);
}

#[test]
fn no_remainder() {
    let dim = WorkgroupDimensions::new(64, 64);
    assert_eq!(dim.0, 64);
    assert_eq!(dim.1, 64);
    assert_eq!(dim.2, 1);
}

#[test]
fn remainder() {
    let dim = WorkgroupDimensions::new(63, 62);
    assert_eq!(dim.0, 64);
    assert_eq!(dim.1, 64);
    assert_eq!(dim.2, 1);
}
