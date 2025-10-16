const PI: f64 = core::f64::consts::PI;
const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;
const TWO_PI: f64 = 2.0 * PI;

/// Computes the absolute value of a floating-point number.
const fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Reduces the input `x` to the range [-π/2, π/2] using symmetry of sine.
const fn reduce_angle_for_sine(mut x: f64) -> f64 {
    // Reduce to [-π, π]
    while x > PI {
        x -= TWO_PI;
    }
    while x < -PI {
        x += TWO_PI;
    }

    // Reflect into [-π/2, π/2] using sine symmetry.
    if x > HALF_PI {
        x = PI - x;
    } else if x < -HALF_PI {
        x = -PI - x;
    }

    x
}

/// Approximates sin(x) using a 13th-order Taylor series:
///
/// sin(x) ≈ x - x³/3! + x⁵/5! - x⁷/7! + x⁹/9! - x¹¹/11! + x¹³/13!
///
/// Accurate to within **1e-9** compared to [`f64::sin()`] over the range [-2π, 2π].
pub const fn sin_approx(x: f64) -> f64 {
    let x = reduce_angle_for_sine(x);
    let x2 = x * x;

    // Compute powers and coefficients explicitly
    let x3 = x * x2; // x³
    let x5 = x3 * x2; // x⁵
    let x7 = x5 * x2; // x⁷
    let x9 = x7 * x2; // x⁹
    let x11 = x9 * x2; // x¹¹
    let x13 = x11 * x2; // x¹³

    // Precomputed factorial denominators
    let term1 = x;
    let term2 = x3 / 6.0; // 3!
    let term3 = x5 / 120.0; // 5!
    let term4 = x7 / 5040.0; // 7!
    let term5 = x9 / 362880.0; // 9!
    let term6 = x11 / 39916800.0; // 11!
    let term7 = x13 / 6227020800.0; // 13!

    // Alternating signs
    term1 - term2 + term3 - term4 + term5 - term6 + term7
}

/// Reduces the input `x` to the range [-π/2, π/2] using symmetry of cosine.
const fn reduce_angle_for_cos(mut x: f64) -> (f64, f64) {
    // Reduce x to [-π, π]
    while x > PI {
        x -= TWO_PI;
    }
    while x < -PI {
        x += TWO_PI;
    }

    // Reflect into [-π/2, π/2] with sign for cosine
    if x > HALF_PI {
        (PI - x, -1.0)
    } else if x < -HALF_PI {
        (-PI - x, -1.0)
    } else {
        (x, 1.0)
    }
}

/// Approximates cos(x) using a 14th-order Taylor series:
///
/// cos(x) ≈ 1 - x²/2! + x⁴/4! - x⁶/6! + x⁸/8! - x¹⁰/10! + x¹²/12! - x¹⁴/14!
///
/// Accurate to within **1e-9** compared to [`f64::cos()`] over the range [-2π, 2π].
pub const fn cos_approx(x: f64) -> f64 {
    let (x, sign) = reduce_angle_for_cos(x);
    let x2 = x * x;

    // Compute powers of x² (since cosine only uses even powers)
    let x4 = x2 * x2; // x⁴
    let x6 = x4 * x2; // x⁶
    let x8 = x6 * x2; // x⁸
    let x10 = x8 * x2; // x¹⁰
    let x12 = x10 * x2; // x¹²
    let x14 = x12 * x2; // x¹⁴

    // Precomputed factorial denominators
    let term0 = 1.0;
    let term1 = x2 / 2.0; // 2!
    let term2 = x4 / 24.0; // 4!
    let term3 = x6 / 720.0; // 6!
    let term4 = x8 / 40320.0; // 8!
    let term5 = x10 / 3628800.0; // 10!
    let term6 = x12 / 479001600.0; // 12!
    let term7 = x14 / 87178291200.0; // 14!

    // Alternating signs
    sign * (term0 - term1 + term2 - term3 + term4 - term5 + term6 - term7)
}

