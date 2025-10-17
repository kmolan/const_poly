use const_poly::VarFunction::*;
use const_poly::{const_poly, Polynomial};
use static_assertions::const_assert;

const fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < epsilon
}

#[test]
fn test_sin_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-6;

    const fn inner() -> bool {
        // Single variable: 3 * sin(π/2) ≈ 3 * 1 = 3
        const POLY1: Polynomial<1, 1> = const_poly!({ [3.0, [Sin]] });
        const RES1: f64 = POLY1.evaluate([1.57079632679]);
        if !approx_eq(RES1, 3.0, MAX_ERROR) {
            return false;
        }

        // Two variables: 2 * sin(0) * sin(π/2) = 2 * 0 * 1 = 0
        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Sin, Sin]] });
        const RES2: f64 = POLY2.evaluate([0.0, 1.57079632679]);
        if !approx_eq(RES2, 0.0, MAX_ERROR) {
            return false;
        }

        // Three variables: 1 * sin(π/6) * sin(π/3) * sin(0) = 0
        const POLY3: Polynomial<3, 1> = const_poly!({ [1.0, [Sin, Sin, Sin]] });
        const RES3: f64 = POLY3.evaluate([0.5235987756, 1.0471975512, 0.0]);
        if !approx_eq(RES3, 0.0, MAX_ERROR) {
            return false;
        }

        // Three variables: sin(π/6) * sin(π/3) * sin(π/2) ≈ 0.5 * 0.86602540378 * 1 = ~0.43301270189
        const POLY4: Polynomial<3, 1> = const_poly!({ [1.0, [Sin, Sin, Sin]] });
        const RES4: f64 = POLY4.evaluate([0.5235987756, 1.0471975512, 1.57079632679]); // π/6, π/3, π/2
        if !approx_eq(RES4, 0.43301270189, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_cos_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-6;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [5.0, [Cos]] });
        if !approx_eq(POLY1.evaluate([0.0]), 5.0, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Cos, Cos]] });
        if !approx_eq(
            POLY2.evaluate([3.14159265359, 1.57079632679]),
            0.0,
            MAX_ERROR,
        ) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.0, [Cos, Cos, Cos]] });
        if !approx_eq(
            POLY3.evaluate([1.0471975512, 0.7853981634, 0.5235987756]),
            0.3061862178,
            MAX_ERROR,
        ) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_exp_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-3;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [1.5, [Exp]] });
        if !approx_eq(POLY1.evaluate([0.0]), 1.5, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Exp, Exp]] });
        if !approx_eq(
            POLY2.evaluate([1.0, 0.0]),
            2.0 * 2.718281828459045,
            MAX_ERROR,
        ) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.2, [Exp, Exp, Exp]] });
        const EXPECTED3: f64 = 0.9822577906240841;
        if !approx_eq(POLY3.evaluate([0.5, -1.0, 0.3]), EXPECTED3, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_tan_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-6;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [3.0, [Tan]] });
        if !approx_eq(POLY1.evaluate([0.0]), 0.0, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Tan, Tan]] });
        if !approx_eq(POLY2.evaluate([0.78539816339, 0.0]), 0.0, MAX_ERROR) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.5, [Tan, Tan, Tan]] });
        const EXPECTED3: f64 = 1.5 * 0.309336 * 0.422793 * 0.546302;
        if !approx_eq(POLY3.evaluate([0.3, 0.4, 0.5]), EXPECTED3, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_static_powi_polynomial() {
    const MAX_ERROR: f64 = 1e-50;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [1.0, [Pow(3)]] });
        if !approx_eq(POLY1.evaluate([2.0]), 8.0, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [3.0, [Pow(1), Pow(2)]] });
        if !approx_eq(POLY2.evaluate([2.0, 3.0]), 54.0, MAX_ERROR) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.5, [Pow(2), Pow(-3), Pow(1)]] });
        if !approx_eq(
            POLY3.evaluate([-2.0, 3.0, -4.0]),
            -0.8888888888888888,
            MAX_ERROR,
        ) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_ln_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-12;

    const LN_2: f64 = 0.6931471805599453;
    const LN_HALF: f64 = -0.6931471805599453;
    const LN_10: f64 = 2.302585092994046;
    const LN_1_5: f64 = 0.4054651081081644;
    const LN_0_1: f64 = -2.3025850929940455;
    const LN_5: f64 = 1.6094379124341003;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [3.0, [Ln]] });
        if !approx_eq(POLY1.evaluate([2.0]), 3.0 * LN_2, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Ln, Ln]] });
        if !approx_eq(
            POLY2.evaluate([0.5, 10.0]),
            2.0 * LN_HALF * LN_10,
            MAX_ERROR,
        ) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.5, [Ln, Ln, Ln]] });
        if !approx_eq(
            POLY3.evaluate([1.5, 0.1, 5.0]),
            1.5 * LN_1_5 * LN_0_1 * LN_5,
            MAX_ERROR,
        ) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_sqrt_polynomial() {
    const MAX_ERROR: f64 = 1e-10;

    const fn inner() -> bool {
        const TERM1: Polynomial<1, 1> = const_poly!({ [1.0, [Sqrt]] });
        if !approx_eq(TERM1.evaluate([4.0]), 2.0, MAX_ERROR) {
            return false;
        }

        const TERM2: Polynomial<1, 1> = const_poly!({ [2.0, [Sqrt]] });
        if !approx_eq(TERM2.evaluate([0.0]), 0.0, MAX_ERROR) {
            return false;
        }

        const TERM3: Polynomial<3, 1> = const_poly!({ [1.5, [Sqrt, Sqrt, Sqrt]] });
        if !approx_eq(TERM3.evaluate([9.0, 16.0, 25.0]), 90.0, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_arctan_polynomial() {
    const MAX_ERROR: f64 = 1e-15;

    const ATAN_0_5: f64 = 0.4636476090008061;
    const ATAN_1: f64 = 0.7853981633974483;
    const ATAN_2: f64 = 1.1071487177940904;
    const ATAN_0_1: f64 = 0.09966865249116204;
    const ATAN_NEG_0_7: f64 = -0.6107259643892086;
    const ATAN_3: f64 = 1.2490457723982544;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [4.0, [Arctan]] });
        if !approx_eq(POLY1.evaluate([0.5]), 4.0 * ATAN_0_5, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<2, 1> = const_poly!({ [2.0, [Arctan, Arctan]] });
        if !approx_eq(POLY2.evaluate([1.0, 2.0]), 2.0 * ATAN_1 * ATAN_2, MAX_ERROR) {
            return false;
        }

        const POLY3: Polynomial<3, 1> = const_poly!({ [1.5, [Arctan, Arctan, Arctan]] });
        if !approx_eq(
            POLY3.evaluate([0.1, -0.7, 3.0]),
            1.5 * ATAN_0_1 * ATAN_NEG_0_7 * ATAN_3,
            MAX_ERROR,
        ) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_sinh_polynomial() {
    const MAX_ERROR: f64 = 1e-10;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [1.0, [Sinh]] });
        if !approx_eq(POLY1.evaluate([0.0]), 0.0, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<1, 1> = const_poly!({ [2.0, [Sinh]] });
        if !approx_eq(POLY2.evaluate([1.0]), 2.35040238728, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_cosh_polynomial() {
    const MAX_ERROR: f64 = 1e-10;

    const fn inner() -> bool {
        const POLY1: Polynomial<1, 1> = const_poly!({ [1.0, [Cosh]] });
        if !approx_eq(POLY1.evaluate([0.0]), 1.0, MAX_ERROR) {
            return false;
        }

        const POLY2: Polynomial<1, 1> = const_poly!({ [3.0, [Cosh]] });
        if !approx_eq(POLY2.evaluate([1.0]), 4.62924190446, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
fn test_polynomial_all_pow_functions() {
    const MAX_ERROR: f64 = 1e-9;

    // (1.2 * x² * y⁻¹ * z⁰) +
    // (-0.8 * x³ * y¹ * z⁻²) +
    // (2.5 * x⁻³ * y⁴ * z¹) +
    // (-1.1 * x⁰ * y⁻² * z³) +
    // (0.9 * x¹ * y² * z⁻¹)
    const POLY: Polynomial<3, 5> = const_poly!({
        [1.2, [Pow(2), Pow(-1), Pow(0)]],
        [-0.8, [Pow(3), Pow(1), Pow(-2)]],
        [2.5, [Pow(-3), Pow(4), Pow(1)]],
        [-1.1, [Pow(0), Pow(-2), Pow(3)]],
        [0.9, [Pow(1), Pow(2), Pow(-1)]]
    });

    const VARS: [f64; 3] = [2.0, 3.0, 0.5];

    const EXPECTED_RESULT: f64 = -30.159027778;

    const fn inner() -> bool {
        let res = POLY.evaluate(VARS);
        approx_eq(res, EXPECTED_RESULT, MAX_ERROR)
    }

    const_assert!(inner());
    assert!(inner());
}
