use crate::term::Term;

/// Represents a polynomial with a fixed number of variables and terms.
///
/// The polynomial is defined as a sum of `NUM_TERMS` terms, where each term
/// operates on `NUM_VARIABLES` variables. Each term is an instance of `Term<N>`,
/// encapsulating a coefficient and a set of variable functions.
///
/// # Type Parameters
///
/// - `NUM_VARIABLES`: Number of variables in the polynomial.
/// - `NUM_TERMS`: Number of terms in the polynomial.
///
/// # Example
///
/// ```
/// use const_poly::VarFunction::*;
/// use const_poly::{const_poly, Polynomial};
///
/// // Create a polynomial, f(x,y) = 3 * sin(x) * cos(y)
/// const POLY: Polynomial<2, 1> = const_poly!({[3.0, [Sin, Cos]]});
///
/// // Evaluate at x = π/2, y = 0 => 3 * sin(π/2) * cos(0) = 3 * 1 * 1 = 3
/// let result = POLY.evaluate([1.57079632679, 0.0]);
/// assert!((result - 3.0).abs() < 1e-6);
/// ```
pub struct Polynomial<const NUM_VARIABLES: usize, const NUM_TERMS: usize> {
    terms: [Term<NUM_VARIABLES>; NUM_TERMS],
}

impl<const NUM_VARIABLES: usize, const NUM_TERMS: usize> Polynomial<NUM_VARIABLES, NUM_TERMS> {
    /// Creates a new `Polynomial` from an array of terms.
    ///
    /// # Parameters
    ///
    /// - `terms`: An array containing `NUM_TERMS` terms, each of which operates on `NUM_VARIABLES` variables.
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
    /// const POLY: Polynomial<2, 2> = const_poly!({[1.0, [Identity, Identity]],
    ///                                             [2.0, [Sin,      Cos]]});
    /// ```
    pub const fn new(terms: [Term<NUM_VARIABLES>; NUM_TERMS]) -> Self {
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
    pub const fn evaluate(&self, vars: [f64; NUM_VARIABLES]) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;

        while i < NUM_TERMS {
            sum += self.terms[i].evaluate(vars);
            i += 1;
        }

        sum
    }
}