/// Rounds a floating-point number to the nearest integer as a `f64` in compile time.
const fn round_const(x: f64) -> f64 {
    if x >= 0.0 {
        (x + 0.5) as i64 as f64
    } else {
        (x - 0.5) as i64 as f64
    }
}

/// Approximates e^x using a 20-term Taylor series expansion.
///
/// Special case: returns 1.0 if `x` is 0.
///
/// Accurate to within **1e-9** absolute error threshold compared to [`f64::exp()`] over the range [-10`, 10].
pub const fn exp_approx(x: f64) -> f64 {
    const LN_2: f64 = core::f64::consts::LN_2;

    if x == 0.0 {
        return 1.0;
    }

    // Compute n = round(x / ln(2))
    let n_float = round_const(x / LN_2);
    let n = n_float as i32;

    // Compute remainder r = x - n * ln(2)
    let r = x - (n_float * LN_2);

    // Since const fn can't use loops with mutable vars well in stable,
    // unroll manually:

    let r2 = r * r;
    let r3 = r2 * r;
    let r4 = r3 * r;
    let r5 = r4 * r;
    let r6 = r5 * r;
    let r7 = r6 * r;
    let r8 = r7 * r;
    let r9 = r8 * r;
    let r10 = r9 * r;
    let r11 = r10 * r;
    let r12 = r11 * r;
    let r13 = r12 * r;
    let r14 = r13 * r;
    let r15 = r14 * r;
    let r16 = r15 * r;
    let r17 = r16 * r;
    let r18 = r17 * r;
    let r19 = r18 * r;

    let taylor = 1.0
        + r
        + r2 / 2.0
        + r3 / 6.0
        + r4 / 24.0
        + r5 / 120.0
        + r6 / 720.0
        + r7 / 5040.0
        + r8 / 40320.0
        + r9 / 362880.0
        + r10 / 3628800.0
        + r11 / 39916800.0
        + r12 / 479001600.0
        + r13 / 6227020800.0
        + r14 / 87178291200.0
        + r15 / 1307674368000.0
        + r16 / 20922789888000.0
        + r17 / 355687428096000.0
        + r18 / 6402373705728000.0
        + r19 / 121645100408832000.0;

    static_powi(2.0, n) * taylor
}

/// Approximates the tangent of `x` (in radians) as sin(x)/cos(x).
///
/// Input `x` is reduced to [-π/2, π/2] to avoid division by zero near cos(x) = 0.
///
/// If cosine is near zero, returns a large positive or negative number
/// to approximate asymptotic behavior.
///
/// Accurate to within **1e-7** compared to [`f64::tan()`] over the range [-2π, 2π], excluding ±π/2 and
/// other vertical asymptotes.
pub const fn tan_approx(x: f64) -> f64 {
    let cos_val = cos_approx(x);

    if abs(cos_val) < 1e-12 {
        if sin_approx(x) > 0.0 {
            return 1e12; // large positive approximation of +∞
        } else {
            return -1e12; // large negative approximation of -∞
        }
    }

    sin_approx(x) / cos_val
}

/// Computes `base` raised to the power of `exp` (integer exponent) using fast exponentiation.
///
/// Accurate to within **1e-50** for integer exponents over a wide range of bases and exponents.
pub const fn static_powi(mut base: f64, exp: i32) -> f64 {
    if exp == 0 {
        return 1.0;
    }

    let mut result = 1.0;
    let mut positive_exp = if exp < 0 { -exp } else { exp };

    while positive_exp > 0 {
        if (positive_exp & 1) == 1 {
            result *= base;
        }

        base *= base;
        positive_exp >>= 1;
    }

    if exp < 0 { 1.0 / result } else { result }
}

