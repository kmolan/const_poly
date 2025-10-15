use crate::const_poly::function_approximations::*;

#[derive(Clone, Copy)]
pub enum VarFunction {
    Identity, // x
    Pow(u32), // x^n
    Sin,      // sin(x)
    Cos,      // cos(x)
    Tan,      // tan(x)
    Exp,      // exp(x)
}

#[derive(Copy, Clone)]
pub struct Term<const N: usize> {
    coeff: f64,
    functions: [VarFunction; N],
}

impl<const N: usize> Term<N> {
    pub const fn new(coefficient: f64, functions: [VarFunction; N]) -> Self {
        Self {
            coeff: coefficient,
            functions,
        }
    }

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
            };

            result *= value;
            i += 1;
        }

        result
    }
}
