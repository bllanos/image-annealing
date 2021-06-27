use super::WorkgroupGridDimensions;
use wgpu::Extent3d;

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn zero_width() {
    WorkgroupGridDimensions::from(Extent3d {
        width: 0,
        height: 1,
        depth_or_array_layers: 1,
    });
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn zero_height() {
    WorkgroupGridDimensions::from(Extent3d {
        width: 1,
        height: 0,
        depth_or_array_layers: 1,
    });
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn zero_depth() {
    WorkgroupGridDimensions::from(Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 0,
    });
}

#[test]
fn no_remainder() {
    let dim = WorkgroupGridDimensions::from(Extent3d {
        width: 64,
        height: 64,
        depth_or_array_layers: 1,
    });
    assert_eq!(dim.x(), 2);
    assert_eq!(dim.y(), 2);
    assert_eq!(dim.z(), 1);
}

#[test]
fn remainder() {
    let dim = WorkgroupGridDimensions::from(Extent3d {
        width: 63,
        height: 65,
        depth_or_array_layers: 1,
    });
    assert_eq!(dim.x(), 2);
    assert_eq!(dim.y(), 3);
    assert_eq!(dim.z(), 1);
}
