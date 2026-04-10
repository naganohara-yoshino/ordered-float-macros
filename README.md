# ordered-float-macros

Declarative macros for constructing `ordered_float::NotNan` and `ordered_float::OrderedFloat`.

The crate focuses on two small jobs:

- `nn32!` and `nn64!` build `NotNan<f32>` and `NotNan<f64>` from const-evaluable expressions and reject `NAN` during compilation.
- `of32!` and `of64!` wrap ordinary float expressions in `OrderedFloat`, including runtime values.

## Installation

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
ordered-float = "5.3"
ordered-float-macros = "0.6"
```

`ordered-float` must be a **direct dependency** of the calling crate. These
proc-macros intentionally expand to `::ordered_float::...`, so depending only on
`ordered-float-macros` is not enough.

## Usage

```rust
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
```

## Guarantees And Limits

- `nn32!` and `nn64!` only accept expressions that can be evaluated in a `const` context.
- If such an expression evaluates to `NAN`, compilation fails with a macro-specific error message.
- `of32!` and `of64!` do not perform validation; they simply wrap the value in `OrderedFloat`.

`nn32!` rejects `NAN`:

```compile_fail
use ordered_float_macros::nn32;

fn main() {
    let _ = nn32!(f32::NAN);
}
```

`nn64!` also rejects non-const expressions:

```compile_fail
use ordered_float_macros::nn64;

fn main() {
    let value = 1.0_f64;
    let _ = nn64!(value);
}
```
