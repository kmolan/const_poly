use const_poly::VarFunction::*;
use const_poly::{Polynomial, const_poly};

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
    // (1.5 * sin(x₀) * x₁ * x₂²) +
    // (-2.0 * cos(x₀) * x₁³ * x₂) +
    // (0.5 * exp(x₀) * ln(x₁) * sqrt(x₂))
    const POLY_3V_3T: Polynomial<3> = const_poly!({
        [1.5, Sin, Identity, Pow(2)],
        [-2.0, Cos, Pow(3), Identity],
        [0.5, Exp, Ln, Sqrt]
    });

    // (3.0 * x₀ * sin(x₁) * x₂² * cos(x₃)) +
    // (-1.2 * x₀³ * tan(x₁) * exp(x₂) * x₃) +
    // (0.7 * ln(x₀) * sqrt(x₁) * atan(x₂) * sinh(x₃)) +
    // (1.1 * cosh(x₀) * x₁ * x₂ * sin(x₃))
    const POLY_4V_4T: Polynomial<4> = const_poly!({
        [3.0, Identity, Sin, Pow(2), Cos],
        [-1.2, Pow(3), Tan, Exp, Identity],
        [0.7, Ln, Sqrt, Arctan, Sinh],
        [1.1, Cosh, Identity, Pow(1), Sin]
    });

    // (2.0 * sin(x₀) * cos(x₁) * x₂² * x₃ * ln(x₄)) +
    // (-3.3 * exp(x₀) * x₁³ * tan(x₂) * sqrt(x₃) * x₄) +
    // (1.7 * atan(x₀) * sinh(x₁) * cosh(x₂) * x₃ * x₄) +
    // (0.9 * x₀ * x₁ * x₂ * x₃ * x₄) +
    // (-0.5 * x₀ * x₁ * x₂ * x₃ * x₄)
    const POLY_5V_5T: Polynomial<5> = const_poly!({
        [2.0, Sin, Cos, Pow(2), Identity, Ln],
        [-3.3, Exp, Pow(3), Tan, Sqrt, Identity],
        [1.7, Arctan, Sinh, Cosh, Identity, Pow(1)],
        [0.9, Identity, Identity, Identity, Identity, Identity],
        [-0.5, Pow(1), Pow(1), Pow(1), Pow(1), Pow(1)]
    });

    // (1.1 * cos(x₀) * exp(x₁) * x₂ * ln(x₃) * sqrt(x₄)) +
    // (-2.2 * x₀² * sin(x₁) * x₂ * cosh(x₃) * x₄³) +
    // (0.8 * tan(x₀) * atan(x₁) * sinh(x₂) * x₃² * x₄) +
    // (-0.7 * x₀ * ln(x₁) * cos(x₂) * x₃ * exp(x₄)) +
    // (1.5 * x₀ * x₁ * x₂ * x₃ * x₄)
    const POLY_5V_5T_2: Polynomial<5> = const_poly!({
        [1.1, Cos, Exp, Pow(1), Ln, Sqrt],
        [-2.2, Pow(2), Sin, Identity, Cosh, Pow(3)],
        [0.8, Tan, Arctan, Sinh, Pow(2), Identity],
        [-0.7, Identity, Ln, Cos, Pow(1), Exp],
        [1.5, Pow(1), Pow(1), Pow(1), Pow(1), Pow(1)]
    });

    // (-1.3 * sinh(x₀) * x₁³ * x₂ * ln(x₃) * cos(x₄)) +
    // (2.4 * exp(x₀) * x₁ * sin(x₂) * cosh(x₃) * x₄²) +
    // (-0.9 * atan(x₀) * x₁ * x₂² * tan(x₃) * sqrt(x₄)) +
    // (1.0 * x₀ * x₁ * x₂ * x₃ * x₄) +
    // (-0.6 * x₀ * cos(x₁) * exp(x₂) * x₃³ * sin(x₄))
    const POLY_5V_5T_3: Polynomial<5> = const_poly!({
        [-1.3, Sinh, Pow(3), Identity, Ln, Cos],
        [2.4, Exp, Pow(1), Sin, Cosh, Pow(2)],
        [-0.9, Arctan, Identity, Pow(2), Tan, Sqrt],
        [1.0, Pow(1), Pow(1), Pow(1), Pow(1), Pow(1)],
        [-0.6, Identity, Cos, Exp, Pow(3), Sin]
    });

    // (2.2 * x₀ * sqrt(x₁) * ln(x₂) * sin(x₃) * exp(x₄)) +
    // (-1.8 * x₀³ * cos(x₁) * x₂ * tan(x₃) * x₄²) +
    // (1.3 * cosh(x₀) * x₁ * x₂ * atan(x₃) * x₄) +
    // (-0.4 * x₀ * x₁² * sinh(x₂) * x₃³ * cos(x₄)) +
    // (0.7 * x₀ * x₁ * x₂ * x₃ * x₄)
    const POLY_5V_5T_4: Polynomial<5> = const_poly!({
        [2.2, Pow(1), Sqrt, Ln, Sin, Exp],
        [-1.8, Pow(3), Cos, Identity, Tan, Pow(2)],
        [1.3, Cosh, Pow(1), Pow(1), Arctan, Pow(1)],
        [-0.4, Identity, Pow(2), Sinh, Pow(3), Cos],
        [0.7, Pow(1), Pow(1), Pow(1), Pow(1), Pow(1)]
    });

    // --- Test 1: 3 variables, 3 terms ---
    let vars_3 = [1.0_f64, 2.0, 3.0];
    let runtime_val_3 = POLY_3V_3T.evaluate(&vars_3);
    const COMPILE_VAL_3: f64 = POLY_3V_3T.evaluate(&[1.0, 2.0, 3.0]);
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
    let runtime_val_4 = POLY_4V_4T.evaluate(&vars_4);
    const COMPILE_VAL_4: f64 = POLY_4V_4T.evaluate(&[0.5, 1.5, 2.5, 3.5]);
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
    let runtime_val_5 = POLY_5V_5T.evaluate(&vars_5);
    const COMPILE_VAL_5: f64 = POLY_5V_5T.evaluate(&[0.1, 1.2, 2.3, 3.4, 4.5]);
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
    let runtime_val_5_2 = POLY_5V_5T_2.evaluate(&vars_5_2);
    const COMPILE_VAL_5_2: f64 = POLY_5V_5T_2.evaluate(&[0.9, 1.1, 1.3, 1.5, 1.7]);
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
    let runtime_val_5_3 = POLY_5V_5T_3.evaluate(&vars_5_3);
    const COMPILE_VAL_5_3: f64 = POLY_5V_5T_3.evaluate(&[1.2, 1.4, 1.6, 1.8, 2.0]);
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
    let runtime_val_5_4 = POLY_5V_5T_4.evaluate(&vars_5_4);
    const COMPILE_VAL_5_4: f64 = POLY_5V_5T_4.evaluate(&[0.8, 1.0, 1.2, 1.4, 1.6]);
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
