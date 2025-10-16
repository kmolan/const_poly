use const_poly::const_poly::polynomial::Polynomial;
use const_poly::const_poly::term::{Term, VarFunction};
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
        const TERM1: Term<1> = Term::new(3.0, [VarFunction::Sin]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([1.57079632679]);
        if !approx_eq(RES1, 3.0, MAX_ERROR) {
            return false;
        }

        // Two variables: 2 * sin(0) * sin(π/2) = 2 * 0 * 1 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Sin, VarFunction::Sin]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([0.0, 1.57079632679]);
        if !approx_eq(RES2, 0.0, MAX_ERROR) {
            return false;
        }

        // Three variables: 1 * sin(π/6) * sin(π/3) * sin(0) = 0
        const TERM3: Term<3> =
            Term::new(1.0, [VarFunction::Sin, VarFunction::Sin, VarFunction::Sin]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.5235987756, 1.0471975512, 0.0]);
        if !approx_eq(RES3, 0.0, MAX_ERROR) {
            return false;
        }

        // Three variables: sin(π/6) * sin(π/3) * sin(π/2) ≈ 0.5 * 0.86602540378 * 1 = ~0.43301270189
        const TERM4: Term<3> =
            Term::new(1.0, [VarFunction::Sin, VarFunction::Sin, VarFunction::Sin]);
        const POLY4: Polynomial<3, 1> = Polynomial::new([TERM4]);
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
        // Single var: cos(0) = 1, coefficient 5 => 5*1=5
        const TERM1: Term<1> = Term::new(5.0, [VarFunction::Cos]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 5.0, MAX_ERROR) {
            return false;
        }

        // Two vars: 2 * cos(π) * cos(π/2) = 2 * (-1) * 0 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Cos, VarFunction::Cos]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([3.14159265359, 1.57079632679]);
        if !approx_eq(RES2, 0.0, MAX_ERROR) {
            return false;
        }

        // Three vars: 1 * cos(π/3) * cos(π/4) * cos(π/6) ≈ 0.30618621784789724
        const TERM3: Term<3> =
            Term::new(1.0, [VarFunction::Cos, VarFunction::Cos, VarFunction::Cos]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([1.0471975512, 0.7853981634, 0.5235987756]);
        if !approx_eq(RES3, 0.3061862178, MAX_ERROR) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_exp_approx_polynomial() {
    const MAX_ERROR: f64 = 1e-3;

    const fn inner() -> bool {
        // Single var: e^0 = 1, coeff 1.5 => 1.5
        const TERM1: Term<1> = Term::new(1.5, [VarFunction::Exp]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 1.5, MAX_ERROR) {
            return false;
        }

        // Two vars: 2 * e^1 * e^0 = 2 * e * 1 = 2*e
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Exp, VarFunction::Exp]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([1.0, 0.0]);
        if !approx_eq(RES2, 2.0 * 2.718281828459045, MAX_ERROR) {
            return false;
        }

        // Three vars: 1.2 * e^0.5 * e^-1.0 * e^0.3 ≈ 1.2 * e^{-0.2}
        const TERM3: Term<3> =
            Term::new(1.2, [VarFunction::Exp, VarFunction::Exp, VarFunction::Exp]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.5, -1.0, 0.3]);
        const EXPECTED3: f64 = 0.9822577906240841; // e^{-0.2}
        if !approx_eq(RES3, EXPECTED3, MAX_ERROR) {
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
        // Single var: tan(0) = 0, coeff 3 => 0
        const TERM1: Term<1> = Term::new(3.0, [VarFunction::Tan]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 0.0, MAX_ERROR) {
            return false;
        }

        // Two vars: 2 * tan(π/4) * tan(0) = 2 * 1 * 0 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Tan, VarFunction::Tan]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([0.78539816339, 0.0]);
        if !approx_eq(RES2, 0.0, MAX_ERROR) {
            return false;
        }

        // Three vars: 1.5 * tan(0.3) * tan(0.4) * tan(0.5)
        const TERM3: Term<3> =
            Term::new(1.5, [VarFunction::Tan, VarFunction::Tan, VarFunction::Tan]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.3, 0.4, 0.5]);
        // Compute expected manually (hardcoded for const fn)
        const EXPECTED3: f64 = 1.5 * 0.309336 * 0.422793 * 0.546302;
        if !approx_eq(RES3, EXPECTED3, MAX_ERROR) {
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
        // Single var: 2^3 * 1 = 8
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Pow(3)]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([2.0]);
        if !approx_eq(RES1, 8.0, MAX_ERROR) {
            return false;
        }

        // Two vars: 3 * (2^1) * (3^2) = 3 * 2 * 9 = 54
        const TERM2: Term<2> = Term::new(3.0, [VarFunction::Pow(1), VarFunction::Pow(2)]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([2.0, 3.0]);
        if !approx_eq(RES2, 54.0, MAX_ERROR) {
            return false;
        }

        // Three vars with mixed positive/negative bases and exponents:
        // 1.5 * (-2)^2 * (3^-3) * (-4)^1 = 1.5 * 4 * (1/27) * (-4) = 1.5 * 4 * (1/27) * -4 = -0.8888888888888888
        const TERM3: Term<3> = Term::new(
            1.5,
            [
                VarFunction::Pow(2),
                VarFunction::Pow(-3),
                VarFunction::Pow(1),
            ],
        );
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([-2.0, 3.0, -4.0]);
        if !approx_eq(RES3, -0.8888888888888888, MAX_ERROR) {
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

    // Precomputed ln values for const_assert checks
    const LN_2: f64 = 0.6931471805599453;
    const LN_HALF: f64 = -0.6931471805599453;
    const LN_10: f64 = 2.302585092994046;
    const LN_1_5: f64 = 0.4054651081081644;
    const LN_0_1: f64 = -2.3025850929940455;
    const LN_5: f64 = 1.6094379124341003;

    const fn inner() -> bool {
        // Single variable: coefficient 3.0 * ln(2)
        const TERM1: Term<1> = Term::new(3.0, [VarFunction::Ln]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([2.0]);
        if !approx_eq(RES1, 3.0 * LN_2, MAX_ERROR) {
            return false;
        }

        // Two variables: 2.0 * ln(0.5) * ln(10)
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Ln, VarFunction::Ln]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([0.5, 10.0]);
        if !approx_eq(RES2, 2.0 * LN_HALF * LN_10, MAX_ERROR) {
            return false;
        }

        // Three variables mixed positive/negative inputs and ln function:
        // 1.5 * ln(1.5) * ln(0.1) * ln(5)
        const TERM3: Term<3> = Term::new(1.5, [VarFunction::Ln, VarFunction::Ln, VarFunction::Ln]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([1.5, 0.1, 5.0]);
        if !approx_eq(RES3, 1.5 * LN_1_5 * LN_0_1 * LN_5, MAX_ERROR) {
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
        // sqrt(4) = 2, coefficient = 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Sqrt]);
        const RES1: f64 = TERM1.evaluate([4.0]);
        if !approx_eq(RES1, 2.0, MAX_ERROR) {
            return false;
        }

        // sqrt(0) = 0
        const TERM2: Term<1> = Term::new(2.0, [VarFunction::Sqrt]); // coeff 2.0, 2*0=0
        const RES2: f64 = TERM2.evaluate([0.0]);
        if !approx_eq(RES2, 0.0, MAX_ERROR) {
            return false;
        }

        // Three variables:
        // coeff = 1.5, sqrt(9) = 3, sqrt(16) = 4, sqrt(25) = 5
        const TERM3: Term<3> = Term::new(
            1.5,
            [VarFunction::Sqrt, VarFunction::Sqrt, VarFunction::Sqrt],
        );
        const RES3: f64 = TERM3.evaluate([9.0, 16.0, 25.0]);
        if !approx_eq(RES3, 90.0, MAX_ERROR) {
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

    // Precomputed arctan values for const_assert checks (from std atan)
    const ATAN_0_5: f64 = 0.4636476090008061;
    const ATAN_1: f64 = 0.7853981633974483;
    const ATAN_2: f64 = 1.1071487177940904;
    const ATAN_0_1: f64 = 0.09966865249116204;
    const ATAN_NEG_0_7: f64 = -0.6107259643892086;
    const ATAN_3: f64 = 1.2490457723982544;

    const fn inner() -> bool {
        // Single variable: 4.0 * arctan(0.5)
        const TERM1: Term<1> = Term::new(4.0, [VarFunction::Arctan]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.5]);
        if !approx_eq(RES1, 4.0 * ATAN_0_5, MAX_ERROR) {
            return false;
        }

        // Two variables: 2.0 * arctan(1) * arctan(2)
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Arctan, VarFunction::Arctan]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([1.0, 2.0]);
        if !approx_eq(RES2, 2.0 * ATAN_1 * ATAN_2, MAX_ERROR) {
            return false;
        }

        // Three variables: 1.5 * arctan(0.1) * arctan(-0.7) * arctan(3)
        const TERM3: Term<3> = Term::new(
            1.5,
            [
                VarFunction::Arctan,
                VarFunction::Arctan,
                VarFunction::Arctan,
            ],
        );
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.1, -0.7, 3.0]);
        if !approx_eq(RES3, 1.5 * ATAN_0_1 * ATAN_NEG_0_7 * ATAN_3, MAX_ERROR) {
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
        // sinh(0) = 0, coeff 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Sinh]);
        const RES1: f64 = TERM1.evaluate([0.0]);
        if !approx_eq(RES1, 0.0, MAX_ERROR) {
            return false;
        }

        // sinh(1) ~ 1.17520119364
        const TERM2: Term<1> = Term::new(2.0, [VarFunction::Sinh]); // coeff 2.0, so result ~2.3504
        const RES2: f64 = TERM2.evaluate([1.0]);
        if !approx_eq(RES2, 2.35040238728, MAX_ERROR) {
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
        // cosh(0) = 1, coeff 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Cosh]);
        const RES1: f64 = TERM1.evaluate([0.0]);
        if !approx_eq(RES1, 1.0, MAX_ERROR) {
            return false;
        }
        // cosh(1) ~ 1.54308063482
        const TERM2: Term<1> = Term::new(3.0, [VarFunction::Cosh]); // coeff 3.0, so result ~4.6292419
        const RES2: f64 = TERM2.evaluate([1.0]);
        if !approx_eq(RES2, 4.62924190446, MAX_ERROR) {
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

    // Terms with Pow exponents: mix positive, zero, and negative powers
    const TERM1: Term<3> = Term::new(
        1.2,
        [
            VarFunction::Pow(2),  // x^2
            VarFunction::Pow(-1), // y^-1 = 1/y
            VarFunction::Pow(0),  // z^0 = 1
        ],
    );

    const TERM2: Term<3> = Term::new(
        -0.8,
        [
            VarFunction::Pow(3),  // x^3
            VarFunction::Pow(1),  // y^1 = y
            VarFunction::Pow(-2), // z^-2 = 1 / z^2
        ],
    );

    const TERM3: Term<3> = Term::new(
        2.5,
        [
            VarFunction::Pow(-3), // x^-3 = 1 / x^3
            VarFunction::Pow(4),  // y^4
            VarFunction::Pow(1),  // z^1 = z
        ],
    );

    const TERM4: Term<3> = Term::new(
        -1.1,
        [
            VarFunction::Pow(0),  // x^0 = 1
            VarFunction::Pow(-2), // y^-2 = 1 / y^2
            VarFunction::Pow(3),  // z^3
        ],
    );

    const TERM5: Term<3> = Term::new(
        0.9,
        [
            VarFunction::Pow(1),  // x^1 = x
            VarFunction::Pow(2),  // y^2
            VarFunction::Pow(-1), // z^-1 = 1/z
        ],
    );

    const POLY: Polynomial<3, 5> = Polynomial::new([TERM1, TERM2, TERM3, TERM4, TERM5]);

    // Variables chosen to avoid domain errors and zeros:
    // x = 2.0, y = 3.0, z = 0.5
    const VARS: [f64; 3] = [2.0, 3.0, 0.5];

    // Compute expected value offline:

    // TERM1: 1.2 * (2.0^2) * (3.0^-1) * (0.5^0)
    //       = 1.2 * 4.0 * (1/3) * 1 = 1.6
    // TERM2: -0.8 * (2.0^3) * (3.0^1) * (0.5^-2)
    //       = -0.8 * 8.0 * 3.0 * 4.0 = -76.8
    // TERM3: 2.5 * (2.0^-3) * (3.0^4) * (0.5^1)
    //       = 2.5 * (1/8) * 81.0 * 0.5 = 12.65625
    // TERM4: -1.1 * (2.0^0) * (3.0^-2) * (0.5^3)
    //       = -1.1 * 1 * (1/9) * 0.125 = -0.015277777...
    // TERM5: 0.9 * (2.0^1) * (3.0^2) * (0.5^-1)
    //       = 0.9 * 2.0 * 9.0 * 2.0 = 32.4

    // Sum all terms:
    // 1.6 - 76.8 + 12.65625 - 0.0152777778 + 32.4 = -30.159027778

    const EXPECTED_RESULT: f64 = -30.159027778;

    const fn inner() -> bool {
        let res = POLY.evaluate(VARS);
        approx_eq(res, EXPECTED_RESULT, MAX_ERROR)
    }

    const_assert!(inner());
    assert!(inner());
}
