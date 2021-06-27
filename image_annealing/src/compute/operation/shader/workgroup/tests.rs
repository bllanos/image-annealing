use super::WorkgroupGridDimensions;
use wgpu::Extent3d;

#[test]
#[should_panic(expected = "assertion failed")]
fn zero_width() {
    WorkgroupGridDimensions::new(Extent3d {
        width: 0,
        height: 1,
        depth_or_array_layers: 1,
    });
}

#[test]
#[should_panic(expected = "assertion failed")]
fn zero_height() {
    WorkgroupGridDimensions::new(Extent3d {
        width: 1,
        height: 0,
        depth_or_array_layers: 1,
    });
}

#[test]
#[should_panic(expected = "assertion failed")]
fn zero_depth() {
    WorkgroupGridDimensions::new(Extent3d {
        width: 1,
        height: 1,
        depth_or_array_layers: 0,
    });
}

#[test]
fn no_remainder() {
    let dim = WorkgroupGridDimensions::new(Extent3d {
        width: 64,
        height: 64,
        depth_or_array_layers: 1,
    });
    assert_eq!(dim.0, 2);
    assert_eq!(dim.1, 2);
    assert_eq!(dim.2, 1);
}

#[test]
fn remainder() {
    let dim = WorkgroupGridDimensions::new(Extent3d {
        width: 63,
        height: 65,
        depth_or_array_layers: 1,
    });
    assert_eq!(dim.0, 2);
    assert_eq!(dim.1, 3);
    assert_eq!(dim.2, 1);
}
