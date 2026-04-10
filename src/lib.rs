#![doc = include_str!("../README.md")]

/// Constructs a `ordered_float::NotNan<f32>` from a const-evaluable `f32` expression.
///
/// The expression is evaluated in a `const` context, so ordinary runtime values are rejected.
/// If the expression evaluates to `NAN`, compilation fails.
///
/// # Examples
///
/// ```rust
/// use ordered_float::NotNan;
/// use ordered_float_macros::nn32;
///
/// const SCALE: f32 = 1.5;
///
/// let value: NotNan<f32> = nn32!(SCALE + 0.5);
/// assert_eq!(value.into_inner(), 2.0);
/// ```
/// Examples of invalid usage are rejected at compile time:
///
/// ```compile_fail
/// use ordered_float_macros::nn32;
///
/// let runtime = 1.0_f32;
/// let _ = nn32!(runtime);
/// ```
///
/// ```compile_fail
/// use ordered_float_macros::nn32;
///
/// let _ = nn32!(f32::NAN);
/// ```
#[macro_export]
macro_rules! nn32 {
    ($expr:expr) => {{
        const __SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS: f32 = $expr;
        const _: () = {
            assert!(
                !__SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS.is_nan(),
                "nn32!: expression evaluated to NAN"
            );
        };
        const {
            unsafe {
                ::ordered_float::NotNan::<f32>::new_unchecked(
                    __SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS,
                )
            }
        }
    }};
}

/// Constructs a `ordered_float::NotNan<f64>` from a const-evaluable `f64` expression.
///
/// The expression is evaluated in a `const` context, so ordinary runtime values are rejected.
/// If the expression evaluates to `NAN`, compilation fails.
///
/// # Examples
///
/// ```rust
/// use ordered_float::NotNan;
/// use ordered_float_macros::nn64;
///
/// const SCALE: f64 = 2.5;
///
/// let value: NotNan<f64> = nn64!(SCALE * 2.0);
/// assert_eq!(value.into_inner(), 5.0);
/// ```
/// Examples of invalid usage are rejected at compile time:
///
/// ```compile_fail
/// use ordered_float_macros::nn64;
///
/// let runtime = 2.0_f64;
/// let _ = nn64!(runtime);
/// ```
///
/// ```compile_fail
/// use ordered_float_macros::nn64;
///
/// let _ = nn64!(f64::INFINITY + f64::NEG_INFINITY);
/// ```
#[macro_export]
macro_rules! nn64 {
    ($expr:expr) => {{
        const __SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS: f64 = $expr;
        const _: () = {
            assert!(
                !__SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS.is_nan(),
                "nn64!: expression evaluated to NAN"
            );
        };
        const {
            unsafe {
                ::ordered_float::NotNan::<f64>::new_unchecked(
                    __SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS,
                )
            }
        }
    }};
}

/// Wraps an `f32` expression in `ordered_float::OrderedFloat<f32>`.
///
/// Unlike [`nn32!`], this macro accepts ordinary runtime expressions and performs no validation.
///
/// # Examples
///
/// ```rust
/// use ordered_float::OrderedFloat;
/// use ordered_float_macros::of32;
///
/// let runtime = 3.5_f32;
/// let value: OrderedFloat<f32> = of32!(runtime - 0.5);
///
/// assert_eq!(value, OrderedFloat(3.0));
/// ```
#[macro_export]
macro_rules! of32 {
    ($expr:expr) => {
        ::ordered_float::OrderedFloat::<f32>($expr)
    };
}

/// Wraps an `f64` expression in `ordered_float::OrderedFloat<f64>`.
///
/// Unlike [`nn64!`], this macro accepts ordinary runtime expressions and performs no validation.
///
/// # Examples
///
/// ```rust
/// use ordered_float::OrderedFloat;
/// use ordered_float_macros::of64;
///
/// let runtime = 8.0_f64;
/// let value: OrderedFloat<f64> = of64!(runtime / 2.0);
///
/// assert_eq!(value, OrderedFloat(4.0));
/// ```
#[macro_export]
macro_rules! of64 {
    ($expr:expr) => {
        ::ordered_float::OrderedFloat::<f64>($expr)
    };
}