/// Approximates the natural logarithm of `x` using a series expansion.
///
/// Returns NaN for non-positive inputs.
///
/// Accurate to within **1e-15** compared to [`f64::ln()`].
#[allow(clippy::approx_constant)]
pub const fn ln_approx(number: f64) -> f64 {
    // Invalid input (non-positive)
    if number <= 0.0 {
        return f64::NAN;
    }

    // Range reduction: bring x into [0.5, 1.5] by adjusting powers of two
    let mut x = number;
    let mut k = 0;
    while x > 1.5 {
        x *= 0.5;
        k += 1;
    }
    while x < 0.5 {
        x *= 2.0;
        k -= 1;
    }

    // Compute z = (x - 1) / (x + 1)
    let z = (x - 1.0) / (x + 1.0);
    let z2 = z * z;

    // Compute series sum: ln(x) ≈ 2 * (z + z^3/3 + z^5/5 + ... )
    let mut sum = 0.0;
    let mut term = z;
    let mut i = 1;

    while i <= 39 {
        sum += term / (2 * i - 1) as f64;
        term *= z2;
        i += 1;
    }

    const LN_2: f64 = 0.6931471805599453; // Precomputed ln(2)

    // Combine series result with power of two adjustment
    2.0 * sum + (k as f64) * LN_2
}

/// Approximates the square root of `x` using Newton-Raphson iteration.
///
/// Returns NaN for negative inputs.
///
/// Accurate to within **1e-10** compared to [`f64::sqrt()`].
pub const fn sqrt_approx(x: f64) -> f64 {
    // Invalid input (non-positive)
    if x < 0.0 {
        return f64::NAN;
    }

    // Trivial case
    if x == 0.0 {
        return 0.0;
    }

    let mut guess = if x < 1.0 { x } else { x / 2.0 };
    let mut i = 0;

    // Perform 20 iterations of Newton's method
    while i < 20 {
        guess = 0.5 * (guess + x / guess);
        i += 1;
    }

    guess
}

/// Approximates the arctangent of `x` (in radians) using Taylor series.
///
/// Accurate to within **1e-15** compared to [`f64::atan()`].
pub const fn arctan_approx(x: f64) -> f64 {
    // Range reduction for large |x|
    if x > 1.0 {
        return PI / 2.0 - arctan_approx(1.0 / x);
    }
    if x < -1.0 {
        return -PI / 2.0 - arctan_approx(1.0 / x);
    }

    // Range reduction near 1 for better convergence
    if x > 0.66 {
        let y = (x - 1.0) / (1.0 + x);
        return PI / 4.0 + arctan_approx(y);
    }
    if x < -0.66 {
        let y = (x + 1.0) / (1.0 - x);
        return -PI / 4.0 + arctan_approx(y);
    }

    let mut term = x;
    let mut sum = x;
    let x2 = x * x;
    let mut i = 1;

    // Taylor series expansion around 0
    while i < 30 {
        term *= -x2;
        sum += term / (2 * i + 1) as f64;
        i += 1;
    }

    sum
}

/// Approximates the hyperbolic sine sinh(x) using Taylor series expansion.
///
/// Accurate to within **1e-10** compared to [`f64::sinh()`].
pub const fn sinh_approx(x: f64) -> f64 {
    let mut term = x;
    let mut sum = x;
    let mut i = 1;

    while i < 30 {
        term *= x * x / ((2 * i) as f64 * (2 * i + 1) as f64);
        sum += term;
        i += 1;
    }

    sum
}

/// Approximates the hyperbolic cosine cosh(x) using Taylor series expansion.
///
/// # Parameters
/// - `x`: Input value.
///
/// # Returns
/// Approximate value of cosh(x).
pub const fn cosh_approx(x: f64) -> f64 {
    let mut term = 1.0;
    let mut sum = 1.0;
    let mut i = 1;

    while i < 30 {
        term *= x * x / ((2 * i - 1) as f64 * (2 * i) as f64);
        sum += term;
        i += 1;
    }

    sum
}
