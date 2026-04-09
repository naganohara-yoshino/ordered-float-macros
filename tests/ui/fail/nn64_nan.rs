use ordered_float_macros::nn64;

fn main() {
    let _ = nn64!(f64::INFINITY + f64::NEG_INFINITY);
}
