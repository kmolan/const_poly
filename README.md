[![On crates.io](https://img.shields.io/crates/v/const_poly.svg)](https://crates.io/crates/const_poly)
![Downloads](https://img.shields.io/crates/d/const_poly?style=flat-square)
![github](https://github.com/kmolan/const_poly/actions/workflows/build-tests.yml/badge.svg)
![github](https://github.com/kmolan/const_poly/actions/workflows/code-coverage.yml/badge.svg)

- Define and evaluate any multivariable polynomial or equation at compile time.
- Handle any number of variables and function types (Pow, Sin, Exp, Ln, etc.) with a single abstraction that is always const-safe.
- Zero Dependencies and `no_std`, ensuring no heap allocations, no panics, and no external crates.
- Helps reduce boilerplate so you don't have to keep defining custom `const fn` for each equation.

## Introduction
`const_poly` is a lightweight abstraction for evaluating multi-variable polynomials entirely at compile time. Instead of writing specialized `const fn` evaluators for every equation you need, `const_poly` abstracts all of this away using generic, compile-time constructs. The implementation can handle any number of variables, and is capable of evaluating any complex equation with high accuracy. This approach reduces boilerplate, so you can focus on the actual algorithms while writing math expressions in a natural, equation-like syntax:

```rust
use const_poly::VarFunction::*;
use const_poly::{Polynomial, const_poly};

//define f(x,y) = 2.5 * x² * y³
const POLY = Polynomial<2, 1> = const_poly!([2.5, Pow(2), Pow(3)]);

//evaluate f(x,y) at (x,y) = (10.0, -5.0)
const RESULT: f64 = POLY_1.evaluate([10.0, -5.0]); // -31250.0

// Multi-term polynomial g(x,y,z)
const POLY_3V_5T: Polynomial<3, 5> = const_poly!({
    [1.2, Pow(2), Pow(-1), Pow(0)],  // 1.2 * x² * y⁻¹ * z⁰
    [-0.8, Pow(3), Pow(1), Pow(-2)], // -0.8 * x³ * y¹ * z⁻²
    [2.5, Pow(-3), Pow(4), Pow(1)],  // 2.5 * x⁻³ * y⁴ * z¹
    [-1.1, Pow(0), Pow(-2), Pow(3)], // -1.1 * x⁰ * y⁻² * z³
    [0.9, Pow(1), Pow(2), Pow(-1)]   // 0.9 * x¹ * y² * z⁻¹
});

// (x,y,z) = (2.0,3.0,0.5)
const VARS: [f64; 3] = [2.0, 3.0, 0.5]; 
const RES: f64 = POLY_3V_5T.evaluate(VARS); // -30.159027778
```

Every polynomial defined with `const_poly` is a fully constant object, meaning it can be safely passed, composed, or evaluated anywhere in the codebase in a const context. You can freely pass this object to other `const fn`, or embed it inside larger data structures. `const_poly` has zero dependencies and is written completely in a `no_std` environment.

## Who is this for?
 - This library is primarily meant to empower scientific computing and mathematical libraries in rust to perform all numerical approximations entirely at compile time or in const contexts. 

 - Embedded and no_std environments where heapless, panic-free code is essential.

 - Metaprogramming and symbolic math tools that benefit from evaluating complex expressions entirely at compile time.

## More code examples

### 1. Simple polynomial
```rust
//define f(x) = 3 * x²
const POLY: Polynomial<1, 1> = const_poly!([3.0, Pow(2)]);

const RESULT: f64 = POLY.evaluate([4.0]); // 3 * (4^2) = 48.0
``` 

### 2. Trigonometric Functions
```rust
// define f(x, y) = 2.0 * Sin(x) * Cos(y)
const POLY: Polynomial<2, 1> = const_poly!([2.0, Sin, Cos]);
const RESULT: f64 = POLY.evaluate([1.57079632679, 0.0]); // 2.0 * sin(π/2) * cos(0) = 2.0
```

### 3. Multi-Term Mixed Polynomial
```rust
const POLY: Polynomial<2, 3> = const_poly!({
    [1.0, Pow(2), Pow(1)],   // 1.0 * x² * y
    [0.5, Sin, Cos],         // 0.5 * sin(x) * cos(y)
    [-2.0, Exp, Pow(-1)]     // -2.0 * e^(x) * y⁻¹
});

const RESULT: f64 = POLY.evaluate([1.0, 2.0]); // -0.2182818
```

### 4. Logarithmic and Root Operations
```rust
// f(x, y) = 1.5 * ln(x) * sqrt(y)
const POLY: Polynomial<2, 1> = const_poly!([1.5, Ln, Sqrt]);
const RESULT: f64 = POLY.evaluate([2.0, 9.0]); // 3.119162312
```

### 5. Full Expression with Multiple Terms & Complex Functions
```rust
const POLY: Polynomial<3, 4> = const_poly!({
    [2.0, Pow(2), Sin, Exp],       // 2x² * sin(y) * e^(z)
    [-1.5, Ln, Pow(-1), Cos],      // -1.5 * ln(x) * y⁻¹ * cos(z)
    [0.5, Sqrt, Tan, Pow(0)],      // 0.5 * sqrt(x) * tan(y)
    [1.0, Pow(1), Pow(1), Pow(1)]  // x * y * z
});

const VARS: [f64; 3] = [2.0, 0.5, 1.0];
const RESULT: f64 = POLY.evaluate(VARS); // 10.688476972
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


