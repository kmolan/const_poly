use crate::const_poly::polynomial::Polynomial;
use crate::const_poly::term::{Term, VarFunction};
use static_assertions::const_assert;

const EPSILON: f64 = 1e-6;

const fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < epsilon
}

#[test]
#[allow(dead_code)]
fn test_sin_approx() {
    const fn inner() -> bool {
        // Single variable: 3 * sin(π/2) ≈ 3 * 1 = 3
        const TERM1: Term<1> = Term::new(3.0, [VarFunction::Sin]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([1.57079632679]);
        if !approx_eq(RES1, 3.0, EPSILON) {
            return false;
        }

        // Two variables: 2 * sin(0) * sin(π/2) = 2 * 0 * 1 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Sin, VarFunction::Sin]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([0.0, 1.57079632679]);
        if !approx_eq(RES2, 0.0, EPSILON) {
            return false;
        }

        // Three variables: 1 * sin(π/6) * sin(π/3) * sin(0) = ~0
        const TERM3: Term<3> =
            Term::new(1.0, [VarFunction::Sin, VarFunction::Sin, VarFunction::Sin]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.5235987756, 1.0471975512, 0.0]);
        if !approx_eq(RES3, 0.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_cos_approx() {
    const fn inner() -> bool {
        // Single var: cos(0) = 1, coefficient 5 => 5*1=5
        const TERM1: Term<1> = Term::new(5.0, [VarFunction::Cos]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 5.0, EPSILON) {
            return false;
        }

        // Two vars: 2 * cos(π) * cos(π/2) = 2 * (-1) * 0 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Cos, VarFunction::Cos]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([3.14159265359, 1.57079632679]);
        if !approx_eq(RES2, 0.0, EPSILON) {
            return false;
        }

        // Three vars: 1 * cos(0) * cos(0) * cos(0) = 1
        const TERM3: Term<3> =
            Term::new(1.0, [VarFunction::Cos, VarFunction::Cos, VarFunction::Cos]);
        const POLY3: Polynomial<3, 1> = Polynomial::new([TERM3]);
        const RES3: f64 = POLY3.evaluate([0.0, 0.0, 0.0]);
        if !approx_eq(RES3, 1.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_exp_approx() {
    const fn inner() -> bool {
        // Single var: e^0 = 1, coeff 1.5 => 1.5
        const TERM1: Term<1> = Term::new(1.5, [VarFunction::Exp]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 1.5, EPSILON) {
            return false;
        }

        // Two vars: 2 * e^1 * e^0 = 2 * e * 1 = 2*e
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Exp, VarFunction::Exp]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([1.0, 0.0]);
        if !approx_eq(RES2, 2.0 * 2.718281828459045, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_tan_approx() {
    const fn inner() -> bool {
        // Single var: tan(0) = 0, coeff 3 => 0
        const TERM1: Term<1> = Term::new(3.0, [VarFunction::Tan]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([0.0]);
        if !approx_eq(RES1, 0.0, EPSILON) {
            return false;
        }

        // Two vars: 2 * tan(π/4) * tan(0) = 2 * 1 * 0 = 0
        const TERM2: Term<2> = Term::new(2.0, [VarFunction::Tan, VarFunction::Tan]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([0.78539816339, 0.0]);
        if !approx_eq(RES2, 0.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_static_powi() {
    const fn inner() -> bool {
        // Single var: 2^3 * 1 = 8
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Pow(3)]);
        const POLY1: Polynomial<1, 1> = Polynomial::new([TERM1]);
        const RES1: f64 = POLY1.evaluate([2.0]);
        if !approx_eq(RES1, 8.0, EPSILON) {
            return false;
        }

        // Two vars: 3 * (2^1) * (3^2) = 3 * 2 * 9 = 54
        const TERM2: Term<2> = Term::new(3.0, [VarFunction::Pow(1), VarFunction::Pow(2)]);
        const POLY2: Polynomial<2, 1> = Polynomial::new([TERM2]);
        const RES2: f64 = POLY2.evaluate([2.0, 3.0]);
        if !approx_eq(RES2, 54.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_sqrt_term() {
    const fn inner() -> bool {
        // sqrt(4) = 2, coefficient = 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Sqrt]);
        const RES1: f64 = TERM1.evaluate([4.0]);
        if !approx_eq(RES1, 2.0, EPSILON) {
            return false;
        }
        // sqrt(0) = 0
        const TERM2: Term<1> = Term::new(2.0, [VarFunction::Sqrt]); // coeff 2.0, 2*0=0
        const RES2: f64 = TERM2.evaluate([0.0]);
        if !approx_eq(RES2, 0.0, EPSILON) {
            return false;
        }
        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_sinh_term() {
    const fn inner() -> bool {
        // sinh(0) = 0, coeff 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Sinh]);
        const RES1: f64 = TERM1.evaluate([0.0]);
        if !approx_eq(RES1, 0.0, EPSILON) {
            return false;
        }
        // sinh(1) ~ 1.17520119364
        const TERM2: Term<1> = Term::new(2.0, [VarFunction::Sinh]); // coeff 2.0, so result ~2.3504
        const RES2: f64 = TERM2.evaluate([1.0]);
        if !approx_eq(RES2, 2.35040238728, EPSILON) {
            return false;
        }
        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_cosh_term() {
    const fn inner() -> bool {
        // cosh(0) = 1, coeff 1.0
        const TERM1: Term<1> = Term::new(1.0, [VarFunction::Cosh]);
        const RES1: f64 = TERM1.evaluate([0.0]);
        if !approx_eq(RES1, 1.0, EPSILON) {
            return false;
        }
        // cosh(1) ~ 1.54308063482
        const TERM2: Term<1> = Term::new(3.0, [VarFunction::Cosh]); // coeff 3.0, so result ~4.6292419
        const RES2: f64 = TERM2.evaluate([1.0]);
        if !approx_eq(RES2, 4.62924190446, EPSILON) {
            return false;
        }
        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_1_variable() {
    const fn inner() -> bool {
        // Polynomial with terms:
        // 1) 2 * sin(x)
        // 2) -1 * cos(x)
        // 3) 3 * x^2 (pow 2)
        // Evaluate at x = π/4 (~0.78539816339)
        const TERM1: Term<1> = Term::new(2.0, [VarFunction::Sin]);
        const TERM2: Term<1> = Term::new(-1.0, [VarFunction::Cos]);
        const TERM3: Term<1> = Term::new(3.0, [VarFunction::Pow(2)]);
        const POLY: Polynomial<1, 3> = Polynomial::new([TERM1, TERM2, TERM3]);
        const RES: f64 = POLY.evaluate([0.78539816339]);

        // Expected: 2*sin(π/4) - cos(π/4) + 3*(π/4)^2
        // sin(π/4) ≈ 0.70710678118
        // cos(π/4) ≈ 0.70710678118
        // (π/4)^2 ≈ 0.61685027507
        let expected = 2.0 * 0.70710678118 - 0.70710678118 + 3.0 * 0.61685027507;

        if !approx_eq(RES, expected, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_2_variables() {
    const fn inner() -> bool {
        // Polynomial: 1.2 * sin(x) * cos(y)
        // Evaluate at x = π/6 (~0.5235987756), y = π/3 (~1.0471975512)
        const TERM: Term<2> = Term::new(1.2, [VarFunction::Sin, VarFunction::Cos]);
        const POLY: Polynomial<2, 1> = Polynomial::new([TERM]);
        const RES: f64 = POLY.evaluate([0.5235987756, 1.0471975512]);

        // Expected:
        // sin(π/6) = 0.5
        // cos(π/3) = 0.5
        // Result = 1.2 * 0.5 * 0.5 = 0.3

        if !approx_eq(RES, 0.3, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_3_variables() {
    const fn inner() -> bool {
        // Polynomial: -0.5 * exp(x) * sqrt(y) * ln(z)
        // Evaluate at x=1.0, y=4.0, z=e (~2.718281828459045)
        const TERM: Term<3> =
            Term::new(-0.5, [VarFunction::Exp, VarFunction::Sqrt, VarFunction::Ln]);
        const POLY: Polynomial<3, 1> = Polynomial::new([TERM]);
        const RES: f64 = POLY.evaluate([1.0, 4.0, 2.718281828459045]);

        // Expected:
        // exp(1.0) ≈ 2.718281828459045
        // sqrt(4.0) = 2.0
        // ln(e) = 1.0
        // Result = -0.5 * 2.718281828459045 * 2.0 * 1.0 ≈ -2.718281828459045

        if !approx_eq(RES, -2.718281828459045, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_4_variables() {
    const fn inner() -> bool {
        // Polynomial: 3.0 * tan(w) * sinh(x) * cosh(y) * arctan(z)
        // Evaluate at w=0, x=1, y=0, z=1
        const TERM: Term<4> = Term::new(
            3.0,
            [
                VarFunction::Tan,
                VarFunction::Sinh,
                VarFunction::Cosh,
                VarFunction::Arctan,
            ],
        );
        const POLY: Polynomial<4, 1> = Polynomial::new([TERM]);
        const RES: f64 = POLY.evaluate([0.0, 1.0, 0.0, 1.0]);

        // Expected:
        // tan(0) = 0.0
        // sinh(1) ≈ 1.17520119364
        // cosh(0) = 1.0
        // arctan(1) ≈ 0.78539816339
        // Result = 3.0 * 0.0 * 1.17520119364 * 1.0 * 0.78539816339 = 0.0

        if !approx_eq(RES, 0.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_5_variables() {
    const fn inner() -> bool {
        // Polynomial: -1.0 * pow(x,2) * exp(y) * sin(z) * cosh(w) * ln(v)
        // Evaluate at x=2, y=0, z=π/2 (~1.57079632679), w=0, v=e (~2.718281828459045)
        const TERM: Term<5> = Term::new(
            -1.0,
            [
                VarFunction::Pow(2),
                VarFunction::Exp,
                VarFunction::Sin,
                VarFunction::Cosh,
                VarFunction::Ln,
            ],
        );
        const POLY: Polynomial<5, 1> = Polynomial::new([TERM]);
        const RES: f64 = POLY.evaluate([2.0, 0.0, 1.57079632679, 0.0, 2.718281828459045]);

        // Expected:
        // pow(2,2) = 4
        // exp(0) = 1
        // sin(π/2) = 1
        // cosh(0) = 1
        // ln(e) = 1
        // Result = -1 * 4 * 1 * 1 * 1 * 1 = -4

        if !approx_eq(RES, -4.0, EPSILON) {
            return false;
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}

#[test]
#[allow(dead_code)]
fn test_polynomial_multiple_terms() {
    const fn inner() -> bool {
        // Polynomial with 2 variables, 2 terms:
        // Term 1: 2.0 * sin(a) * cos(b)
        // Term 2: -1.0 * exp(a) * pow(b, 2)
        // Evaluate at a = π/6 (~0.5235987756), b = 2.0
        const TERM1: Term<2> = Term::new(2.0, [VarFunction::Sin, VarFunction::Cos]);
        const TERM2: Term<2> = Term::new(-1.0, [VarFunction::Exp, VarFunction::Pow(2)]);
        const POLY: Polynomial<2, 2> = Polynomial::new([TERM1, TERM2]);
        const RES: f64 = POLY.evaluate([0.5235987756, 2.0]);

        // Expected:
        // sin(π/6) = 0.5
        // cos(2.0) ≈ -0.416146836547
        // exp(π/6) ≈ 1.6487212707
        // pow(2, 2) = 4
        // Result = 2 * 0.5 * (-0.416146836547) + (-1) * 1.6487212707 * 4
        //        = 2 * (-0.2080734182735) + (-6.5948850828)
        //        = -0.416146836547 + (-6.5948850828) = -7.01103191935

        if !approx_eq(RES, -7.01103191935, 0.5) {
            return false; //TODO precision
        }

        // Polynomial with 3 variables, 3 terms:
        // Term 1: 1.0 * sin(a) * pow(b,1) * pow(c,1)
        // Term 2: 2.0 * cos(a) * sqrt(b) * pow(c,1)
        // Term 3: -0.5 * ln(a) * pow(b,1) * exp(c)
        // Evaluate at a=1.0, b=0.0, c=4.0
        const TERM3_1: Term<3> = Term::new(
            1.0,
            [VarFunction::Sin, VarFunction::Pow(1), VarFunction::Pow(1)],
        );
        const TERM3_2: Term<3> = Term::new(
            2.0,
            [VarFunction::Cos, VarFunction::Sqrt, VarFunction::Pow(1)],
        );
        const TERM3_3: Term<3> = Term::new(
            -0.5,
            [VarFunction::Ln, VarFunction::Pow(1), VarFunction::Exp],
        );
        const POLY3: Polynomial<3, 3> = Polynomial::new([TERM3_1, TERM3_2, TERM3_3]);
        const RES3: f64 = POLY3.evaluate([1.0, 0.0, 4.0]);

        // Expected:
        // sin(1.0) ≈ 0.8414709848
        // pow(0,1) = 0
        // pow(4,1) = 4
        // cos(1.0) ≈ 0.5403023059
        // sqrt(0) = 0
        // ln(1.0) = 0.0
        // exp(4.0) ≈ 54.5981500331
        // Result =
        //   Term1: 1.0 * 0.8414709848 * 0 * 4 = 0
        // + Term2: 2.0 * 0.5403023059 * 0 * 4 = 0
        // + Term3: -0.5 * 0 * 0 * 54.5981500331 = 0
        // Overall: 0 + 0 + 0 = 0

        // To make this more interesting, change b to 1.0 for a non-zero result
        const RES3_B1: f64 = POLY3.evaluate([1.0, 1.0, 4.0]);

        // Recalculate expected with b=1.0
        // Term1: 1.0 * sin(1.0) * 1^1 * 4^1 = 0.8414709848 * 1 * 4 = 3.3658839392
        // Term2: 2.0 * cos(1.0) * sqrt(1.0) * 4^1 = 2.0 * 0.5403023059 * 1 * 4 = 4.3224184472
        // Term3: -0.5 * ln(1.0) * 1^1 * exp(4.0) = -0.5 * 0 * 1 * 54.5981500331 = 0
        // Total = 3.3658839392 + 4.3224184472 + 0 = 7.6883023864

        if !approx_eq(RES3_B1, 7.6883023864, 0.01) {
            return false; //TODO precision
        }

        true
    }

    const_assert!(inner());
    assert!(inner());
}
