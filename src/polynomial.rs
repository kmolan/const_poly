use crate::term::Term;

/// Represents a polynomial with a fixed number of variables and terms.
///
/// The polynomial is defined as a mathematical operation on `NUM_VARIABLES` variables. Each term is an instance of `Term<N>`,
/// encapsulating a coefficient and a set of variable functions.
///
/// # Type Parameters
///
/// - `NUM_VARIABLES`: Number of variables in the polynomial.
///
/// # Example
///
/// ```
/// use const_poly::VarFunction::*;
/// use const_poly::{const_poly, Polynomial};
///
/// // Create a polynomial, f(x,y) = 3 * sin(x) * cos(y)
/// const POLY: Polynomial<2> = const_poly!({[3.0, Sin, Cos]});
///
/// // Evaluate at x = π/2, y = 0 => 3 * sin(π/2) * cos(0) = 3 * 1 * 1 = 3
/// let result = POLY.evaluate(&[1.57079632679, 0.0]);
/// assert!((result - 3.0).abs() < 1e-6);
/// ```
pub struct Polynomial<const NUM_VARIABLES: usize> {
    terms: &'static [Term<NUM_VARIABLES>],
}

impl<const NUM_VARIABLES: usize> Polynomial<NUM_VARIABLES> {
    /// Creates a new `Polynomial` from an array of terms.
    ///
    /// # Parameters
    ///
    /// - `NUM_VARIABLES`: Number of variables in the polynomial.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` instance encapsulating the given terms.
    ///
    /// # Example
    ///
    /// ```
    /// use const_poly::VarFunction::*;
    /// use const_poly::{const_poly, Polynomial};
    ///
    /// //define polynomial f(x,y) = x*y + 2*Sin(x)*Sin(y)
    /// const POLY: Polynomial<2> = const_poly!({[1.0, Identity, Identity],
    ///                                          [2.0, Sin,      Cos]});
    /// ```
    pub const fn new(terms: &'static [Term<NUM_VARIABLES>]) -> Self {
        Self { terms }
    }

    /// Evaluates the polynomial at the given variable values.
    ///
    /// This computes the sum of the evaluations of all constituent terms.
    ///
    /// # Parameters
    ///
    /// - `vars`: An array of `NUM_VARIABLES` floating-point values representing the variables.
    ///
    /// # Returns
    ///
    /// The floating-point result of evaluating the polynomial.
    pub const fn evaluate(&self, vars: &[f64; NUM_VARIABLES]) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;

        while i < self.terms.len() {
            sum += self.terms[i].evaluate(vars);
            i += 1;
        }
        sum
    }
}

/// --- Special case: Single-variable ---
impl Polynomial<1> {
    /// Allows direct evaluation using a single `f64` value instead of an array.
    ///
    /// # Example
    /// ```
    /// use const_poly::VarFunction::*;
    /// use const_poly::{const_poly, Polynomial};
    ///
    /// // f(x) = 2 * sin(x)
    /// const POLY: Polynomial<1> = const_poly!([2.0, Sin]);
    ///
    /// // Evaluate directly with a single float
    /// let result = POLY.evaluate_scalar(1.57079632679); // ≈ 2.0
    /// assert!((result - 2.0).abs() < 1e-6);
    /// ```
    pub const fn evaluate_scalar(&self, x: f64) -> f64 {
        self.evaluate(&[x])
    }
}
