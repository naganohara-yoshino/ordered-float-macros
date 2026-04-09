# ordered-float-macros

Procedural macros for constructing `ordered_float::NotNan` and
`ordered_float::OrderedFloat`.

## Installation

Add both crates to your `Cargo.toml`:

```toml
[dependencies]
ordered-float = "5.3"
ordered-float-macros = "0.3"
```

`ordered-float` must be a direct dependency of the calling crate. These
proc-macros intentionally expand to `::ordered_float::...`, so depending only on
`ordered-float-macros` is not enough.

## Usage

`nn32!` and `nn64!` require a const-evaluable expression and reject `NAN` at
compile time.

`of32!` and `of64!` wrap any ordinary expression of the matching float type, so
they can be used with runtime values as well.

```rust
use ordered_float::{NotNan, OrderedFloat};
use ordered_float_macros::{nn32, nn64, of32, of64};

const SCALE: f64 = 2.0;

fn main() {
    let runtime = 1.5_f32;

    let a: NotNan<f32> = nn32!(1.25 + 0.75);
    let b: NotNan<f64> = nn64!(SCALE * 2.5);
    let c: OrderedFloat<f32> = of32!(runtime + 2.0);
    let d: OrderedFloat<f64> = of64!(SCALE + 1.0);

    assert_eq!(a.into_inner(), 2.0);
    assert_eq!(b.into_inner(), 5.0);
    assert_eq!(c.into_inner(), 3.5);
    assert_eq!(d.into_inner(), 3.0);
}
```
