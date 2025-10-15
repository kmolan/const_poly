/// Mathematical constant π (pi)
#[allow(clippy::approx_constant)]
const PI: f64 = 3.141_592_653_589_793;

/// Mathematical constant 2π (two times pi)
const TWO_PI: f64 = 2.0 * PI;

/// Mathematical constant π/2 (half pi)
const HALF_PI: f64 = PI / 2.0;

/// Computes the absolute value of a floating-point number.
///
/// # Parameters
/// - `x`: The input value.
///
/// # Returns
/// The absolute value of `x`.
const fn abs(x: f64) -> f64 {
    if x < 0.0 { -x } else { x }
}

/// Reduces an angle `x` to the range [-π, π].
///
/// # Parameters
/// - `x`: The input angle in radians.
///
/// # Returns
/// The angle reduced to [-π, π].
const fn reduce_angle(mut x: f64) -> f64 {
    while x > PI {
        x -= TWO_PI;
    }
    while x < -PI {
        x += TWO_PI;
    }
    x
}

/// Reduces an angle `x` to the range [-π/2, π/2].
///
/// # Parameters
/// - `x`: The input angle in radians.
///
/// # Returns
/// The angle reduced to [-π/2, π/2].
const fn reduce_angle_half_pi(mut x: f64) -> f64 {
    while x > HALF_PI {
        x -= PI;
    }
    while x < -HALF_PI {
        x += PI;
    }
    x
}

/// Approximates the sine of `x` (in radians) using a Taylor series expansion.
///
/// The input `x` is first normalized to [-π, π] for better convergence.
///
/// # Parameters
/// - `x`: Angle in radians.
///
/// # Returns
/// Approximate value of sin(x).
pub const fn sin_approx(x: f64) -> f64 {
    let x = reduce_angle(x);

    let mut term = x;
    let mut sum = x;
    let mut i = 1;

    while i < 10 {
        term *= -x * x / ((2 * i) as f64 * (2 * i + 1) as f64);
        sum += term;
        i += 1;
    }

    sum
}

/// Approximates the cosine of `x` (in radians) using a Taylor series expansion.
///
/// The input `x` is first normalized to [-π, π] for better convergence.
///
/// # Parameters
/// - `x`: Angle in radians.
///
/// # Returns
/// Approximate value of cos(x).
pub const fn cos_approx(x: f64) -> f64 {
    let x = reduce_angle(x);

    let mut term = 1.0;
    let mut sum = 1.0;
    let mut i = 1;

    while i < 10 {
        term *= -x * x / ((2 * i - 1) as f64 * (2 * i) as f64);
        sum += term;
        i += 1;
    }

    sum
}

/// Approximates the exponential function e^x using a Taylor series expansion.
///
/// Special case: returns 1.0 if `x` is 0.
///
/// # Parameters
/// - `x`: The exponent.
///
/// # Returns
/// Approximate value of e raised to the power `x`.
pub const fn exp_approx(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }

    let mut term = 1.0;
    let mut sum = 1.0;
    let mut i = 1;

    while i < 20 {
        term *= x / i as f64;
        sum += term;
        i += 1;
    }

    sum
}

/// Approximates the tangent of `x` (in radians) as sin(x)/cos(x).
///
/// Input `x` is reduced to [-π/2, π/2] to avoid division by zero near cos(x) = 0.
///
/// If cosine is near zero, returns a large positive or negative number
/// to approximate asymptotic behavior.
///
/// # Parameters
/// - `x`: Angle in radians.
///
/// # Returns
/// Approximate value of tan(x).
pub const fn tan_approx(x: f64) -> f64 {
    let x = reduce_angle_half_pi(x);

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
/// # Parameters
/// - `base`: The base value.
/// - `exp`: The exponent (non-negative integer).
///
/// # Returns
/// `base` raised to the power of `exp`.
pub const fn static_powi(mut base: f64, mut exp: u32) -> f64 {
    let mut result = 1.0;

    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }

        base *= base;
        exp /= 2;
    }

    result
}

/// Approximates the natural logarithm of `x` using a series expansion.
///
/// Returns NaN for non-positive inputs (domain error).
///
/// Input `x` is normalized to the range [0.5, 1.5] by factoring powers of two
/// to improve convergence.
///
/// # Parameters
/// - `x`: Input value (must be > 0).
///
/// # Returns
/// Approximate value of ln(x).
#[allow(clippy::approx_constant)]
pub const fn ln_approx(x: f64) -> f64 {
    if x <= 0.0 {
        return f64::NAN;
    }

    let mut y = x;
    let mut k = 0;

    while y > 1.5 {
        y *= 0.5;
        k += 1;
    }
    while y < 0.5 {
        y *= 2.0;
        k -= 1;
    }

    let z = (y - 1.0) / (y + 1.0);
    let z2 = z * z;

    let mut term = z;
    let mut sum = 0.0;
    let mut i = 1;

    while i < 20 {
        sum += term / (2 * i - 1) as f64;
        term *= z2;
        i += 1;
    }

    let ln2 = 0.6931471805599453; // Precomputed ln(2)
    2.0 * sum + (k as f64) * ln2
}

/// Approximates the square root of `x` using Newton-Raphson iteration.
///
/// Returns NaN for negative inputs.
///
/// Uses a fixed number of iterations for const evaluation.
///
/// # Parameters
/// - `x`: Input value.
///
/// # Returns
/// Approximate value of sqrt(x).
pub const fn sqrt_approx(x: f64) -> f64 {
    if x < 0.0 {
        return f64::NAN;
    }

    if x == 0.0 {
        return 0.0;
    }

    let mut guess = x / 2.0;
    let mut i = 0;

    while i < 10 {
        guess = 0.5 * (guess + x / guess);
        i += 1;
    }

    guess
}

/// Approximates the arctangent of `x` (in radians) using Taylor series.
///
/// Uses identities to extend domain to all real numbers:
/// - arctan(x) = π/2 - arctan(1/x) for x > 1
/// - arctan(x) = -π/2 - arctan(1/x) for x < -1
///
/// # Parameters
/// - `x`: Input value.
///
/// # Returns
/// Approximate value of arctan(x).
pub const fn arctan_approx(x: f64) -> f64 {
    if x > 1.0 {
        return PI / 2.0 - arctan_approx(1.0 / x);
    }
    if x < -1.0 {
        return -PI / 2.0 - arctan_approx(1.0 / x);
    }

    let mut term = x;
    let mut sum = x;
    let x2 = x * x;
    let mut i = 1;

    while i < 10 {
        term *= -x2;
        sum += term / (2 * i + 1) as f64;
        i += 1;
    }

    sum
}

/// Approximates the hyperbolic sine sinh(x) using Taylor series expansion.
///
/// # Parameters
/// - `x`: Input value.
///
/// # Returns
/// Approximate value of sinh(x).
pub const fn sinh_approx(x: f64) -> f64 {
    let mut term = x;
    let mut sum = x;
    let mut i = 1;

    while i < 10 {
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

    while i < 10 {
        term *= x * x / ((2 * i - 1) as f64 * (2 * i) as f64);
        sum += term;
        i += 1;
    }

    sum
}
