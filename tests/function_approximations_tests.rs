use const_poly::const_poly::function_approximations;
use static_assertions::const_assert;

const fn approx_eq(a: f64, b: f64, epsilon: f64) -> bool {
    let diff = if a > b { a - b } else { b - a };
    diff < epsilon
}

#[test]
fn test_sin_approx() {
    const MAX_ERROR: f64 = 1e-9;
    const PI: f64 = core::f64::consts::PI;

    // Test over many values: -2π to 2π in small steps
    for i in -1000..=1000 {
        let x = (i as f64) * 2.0 * PI / 1000.0; // Step size: (2π) / 1000
        let actual = x.sin();
        let approx = function_approximations::sin_approx(x);
        let error = (actual - approx).abs();

        assert!(
            error < MAX_ERROR,
            "Failed at x = {}: sin_approx = {}, std::sin = {}, error = {}",
            x,
            approx,
            actual,
            error
        );
    }
}

#[test]
fn test_cos_approx() {
    const MAX_ERROR: f64 = 1e-9;
    const PI: f64 = core::f64::consts::PI;

    // Test over many values: -2π to 2π in small steps
    for i in -1000..=1000 {
        let x = (i as f64) * 2.0 * PI / 1000.0; // Step size: (2π) / 1000
        let actual = x.cos();
        let approx = function_approximations::cos_approx(x);
        let error = (actual - approx).abs();

        assert!(
            error < MAX_ERROR,
            "Failed at x = {}: cos_approx = {}, std::cos = {}, error = {}",
            x,
            approx,
            actual,
            error
        );
    }
}

#[test]
fn test_exp_approx() {
    const MAX_ERROR: f64 = 1e-10;

    // Test exact zero input
    assert!(
        (function_approximations::exp_approx(0.0) - 1.0).abs() < MAX_ERROR,
        "exp_approx(0) failed"
    );

    // Test a range of positive values
    for i in 0..=1000 {
        let x = (i as f64) * 0.01; // 0.0 to 10.0 in 0.01 increments
        let actual = x.exp();
        let approx = function_approximations::exp_approx(x);
        let error = (actual - approx).abs();
        assert!(
            error < MAX_ERROR,
            "exp_approx({}) = {}, actual = {}, error = {}",
            x,
            approx,
            actual,
            error
        );
    }

    // Test a range of negative values
    for i in 0..=1000 {
        let x = -(i as f64) * 0.01; // 0.0 to -10.0 in -0.01 increments
        let actual = x.exp();
        let approx = function_approximations::exp_approx(x);
        let error = (actual - approx).abs();
        assert!(
            error < MAX_ERROR,
            "exp_approx({}) = {}, actual = {}, error = {}",
            x,
            approx,
            actual,
            error
        );
    }
}

#[test]
fn test_tan_approx() {
    const MAX_ERROR: f64 = 1e-7;
    const PI: f64 = core::f64::consts::PI;
    const HALF_PI: f64 = core::f64::consts::FRAC_PI_2;

    // Test over many values: -2π to 2π in small steps
    for i in -1000..=1000 {
        let x = (i as f64) * 2.0 * PI / 1000.0; // Step size: (2π) / 1000

        // Skip values too close to vertical asymptotes of tan(x)
        let k = (x / HALF_PI).round();
        let distance_to_asymptote = (x - k * HALF_PI).abs();
        let is_odd_k = (k as i64) % 2 != 0;

        if is_odd_k && distance_to_asymptote < 0.05 {
            continue; // skip near ±π/2, ±3π/2, etc.
        }

        let actual = x.tan();
        let approx = function_approximations::tan_approx(x);
        let error = (actual - approx).abs();

        assert!(
            error < MAX_ERROR,
            "Failed at x = {}: tan_approx = {}, std::tan = {}, error = {}",
            x,
            approx,
            actual,
            error
        );
    }
}

#[test]
fn test_static_powi() {
    const MAX_ERROR: f64 = 1e-50;

    // Compile-time asserts for trivial cases
    const_assert!((function_approximations::static_powi(2.0, 0) - 1.0).abs() < MAX_ERROR);
    const_assert!((function_approximations::static_powi(5.0, 1) - 5.0).abs() < MAX_ERROR);
    const_assert!((function_approximations::static_powi(3.0, -1) - (1.0 / 3.0)).abs() < MAX_ERROR);
    const_assert!((function_approximations::static_powi(1.0, 100) - 1.0).abs() < MAX_ERROR);
    const_assert!((function_approximations::static_powi(-2.0, 2) - 4.0).abs() < MAX_ERROR);

    // Test exponent zero (any base^0 = 1)
    assert!(
        (function_approximations::static_powi(2.0, 0) - 1.0).abs() < MAX_ERROR,
        "static_powi(2.0, 0) failed"
    );
    assert!(
        (function_approximations::static_powi(0.0, 0) - 1.0).abs() < MAX_ERROR,
        "static_powi(0.0, 0) failed"
    );

    let mut base_i: f64 = 0.1;
    while base_i <= 10.0 {
        for exp in -20..=20 {
            let expected = base_i.powi(exp);
            let actual = function_approximations::static_powi(base_i, exp);
            let error = (expected - actual).abs();

            assert!(
                error < MAX_ERROR,
                "static_powi({}, {}) = {}, expected {}, error {}",
                base_i,
                exp,
                actual,
                expected,
                error
            );
        }

        base_i += 0.1;
    }
}

