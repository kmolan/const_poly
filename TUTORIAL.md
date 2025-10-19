# `const_poly` — Compile-Time Polynomial Evaluation in Rust

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Quick Start Example](#quick-start-example)
- [Understanding the Syntax](#understanding-the-syntax)
  - [1. Basic Structure](#1-basic-structure)
  - [2. Multiple Terms](#2-multiple-terms)
  - [3. Supported Functions](#3-supported-functions)
- [Simple Examples](#simple-examples)
  - [Sine Example](#-sine-example)
  - [Cosine Example](#-cosine-example)
  - [Exponential Example](#-exponential-example)
  - [Power Function Example](#-power-function-example)
- [Advanced: Multi-Term, Multi-Variable Polynomials](#advanced-multi-term-multi-variable-polynomials)

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

Here’s how you can define and evaluate a simple polynomial at compile time:

```rust
use const_poly::VarFunction::*;
use const_poly::{Polynomial, const_poly};

// f(x,y) = 2.5 * x² * y³
const POLY = Polynomial<2, 1> = const_poly!([2.5, Pow(2), Pow(3)]);

// evaluate at (x,y) = (10.0, -5.0)
const RESULT: f64 = POLY_1.evaluate([10.0, -5.0]); // -31250

// g(x,y) = 3.0 * sin(x) * cos(y)
const POLY_2: Polynomial<2, 1> = const_poly!([3.0, Sin, Cos]);

// evaluate at (x,y) = (π/2, 0.0)
const RES_2: f64 = POLY_2.evaluate([1.57079632679, 0.0]); // 3.0

```

## Understanding the Syntax

### 1. Basic Structure

A polynomial is defined as:

```rust
const POLY: Polynomial<N_VARS, N_TERMS> = const_poly!([...]);
```

Each **term** is a list:
```rust
[coefficient, function1, function2, ...]
```

For example:
```rust
const_poly!([2.0, Sin, Cos]);
```
means:  
> `2.0⋅Sin(x)⋅Cos(y)`

---

### 2. Multiple Terms

You can define multi-term polynomials using curly braces `{ ... }`:

```rust
const POLY: Polynomial<3, 2> = const_poly!({
    [1.0, Sin, Cos, Tan],          // 1.0 * sin(x) * cos(y) * tan(z)
    [2.5, Pow(2), Pow(3), Pow(-1)] // 2.5 * x² * y³ * z⁻¹
});
```

This represents:
> `f(x,y,z) = 1.0⋅Sin(x)⋅Cos(y)⋅Tan(z) + 2.5⋅x²⋅y³⋅z⁻¹`

---

### 3. Supported Functions

| Function | Description |
|-----------|--------------|
| `Sin` | Sine
| `Cos` | Cosine
| `Tan` | Tangent
| `Exp` | Exponential
| `Ln` | Natural Logarithm
| `Sqrt` | Square Root
| `Sinh` | Hyperbolic Sine
| `Cosh` | Hyperbolic Cosine
| `Arctan` | Arctangent
| `Pow(n)` | Power (integer exponent)

## Simple Examples

### Sine Example

```rust
const POLY: Polynomial<1, 1> = const_poly!([3.0, Sin]);
const RES: f64 = POLY.evaluate([1.57079632679]); // 3*sin(π/2)
assert!((RES - 3.0).abs() < 1e-6);
```

### Cosine Example

```rust
const POLY: Polynomial<2, 1> = const_poly!([2.0, Cos, Cos]);
const RES: f64 = POLY.evaluate([3.14159265359, 1.57079632679]); // 2*cos(π)*cos(π/2)
assert!((RES - 0.0).abs() < 1e-6);
```

### Exponential Example

```rust
const POLY: Polynomial<2, 1> = const_poly!([2.0, Exp, Exp]);
const RES: f64 = POLY.evaluate([1.0, 0.0]); // 2 * e^1 * e^0
assert!((RES - 2.0 * 2.718281828459045).abs() < 1e-3);
```

### Power Function Example

```rust
const POLY: Polynomial<3, 1> = const_poly!([1.5, Pow(2), Pow(-3), Pow(1)]);
const RES: f64 = POLY.evaluate([-2.0, 3.0, -4.0]);
// 1.5 * (-2)^2 * (3)^-3 * (-4)^1
assert!((RES - -0.8888888888888888).abs() < 1e-50);
```

## Advanced: Multi-Term, Multi-Variable Polynomials

You can define a polynomial with multiple terms and multiple variables:

```rust
// Multi-term polynomial with 3 variables and 5 terms
const POLY_3V_5T: Polynomial<3, 5> = const_poly!({
    [1.2, Pow(2), Pow(-1), Pow(0)],  // 1.2 * x² * y⁻¹ * z⁰
    [-0.8, Pow(3), Pow(1), Pow(-2)], // -0.8 * x³ * y¹ * z⁻²
    [2.5, Pow(-3), Pow(4), Pow(1)],  // 2.5 * x⁻³ * y⁴ * z¹
    [-1.1, Pow(0), Pow(-2), Pow(3)], // -1.1 * x⁰ * y⁻² * z³
    [0.9, Pow(1), Pow(2), Pow(-1)]   // 0.9 * x¹ * y² * z⁻¹
});

const VARS: [f64; 3] = [2.0, 3.0, 0.5]; // (x,y,z) = (2.0,3.0,0.5)
const RES: f64 = POLY_3V_5T.evaluate(VARS); // -30.159027778


// Multi-term polynomial with 3 variables and 3 terms
const POLY_3V_3T: Polynomial<3, 3> = const_poly!({
        [1.5, Sin, Identity, Pow(2)],  // 1.5 * Sin(x) * y * z²
        [-2.0, Cos, Pow(3), Identity], // -2.0 * Cos(x) * y³ * z
        [0.5, Exp, Ln, Sqrt]           // 0.5 * e^x * ln(y) * sqrt(z)
    });

const VARS_2 = [1.0, 2.0, 3.0]; // (x,y,z) = (1.0,2.0,3.0)
const RES_2: f64 = POLY_3V_3T.evaluate(VARS_2); //-1.583055539077
```

Each term multiplies its coefficient by the product of function results over all variables.

## Further Reading
For more examples, see [tests/](./tests/polynomial_tests.rs) for a comprehensive suite of tests.
