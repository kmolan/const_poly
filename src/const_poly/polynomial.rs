use crate::const_poly::term::Term;

pub struct Polynomial<const N: usize, const M: usize> {
    terms: [Term<N>; M],
}

impl<const N: usize, const M: usize> Polynomial<N, M> {
    pub const fn new(terms: [Term<N>; M]) -> Self {
        Self { terms }
    }

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
