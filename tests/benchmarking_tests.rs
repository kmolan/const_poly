use const_poly::polynomial::Polynomial;
use const_poly::term::{Term, VarFunction};

// Helper to print assertion info
fn assert_approx_eq_with_debug(a: f64, b: f64, tol: f64, label: &str) {
    let diff = (a - b).abs();
    if diff >= tol {
        panic!(
            "{} assertion failed:\n  a = {:.12}\n  b = {:.12}\n  diff = {:.12}\n  tol = {:.12}",
            label, a, b, diff, tol
        );
    }
}

#[test]
fn benchmarking_tests() {
    // 3 variables, 3 terms
    const POLY_3V_3T: Polynomial<3, 3> = Polynomial::new([
        Term::new(
            1.5,
            [VarFunction::Sin, VarFunction::Identity, VarFunction::Pow(2)],
        ),
        Term::new(
            -2.0,
            [VarFunction::Cos, VarFunction::Pow(3), VarFunction::Identity],
        ),
        Term::new(0.5, [VarFunction::Exp, VarFunction::Ln, VarFunction::Sqrt]),
    ]);

    // 4 variables, 4 terms
    const POLY_4V_4T: Polynomial<4, 4> = Polynomial::new([
        Term::new(
            3.0,
            [
                VarFunction::Identity,
                VarFunction::Sin,
                VarFunction::Pow(2),
                VarFunction::Cos,
            ],
        ),
        Term::new(
            -1.2,
            [
                VarFunction::Pow(3),
                VarFunction::Tan,
                VarFunction::Exp,
                VarFunction::Identity,
            ],
        ),
        Term::new(
            0.7,
            [
                VarFunction::Ln,
                VarFunction::Sqrt,
                VarFunction::Arctan,
                VarFunction::Sinh,
            ],
        ),
        Term::new(
            1.1,
            [
                VarFunction::Cosh,
                VarFunction::Identity,
                VarFunction::Pow(1),
                VarFunction::Sin,
            ],
        ),
    ]);

    // 5 variables, 5 terms
    const POLY_5V_5T: Polynomial<5, 5> = Polynomial::new([
        Term::new(
            2.0,
            [
                VarFunction::Sin,
                VarFunction::Cos,
                VarFunction::Pow(2),
                VarFunction::Identity,
                VarFunction::Ln,
            ],
        ),
        Term::new(
            -3.3,
            [
                VarFunction::Exp,
                VarFunction::Pow(3),
                VarFunction::Tan,
                VarFunction::Sqrt,
                VarFunction::Identity,
            ],
        ),
        Term::new(
            1.7,
            [
                VarFunction::Arctan,
                VarFunction::Sinh,
                VarFunction::Cosh,
                VarFunction::Identity,
                VarFunction::Pow(1),
            ],
        ),
        Term::new(
            0.9,
            [
                VarFunction::Identity,
                VarFunction::Identity,
                VarFunction::Identity,
                VarFunction::Identity,
                VarFunction::Identity,
            ],
        ),
        Term::new(
            -0.5,
            [
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
            ],
        ),
    ]);

    const POLY_5V_5T_2: Polynomial<5, 5> = Polynomial::new([
        Term::new(
            1.1,
            [
                VarFunction::Cos,
                VarFunction::Exp,
                VarFunction::Pow(1),
                VarFunction::Ln,
                VarFunction::Sqrt,
            ],
        ),
        Term::new(
            -2.2,
            [
                VarFunction::Pow(2),
                VarFunction::Sin,
                VarFunction::Identity,
                VarFunction::Cosh,
                VarFunction::Pow(3),
            ],
        ),
        Term::new(
            0.8,
            [
                VarFunction::Tan,
                VarFunction::Arctan,
                VarFunction::Sinh,
                VarFunction::Pow(2),
                VarFunction::Identity,
            ],
        ),
        Term::new(
            -0.7,
            [
                VarFunction::Identity,
                VarFunction::Ln,
                VarFunction::Cos,
                VarFunction::Pow(1),
                VarFunction::Exp,
            ],
        ),
        Term::new(
            1.5,
            [
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
            ],
        ),
    ]);

    // Additional POLY_5V_5T #2
    const POLY_5V_5T_3: Polynomial<5, 5> = Polynomial::new([
        Term::new(
            -1.3,
            [
                VarFunction::Sinh,
                VarFunction::Pow(3),
                VarFunction::Identity,
                VarFunction::Ln,
                VarFunction::Cos,
            ],
        ),
        Term::new(
            2.4,
            [
                VarFunction::Exp,
                VarFunction::Pow(1),
                VarFunction::Sin,
                VarFunction::Cosh,
                VarFunction::Pow(2),
            ],
        ),
        Term::new(
            -0.9,
            [
                VarFunction::Arctan,
                VarFunction::Identity,
                VarFunction::Pow(2),
                VarFunction::Tan,
                VarFunction::Sqrt,
            ],
        ),
        Term::new(
            1.0,
            [
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
            ],
        ),
        Term::new(
            -0.6,
            [
                VarFunction::Identity,
                VarFunction::Cos,
                VarFunction::Exp,
                VarFunction::Pow(3),
                VarFunction::Sin,
            ],
        ),
    ]);

    // Additional POLY_5V_5T #3
    const POLY_5V_5T_4: Polynomial<5, 5> = Polynomial::new([
        Term::new(
            2.2,
            [
                VarFunction::Pow(1),
                VarFunction::Sqrt,
                VarFunction::Ln,
                VarFunction::Sin,
                VarFunction::Exp,
            ],
        ),
        Term::new(
            -1.8,
            [
                VarFunction::Pow(3),
                VarFunction::Cos,
                VarFunction::Identity,
                VarFunction::Tan,
                VarFunction::Pow(2),
            ],
        ),
        Term::new(
            1.3,
            [
                VarFunction::Cosh,
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Arctan,
                VarFunction::Pow(1),
            ],
        ),
        Term::new(
            -0.4,
            [
                VarFunction::Identity,
                VarFunction::Pow(2),
                VarFunction::Sinh,
                VarFunction::Pow(3),
                VarFunction::Cos,
            ],
        ),
        Term::new(
            0.7,
            [
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
                VarFunction::Pow(1),
            ],
        ),
    ]);

    // --- Test 1: 3 variables, 3 terms ---
    let vars_3 = [1.0_f64, 2.0, 3.0];
    let runtime_val_3 = POLY_3V_3T.evaluate(vars_3);
    const COMPILE_VAL_3: f64 = POLY_3V_3T.evaluate([1.0, 2.0, 3.0]);
    assert_approx_eq_with_debug(
        runtime_val_3,
        COMPILE_VAL_3,
        1e-12,
        "3V3T runtime vs compile-time",
    );

    let expected_3 = 1.5 * vars_3[0].sin() * vars_3[1] * vars_3[2].powi(2)
        - 2.0 * vars_3[0].cos() * vars_3[1].powi(3) * vars_3[2]
        + 0.5 * vars_3[0].exp() * vars_3[1].ln() * vars_3[2].sqrt();
    assert_approx_eq_with_debug(runtime_val_3, expected_3, 1e-10, "3V3T runtime vs expected");

    // --- Test 2: 4 variables, 4 terms ---
    let vars_4 = [0.5_f64, 1.5, 2.5, 3.5];
    let runtime_val_4 = POLY_4V_4T.evaluate(vars_4);
    const COMPILE_VAL_4: f64 = POLY_4V_4T.evaluate([0.5, 1.5, 2.5, 3.5]);
    assert_approx_eq_with_debug(
        runtime_val_4,
        COMPILE_VAL_4,
        1e-12,
        "4V4T runtime vs compile-time",
    );

    let expected_4 = 3.0 * vars_4[0] * vars_4[1].sin() * vars_4[2].powi(2) * vars_4[3].cos()
        - 1.2 * vars_4[0].powi(3) * vars_4[1].tan() * vars_4[2].exp() * vars_4[3]
        + 0.7 * vars_4[0].ln() * vars_4[1].sqrt() * vars_4[2].atan() * vars_4[3].sinh()
        + 1.1 * vars_4[0].cosh() * vars_4[1] * vars_4[2].powi(1) * vars_4[3].sin();
    assert_approx_eq_with_debug(runtime_val_4, expected_4, 1e-7, "4V4T runtime vs expected");

    // --- Test 3: 5 variables, 5 terms ---
    let vars_5 = [0.1_f64, 1.2, 2.3, 3.4, 4.5];
    let runtime_val_5 = POLY_5V_5T.evaluate(vars_5);
    const COMPILE_VAL_5: f64 = POLY_5V_5T.evaluate([0.1, 1.2, 2.3, 3.4, 4.5]);
    assert_approx_eq_with_debug(
        runtime_val_5,
        COMPILE_VAL_5,
        1e-12,
        "5V5T runtime vs compile-time",
    );

    let product_all = vars_5.iter().product::<f64>();
    let expected_5 =
        2.0 * vars_5[0].sin() * vars_5[1].cos() * vars_5[2].powi(2) * vars_5[3] * vars_5[4].ln()
            - 3.3
                * vars_5[0].exp()
                * vars_5[1].powi(3)
                * vars_5[2].tan()
                * vars_5[3].sqrt()
                * vars_5[4]
            + 1.7
                * vars_5[0].atan()
                * vars_5[1].sinh()
                * vars_5[2].cosh()
                * vars_5[3]
                * vars_5[4].powi(1)
            + 0.9 * product_all
            - 0.5 * product_all;
    assert_approx_eq_with_debug(runtime_val_5, expected_5, 1e-12, "5V5T runtime vs expected");

    let vars_5_2 = [0.9_f64, 1.1, 1.3, 1.5, 1.7];
    let runtime_val_5_2 = POLY_5V_5T_2.evaluate(vars_5_2);
    const COMPILE_VAL_5_2: f64 = POLY_5V_5T_2.evaluate([0.9, 1.1, 1.3, 1.5, 1.7]);
    assert_approx_eq_with_debug(
        runtime_val_5_2,
        COMPILE_VAL_5_2,
        1e-12,
        "5V5T #2 runtime vs compile-time",
    );

    let expected_5_2 = 1.1
        * vars_5_2[0].cos()
        * vars_5_2[1].exp()
        * vars_5_2[2]
        * vars_5_2[3].ln()
        * vars_5_2[4].sqrt()
        - 2.2
            * vars_5_2[0].powi(2)
            * vars_5_2[1].sin()
            * vars_5_2[2]
            * vars_5_2[3].cosh()
            * vars_5_2[4].powi(3)
        + 0.8
            * vars_5_2[0].tan()
            * vars_5_2[1].atan()
            * vars_5_2[2].sinh()
            * vars_5_2[3].powi(2)
            * vars_5_2[4]
        - 0.7
            * vars_5_2[0]
            * vars_5_2[1].ln()
            * vars_5_2[2].cos()
            * vars_5_2[3]
            * vars_5_2[4].exp()
        + 1.5 * vars_5_2[0] * vars_5_2[1] * vars_5_2[2] * vars_5_2[3] * vars_5_2[4];
    assert_approx_eq_with_debug(
        runtime_val_5_2,
        expected_5_2,
        1e-10,
        "5V5T #2 runtime vs expected",
    );

    let vars_5_3 = [1.2_f64, 1.4, 1.6, 1.8, 2.0];
    let runtime_val_5_3 = POLY_5V_5T_3.evaluate(vars_5_3);
    const COMPILE_VAL_5_3: f64 = POLY_5V_5T_3.evaluate([1.2, 1.4, 1.6, 1.8, 2.0]);
    assert_approx_eq_with_debug(
        runtime_val_5_3,
        COMPILE_VAL_5_3,
        1e-12,
        "5V5T #3 runtime vs compile-time",
    );

    let expected_5_3 = -1.3
        * vars_5_3[0].sinh()
        * vars_5_3[1].powi(3)
        * vars_5_3[2]
        * vars_5_3[3].ln()
        * vars_5_3[4].cos()
        + 2.4
            * vars_5_3[0].exp()
            * vars_5_3[1]
            * vars_5_3[2].sin()
            * vars_5_3[3].cosh()
            * vars_5_3[4].powi(2)
        - 0.9
            * vars_5_3[0].atan()
            * vars_5_3[1]
            * vars_5_3[2].powi(2)
            * vars_5_3[3].tan()
            * vars_5_3[4].sqrt()
        + 1.0 * vars_5_3[0] * vars_5_3[1] * vars_5_3[2] * vars_5_3[3] * vars_5_3[4]
        - 0.6
            * vars_5_3[0]
            * vars_5_3[1].cos()
            * vars_5_3[2].exp()
            * vars_5_3[3].powi(3)
            * vars_5_3[4].sin();
    assert_approx_eq_with_debug(
        runtime_val_5_3,
        expected_5_3,
        1e-7,
        "5V5T #3 runtime vs expected",
    );

    let vars_5_4 = [0.8_f64, 1.0, 1.2, 1.4, 1.6];
    let runtime_val_5_4 = POLY_5V_5T_4.evaluate(vars_5_4);
    const COMPILE_VAL_5_4: f64 = POLY_5V_5T_4.evaluate([0.8, 1.0, 1.2, 1.4, 1.6]);
    assert_approx_eq_with_debug(
        runtime_val_5_4,
        COMPILE_VAL_5_4,
        1e-12,
        "5V5T #4 runtime vs compile-time",
    );

    let expected_5_4 = 2.2
        * vars_5_4[0]
        * vars_5_4[1].sqrt()
        * vars_5_4[2].ln()
        * vars_5_4[3].sin()
        * vars_5_4[4].exp()
        - 1.8
            * vars_5_4[0].powi(3)
            * vars_5_4[1].cos()
            * vars_5_4[2]
            * vars_5_4[3].tan()
            * vars_5_4[4].powi(2)
        + 1.3 * vars_5_4[0].cosh() * vars_5_4[1] * vars_5_4[2] * vars_5_4[3].atan() * vars_5_4[4]
        - 0.4
            * vars_5_4[0]
            * vars_5_4[1].powi(2)
            * vars_5_4[2].sinh()
            * vars_5_4[3].powi(3)
            * vars_5_4[4].cos()
        + 0.7 * vars_5_4[0] * vars_5_4[1] * vars_5_4[2] * vars_5_4[3] * vars_5_4[4];
    assert_approx_eq_with_debug(
        runtime_val_5_4,
        expected_5_4,
        1e-8,
        "5V5T #4 runtime vs expected",
    );
}
