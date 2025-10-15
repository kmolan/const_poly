pub const fn sin_approx(x: f64) -> f64 {
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

pub const fn cos_approx(x: f64) -> f64 {
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

pub const fn exp_approx(x: f64) -> f64 {
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

// tan(x) = sin(x)/cos(x)
pub const fn tan_approx(x: f64) -> f64 {
    sin_approx(x) / cos_approx(x)
}

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
