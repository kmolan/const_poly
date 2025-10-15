use crate::const_poly::term::Term;

/// Represents a polynomial with a fixed number of variables and terms.
///
/// The polynomial is defined as a sum of `M` terms, where each term
/// operates on `N` variables. Each term is an instance of `Term<N>`,
/// encapsulating a coefficient and a set of variable functions.
///
/// # Type Parameters
///
/// - `N`: Number of variables in the polynomial.
/// - `M`: Number of terms in the polynomial.
///
/// # Example
///
/// ```ignore
/// use crate::const_poly::term::{Term, VarFunction};
/// use crate::const_poly::polynomial::Polynomial;
///
/// // Define a term 3 * sin(x) * cos(y)
/// const TERM1: Term<2> = Term::new(3.0, [VarFunction::Sin, VarFunction::Cos]);
///
/// // Create a polynomial with a single term
/// const POLY: Polynomial<2, 1> = Polynomial::new([TERM1]);
///
/// // Evaluate at x = π/2, y = 0 => 3 * sin(π/2) * cos(0) = 3 * 1 * 1 = 3
/// let result = POLY.evaluate([1.57079632679, 0.0]);
/// assert!((result - 3.0).abs() < 1e-6);
/// ```
pub struct Polynomial<const N: usize, const M: usize> {
    terms: [Term<N>; M],
}

impl<const N: usize, const M: usize> Polynomial<N, M> {
    /// Creates a new `Polynomial` from an array of terms.
    ///
    /// # Parameters
    ///
    /// - `terms`: An array containing `M` terms, each of which operates on `N` variables.
    ///
    /// # Returns
    ///
    /// A new `Polynomial` instance encapsulating the given terms.
    ///
    /// # Example
    ///
    /// ```ignore
    /// const TERMS: [Term<2>; 2] = [
    ///     Term::new(1.0, [VarFunction::Identity, VarFunction::Identity]),
    ///     Term::new(2.0, [VarFunction::Sin, VarFunction::Cos]),
    /// ];
    /// const POLY: Polynomial<2, 2> = Polynomial::new(TERMS);
    /// ```
    pub const fn new(terms: [Term<N>; M]) -> Self {
        Self { terms }
    }

    /// Evaluates the polynomial at the given variable values.
    ///
    /// This computes the sum of the evaluations of all constituent terms.
    ///
    /// # Parameters
    ///
    /// - `vars`: An array of `N` floating-point values representing the variables.
    ///
    /// # Returns
    ///
    /// The floating-point result of evaluating the polynomial.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = poly.evaluate([x_value, y_value, ...]);
    /// ```
    pub const fn evaluate(&self, vars: [f64; N]) -> f64 {
        let mut sum = 0.0;
        let mut i = 0;

        while i < M {
            sum += self.terms[i].evaluate(vars);
            i += 1;
        }

        sum
    }
}
