[![On crates.io](https://img.shields.io/crates/v/const_poly.svg)](https://crates.io/crates/const_poly)
![Downloads](https://img.shields.io/crates/d/const_poly?style=flat-square)
![github](https://github.com/kmolan/const_poly/actions/workflows/build-tests.yml/badge.svg)
![github](https://github.com/kmolan/const_poly/actions/workflows/code-coverage.yml/badge.svg)

Evaluate any multivariable equation or polynomial at compile time with high accuracy and zero runtime overhead.

## Features

- `no_std` compatible – no heap allocations, no panics.
- Full compile-time evaluation of arbitrarily complex equations with high numerical accuracy (benchmarked at 1e-7).
- Fully documented with code examples, user-friendly macros, benchmarking, and a comprehensive suite of tests. 
- Define expressions using variety of mathematical functions, all evaluable at compile time:
     - `Identity`: the variable itself (no function applied)
    - `Pow(u32)`: power function `x^n` where `n` is the exponent
    - `Sin`: sine function
    - `Cos`: cosine function
    - `Tan`: tangent function
    - `Exp`: exponential function (e^x)
    - `Ln`: natural logarithm
    - `Sqrt`: square root function
    - `Arctan`: arctangent function
    - `Sinh`: hyperbolic sine function
    - `Cosh`: hyperbolic cosine function

## Who is this for?
 - This library is primarily meant to empower scientific computing and mathematical libraries in rust to perform all numerical approximations entirely at compile time. 

 - Embedded and no_std environments where heapless, panic-free code is essential.

 - Metaprogramming and symbolic math tools that benefit from evaluating complex expressions entirely at compile time.

## Installation

Add the crate to your project:

```bash
cargo add const_poly
```

or manually in your `Cargo.toml`:

```toml
[dependencies]
const_poly = "0.0.3"
```

## Quick Start Example

```rust
use const_poly::VarFunction::*;
use const_poly::{Polynomial, const_poly};

// Define f(x,y) = 2.5⋅x²y³
const POLY = Polynomial<2, 1> = const_poly!([2.5, Pow(2), Pow(3)]);

// Evaluate f(x,y) at (x,y)=(10.0, -5.0)
const RESULT: f64 = POLY_1.evaluate([10.0, -5.0]); // -31250

// Multi-term polynomial g(x,y,z)
const POLY_3V_5T: Polynomial<3, 5> = const_poly!({
    [1.2, Pow(2), Pow(-1), Pow(0)],  // 1.2 * x² * y⁻¹ * z⁰
    [-0.8, Pow(3), Pow(1), Pow(-2)], // -0.8 * x³ * y¹ * z⁻²
    [2.5, Pow(-3), Pow(4), Pow(1)],  // 2.5 * x⁻³ * y⁴ * z¹
    [-1.1, Pow(0), Pow(-2), Pow(3)], // -1.1 * x⁰ * y⁻² * z³
    [0.9, Pow(1), Pow(2), Pow(-1)]   // 0.9 * x¹ * y² * z⁻¹
});

const VARS: [f64; 3] = [2.0, 3.0, 0.5]; // (x,y,z)=(2.0,3.0,0.5)
const RES: f64 = POLY_3V_5T.evaluate(VARS); // -30.159027778
```
## Tutorials
Follow the full tutorial at [TUTORIAL.md](https://github.com/kmolan/const_poly/blob/main/TUTORIAL.md)

## Benchmarks
See [BENCHMARKS.md](https://github.com/kmolan/const_poly/blob/main/BENCHMARKS.md)

## Contributions Guide
See [CONTRIBUTIONS.md](https://github.com/kmolan/const_poly/blob/main/CONTRIBUTIONS.md)

## LICENSE
const_poly is licensed under the MIT license.

## Contact
anmolkathail@gmail.com

If you use this library in your project, a shoutout or mention would be awesome!

## TODO
-  Add polynomial operations like add/subtract/multiply.
-  String representation for the polynomial.
-  Add more benchmarking.

