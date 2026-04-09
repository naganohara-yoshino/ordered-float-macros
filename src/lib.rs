#![doc = include_str!("../README.md")]

use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, parse_macro_input};

/// Builds an `ordered_float::NotNan<f32>` from a const-evaluable expression.
#[proc_macro]
pub fn nn32(input: TokenStream) -> TokenStream {
    expand_not_nan(input, FloatTy::F32)
}

/// Builds an `ordered_float::NotNan<f64>` from a const-evaluable expression.
#[proc_macro]
pub fn nn64(input: TokenStream) -> TokenStream {
    expand_not_nan(input, FloatTy::F64)
}

/// Builds an `ordered_float::OrderedFloat<f32>` from an expression.
#[proc_macro]
pub fn of32(input: TokenStream) -> TokenStream {
    expand_ordered_float(input, FloatTy::F32)
}

/// Builds an `ordered_float::OrderedFloat<f64>` from an expression.
#[proc_macro]
pub fn of64(input: TokenStream) -> TokenStream {
    expand_ordered_float(input, FloatTy::F64)
}

#[derive(Clone, Copy)]
enum FloatTy {
    F32,
    F64,
}

fn expand_not_nan(input: TokenStream, ty: FloatTy) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let ty_token = match ty {
        FloatTy::F32 => quote!(f32),
        FloatTy::F64 => quote!(f64),
    };

    let macro_name = match ty {
        FloatTy::F32 => "nn32!",
        FloatTy::F64 => "nn64!",
    };

    let expanded = quote! {{
        const __ORDERED_FLOAT_MACROS_VALUE: #ty_token = #expr;
        const _: () = {
            // This assertion runs at compile-time because it's in a const context.
            assert!(!__ORDERED_FLOAT_MACROS_VALUE.is_nan(), concat!(#macro_name, ": expression evaluated to NAN"));
        };
        // SAFETY: The const assertion above ensures the value is not NaN.
        // If the expression is not const-evaluable, the compiler will error
        // at the __ORDERED_FLOAT_MACROS_VALUE definition.
        unsafe {
            ::ordered_float::NotNan::<#ty_token>::new_unchecked(__ORDERED_FLOAT_MACROS_VALUE)
        }
    }};

    expanded.into()
}

fn expand_ordered_float(input: TokenStream, ty: FloatTy) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);

    let ty_token = match ty {
        FloatTy::F32 => quote!(f32),
        FloatTy::F64 => quote!(f64),
    };

    let expanded = quote! {
        ::ordered_float::OrderedFloat::<#ty_token>(#expr)
    };

    expanded.into()
}
