use crate::const_poly::function_approximations::*;

/// Enum representing the mathematical function applied to a variable in a term.
///
/// Each variant corresponds to a supported unary function that can be applied
/// to a variable within a polynomial term.
///
/// Variants:
/// - `Identity`: the variable itself (no function applied)
/// - `Pow(u32)`: power function `x^n` where `n` is the exponent
/// - `Sin`: sine function
/// - `Cos`: cosine function
/// - `Tan`: tangent function
/// - `Exp`: exponential function (e^x)
/// - `Ln`: natural logarithm
/// - `Sqrt`: square root function
/// - `Arctan`: arctangent function
/// - `Sinh`: hyperbolic sine function
/// - `Cosh`: hyperbolic cosine function
#[derive(Clone, Copy)]
pub enum VarFunction {
    Identity, // x
    Pow(i32), // x^n
    Sin,      // sin(x)
    Cos,      // cos(x)
    Tan,      // tan(x)
    Exp,      // exp(x)
    Ln,       // ln(x)
    Sqrt,     // sqrt(x)
    Arctan,   // arctan(x)
    Sinh,     // sinh(x)
    Cosh,     // cosh(x)
}

/// Represents a single term in a polynomial with N variables.
///
/// A term consists of a coefficient and an array of `VarFunction`s, each
/// applied to a corresponding variable in the input.
///
/// For example, for `N = 2`:
/// ```ignore
/// Term::new(3.0, [VarFunction::Sin, VarFunction::Pow(2)])
/// ```
/// represents the term `3 * sin(x_0) * (x_1)^2`.
#[derive(Copy, Clone)]
pub struct Term<const N: usize> {
    coeff: f64,
    functions: [VarFunction; N],
}

impl<const N: usize> Term<N> {
    /// Creates a new `Term` with the specified coefficient and functions.
    ///
    /// # Parameters
    ///
    /// - `coefficient`: The scalar multiplier for the term.
    /// - `functions`: An array of `VarFunction`s, one per variable.
    ///
    /// # Returns
    ///
    /// A new `Term` instance.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use crate::const_poly::term::{Term, VarFunction};
    /// let term = Term::<2>::new(2.0, [VarFunction::Sin, VarFunction::Pow(3)]);
    /// ```
    pub const fn new(coefficient: f64, functions: [VarFunction; N]) -> Self {
        Self {
            coeff: coefficient,
            functions,
        }
    }

    /// Evaluates the term for the given variables.
    ///
    /// Applies each function in `functions` to the corresponding variable,
    /// then multiplies all results together with the coefficient.
    ///
    /// # Parameters
    ///
    /// - `vars`: An array of variables of length `N`.
    ///
    /// # Returns
    ///
    /// The floating-point result of evaluating the term.
    ///
    /// # Example
    ///
    /// ```ignore
    /// # use crate::const_poly::term::{Term, VarFunction};
    /// # use crate::const_poly::function_approximations::*;
    /// let term = Term::<1>::new(2.0, [VarFunction::Sin]);
    /// let val = term.evaluate([1.57079632679]); // Approx sin(pi/2)
    /// assert!((val - 2.0).abs() < 1e-6);
    /// ```
    pub const fn evaluate(&self, vars: [f64; N]) -> f64 {
        let mut result = self.coeff;
        let mut i = 0;

        while i < N {
            let value = match self.functions[i] {
                VarFunction::Identity => vars[i],
                VarFunction::Pow(exp) => static_powi(vars[i], exp),
                VarFunction::Sin => sin_approx(vars[i]),
                VarFunction::Cos => cos_approx(vars[i]),
                VarFunction::Tan => tan_approx(vars[i]),
                VarFunction::Exp => exp_approx(vars[i]),
                VarFunction::Ln => ln_approx(vars[i]),
                VarFunction::Sqrt => sqrt_approx(vars[i]),
                VarFunction::Arctan => arctan_approx(vars[i]),
                VarFunction::Sinh => sinh_approx(vars[i]),
                VarFunction::Cosh => cosh_approx(vars[i]),
            };

            result *= value;
            i += 1;
        }

        result
    }
}
