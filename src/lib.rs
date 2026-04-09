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
        unsafe {
            ::ordered_float::NotNan::<f32>::new_unchecked(__SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS)
        }
    }};
}

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
        unsafe {
            ::ordered_float::NotNan::<f64>::new_unchecked(__SHOULD_BE_CONST_IN_ORDERED_FLOAT_MACROS)
        }
    }};
}

#[macro_export]
macro_rules! of32 {
    ($expr:expr) => {
        ::ordered_float::OrderedFloat::<f32>($expr)
    };
}

#[macro_export]
macro_rules! of64 {
    ($expr:expr) => {
        ::ordered_float::OrderedFloat::<f64>($expr)
    };
}
