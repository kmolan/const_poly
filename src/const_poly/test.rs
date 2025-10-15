use crate::const_poly::polynomial::Polynomial;
use crate::const_poly::term::Term;
use crate::const_poly::term::VarFunction;

#[test]
#[allow(dead_code)]
fn dummy_test_1() {
    const TERM1: Term<3> = Term::new(
        2.0,
        [
            VarFunction::Pow(2), // x^2
            VarFunction::Sin,    // sin(y)
            VarFunction::Exp,    // exp(z)
        ],
    );

    const POLY: Polynomial<3, 1> = Polynomial::new([TERM1]);

    const RESULT: f64 = POLY.evaluate([2.0, 1.57079632679, 0.0]);

    const_assert!((RESULT - 8.0).abs() < 1e-9);
}

#[test]
#[allow(dead_code)]
fn dummy_test_2() {
    const TERM1: Term<1> = Term::new(
        2.0,
        [VarFunction::Pow(2)], // x^2
    );

    const POLY: Polynomial<1, 1> = Polynomial::new([TERM1]);

    const RESULT: f64 = POLY.evaluate([2.0]);

    const_assert!((RESULT - 8.0).abs() < 1e-9);
}