#[test]
fn test_ln_approx() {
    const MAX_ERROR: f64 = 1e-15;

    // Helper closure for approximate equality
    let approx_eq = |a: f64, b: f64| (a - b).abs() < MAX_ERROR;

    // Trivial cases with expected hardcoded values
    let trivial_cases = &[
        (1.0, 0.0),
        (2.0, core::f64::consts::LN_2),
        (0.5, -core::f64::consts::LN_2),
    ];

    for &(input, expected) in trivial_cases {
        let res = function_approximations::ln_approx(input);
        assert!(
            approx_eq(res, expected),
            "ln_approx({}) = {}, expected {}",
            input,
            res,
            expected
        );
    }

    let test_values = &[
        0.7, 0.9, 1.1, 1.3, // around 1
        10.0, 50.0, 100.0, 1e6, // larger values
        1e-6, 1e-3, 0.1, 0.4, // smaller positive values
    ];

    for &x in test_values {
        let res = function_approximations::ln_approx(x);
        let expected = x.ln();
        assert!(
            approx_eq(res, expected),
            "ln_approx({}) = {}, expected {}",
            x,
            res,
            expected
        );
    }

    // Test invalid inputs (<= 0)
    assert!(
        function_approximations::ln_approx(0.0).is_nan(),
        "function_approximations::ln_approx(0.0) should be NaN"
    );
    assert!(
        function_approximations::ln_approx(-1.0).is_nan(),
        "function_approximations::ln_approx(-1.0) should be NaN"
    );
}

#[test]
fn test_sqrt_approx() {
    const MAX_ERROR: f64 = 1e-10;

    // Test zero
    assert!(
        function_approximations::sqrt_approx(0.0) == 0.0,
        "sqrt_approx({}) = {}, expected {}",
        0.0,
        0.0,
        0.0
    );

    // Test one
    assert!(
        function_approximations::sqrt_approx(1.0) == 1.0,
        "sqrt_approx({}) = {}, expected {}",
        1.0,
        1.0,
        1.0
    );

    // Test positive numbers
    for &val in &[
        1e-6, 1e-4, 0.001, 0.01, 0.05, 0.1, 0.25, 0.5, 0.75, 1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 7.5,
        9.0, 10.0, 16.0, 25.0, 36.0, 50.0, 64.0, 100.0, 250.0, 512.0, 1_000.0, 1e4, 1e6, 1e9,
    ] {
        let res = function_approximations::sqrt_approx(val);
        let expected = val.sqrt();
        let error = (res - expected).abs();
        assert!(
            error < MAX_ERROR,
            "sqrt_approx({}) = {}, expected {}, error {}",
            val,
            res,
            expected,
            error
        );
    }

    // Test negative input returns NaN
    let val = -4.0;
    let res = function_approximations::sqrt_approx(val);
    assert!(
        res.is_nan(),
        "sqrt_approx({}) should be NaN, got {}",
        val,
        res
    )
}

#[test]
fn test_arctan_approx() {
    const MAX_ERROR: f64 = 1e-15;

    let approx_eq = |a: f64, b: f64| (a - b).abs() < MAX_ERROR;

    // Test zero
    let res = function_approximations::arctan_approx(0.0);
    assert!(
        approx_eq(res, 0.0),
        "arctan_approx(0.0) = {}, expected 0.0",
        res
    );

    let test_values: &[f64] = &[
        // Positive small
        0.1,
        0.5,
        0.9,
        0.99,
        // Negative small
        -0.1,
        -0.5,
        -0.9,
        -0.99,
        // Large positive and negative
        1.0,
        2.0,
        5.0,
        10.0,
        100.0,
        -1.0,
        -2.0,
        -5.0,
        -10.0,
        -100.0,
        // Edge near ±1
        0.9999999999,
        -0.9999999999,
        1.0000000001,
        -1.0000000001,
    ];

    for &x in test_values {
        let res = function_approximations::arctan_approx(x);
        let expected = x.atan();
        assert!(
            approx_eq(res, expected),
            "arctan_approx({}) = {}, expected {}, error {}",
            x,
            res,
            expected,
            (res - expected).abs()
        );
    }
}

#[test]
fn test_sinh_approx() {
    const MAX_ERROR: f64 = 1e-10;

    // Test a variety of values, positive and negative
    let test_values = [
        0.0, 1e-10, -1e-10, 0.1, -0.1, 0.5, -0.5, 1.0, -1.0, 2.0, -2.0, 5.0, -5.0, 10.0, -10.0,
    ];

    for &x in &test_values {
        let approx = function_approximations::sinh_approx(x);
        let expected = x.sinh();
        let error = (approx - expected).abs();

        assert!(
            approx_eq(approx, expected, MAX_ERROR),
            "sinh_approx({}) = {}, expected {}, error {}",
            x,
            approx,
            expected,
            error
        );
    }
}
