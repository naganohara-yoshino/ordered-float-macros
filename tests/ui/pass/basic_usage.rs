use ordered_float::{NotNan, OrderedFloat};
use ordered_float_macros::{nn32, nn64, of32, of64};

const SCALE32: f32 = 1.5;
const SCALE64: f64 = 3.0;

fn main() {
    let runtime32 = 1.0_f32;
    let runtime64 = 0.5_f64;

    let a: NotNan<f32> = nn32!(SCALE32 + 0.5);
    let b: NotNan<f64> = nn64!(SCALE64 * 2.0);
    let c: OrderedFloat<f32> = of32!(runtime32 + SCALE32 - 0.25);
    let d: OrderedFloat<f64> = of64!(runtime64 + SCALE64 / 3.0);

    assert_eq!(a.into_inner(), 2.0);
    assert_eq!(b.into_inner(), 6.0);
    assert_eq!(c.into_inner(), 2.25);
    assert_eq!(d.into_inner(), 1.5);
}
