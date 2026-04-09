use ordered_float::{NotNan, OrderedFloat};
use ordered_float_macros::{nn32, nn64, of32, of64};

#[test]
fn nn_macros_build_not_nan_values() {
    const LEFT: f32 = 1.25;
    const RIGHT: f64 = 2.5;

    let value32: NotNan<f32> = nn32!(LEFT + 0.75);
    let value64: NotNan<f64> = nn64!(RIGHT * 2.0);

    assert_eq!(value32, NotNan::new(2.0).unwrap());
    assert_eq!(value64, NotNan::new(5.0).unwrap());
}

#[test]
fn ordered_float_macros_build_values() {
    let left: f32 = 3.5;
    let right: f64 = 8.0;

    let value32: OrderedFloat<f32> = of32!(left - 0.5);
    let value64: OrderedFloat<f64> = of64!(right / 2.0);

    assert_eq!(value32, OrderedFloat(3.0));
    assert_eq!(value64, OrderedFloat(4.0));
}
