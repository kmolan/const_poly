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

## Examples

### Simple Sine Polynomial (1 variable, 1 term):
```rust
use const_poly::{const_poly, Polynomial};
use const_poly::VarFunction::Sin;

const POLY_1V_1T: Polynomial<1, 1> = const_poly!({
    [2.0, [Sin]]  // 2 * sin(x)
});

const VARS: [f64; 1] = [1.2]; // sin(1.2) ≈ 0.932039
const RESULT: f64 = POLY_1V_1T.evaluate(VARS); // ≈ 1.864078
``` 

### Mixed Trigonometric Polynomial (2 Variables, 2 terms):
```rust
use const_poly::{const_poly, Polynomial};
use const_poly::VarFunction::{Cos, Sin};

const POLY_2V_2T: Polynomial<2, 2> = const_poly!({
    [1.0, [Sin, Cos]],  // sin(x) * cos(y)
    [0.75, [Cos, Sin]]  // 0.75 * cos(x) * sin(y)
});

const VARS: [f64; 2] = [0.9, 1.1];
const RESULT: f64 = POLY_2V_2T.evaluate(VARS);  // ~ 0.7705881
``` 

### Powers with Mixed Exponents (3 Variables, 5 terms):
```rust
use const_poly::{const_poly, Polynomial};
use const_poly::VarFunction::Pow;

const POLY_3V_5T: Polynomial<3, 5> = const_poly!({
        [1.2, [Pow(2), Pow(-1), Pow(0)]],  // 1.2 * x² * y⁻¹ * z⁰
        [-0.8, [Pow(3), Pow(1), Pow(-2)]], // -0.8 * x³ * y¹ * z⁻²
        [2.5, [Pow(-3), Pow(4), Pow(1)]],  // 2.5 * x⁻³ * y⁴ * z¹
        [-1.1, [Pow(0), Pow(-2), Pow(3)]], // -1.1 * x⁰ * y⁻² * z³
        [0.9, [Pow(1), Pow(2), Pow(-1)]]   // 0.9 * x¹ * y² * z⁻¹
    });

const VARS: [f64; 3] = [2.0, 3.0, 0.5];
const RESULT: f64 = POLY_3V_5T.evaluate(VARS); //~ -30.159027778
```

### Powers with mixed exponents, trignometric and log functions (4 variables, 4 terms):
```rust
use const_poly::{const_poly, Polynomial};
use const_poly::VarFunction::*;

const POLY_4V_4T: Polynomial<4, 4> = const_poly!({
        [3.0, [Identity, Sin, Pow(2), Cos]],  // 3.0 * x₀ * sin(x₁) * x₂² * cos(x₃)
        [-1.2, [Pow(3), Tan, Exp, Identity]], // -1.2 * x₀³ * tan(x₁) * exp(x₂) * x₃
        [0.7, [Ln, Sqrt, Arctan, Sinh]],      // 0.7 * ln(x₀) * sqrt(x₁) * atan(x₂) * sinh(x₃)
        [1.1, [Cosh, Identity, Pow(1), Sin]]  // 1.1 * cosh(x₀) * x₁ * x₂ * sin(x₃)
    });

const VARS = [0.5, 1.5, 2.5, 3.5];
const RESULT = POLY_4V_4T.evaluate(VARS); // ~ -112.280027300776
```

## Benchmarks
See [BENCHMARKS.md](./BENCHMARKS.md)

## Contributions Guide
See [CONTRIBUTIONS.md](./CONTRIBUTIONS.md)

## LICENSE
const_poly is licensed under the MIT license.

## Contact
anmolkathail@gmail.com

If you use this library in your project, a shoutout or mention would be awesome!

## TODO
-  Add polynomial operations like add/subtract/multiply.

-  Add more benchmarking.

