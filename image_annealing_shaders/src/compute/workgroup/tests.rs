use super::WorkgroupDimensions;
use std::num::NonZeroU32;

#[test]
fn texture_patch() {
    let workgroup_dimensions = WorkgroupDimensions::texture_patch();
    assert_eq!(workgroup_dimensions.x(), workgroup_dimensions.y());
    assert_eq!(workgroup_dimensions.z(), 1u32);
}

#[test]
fn horizontal_line() {
    let workgroup_dimensions = WorkgroupDimensions::horizontal_line(24u32);
    assert_eq!(workgroup_dimensions.x(), 24u32);
    assert_eq!(workgroup_dimensions.y(), 1u32);
    assert_eq!(workgroup_dimensions.z(), 1u32);
}

#[test]
fn invocation_count() {
    let workgroup_dimensions = WorkgroupDimensions(
        NonZeroU32::new(2).unwrap(),
        NonZeroU32::new(3).unwrap(),
        NonZeroU32::new(5).unwrap(),
    );
    assert_eq!(workgroup_dimensions.x(), 2u32);
    assert_eq!(workgroup_dimensions.y(), 3u32);
    assert_eq!(workgroup_dimensions.z(), 5u32);
    assert_eq!(workgroup_dimensions.invocation_count(), 30u32);
}
