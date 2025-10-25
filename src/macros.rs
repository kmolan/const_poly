/// Macro to count the number of expressions.
#[macro_export]
macro_rules! count_exprs {
    () => { 0 };
    ($head:expr $(, $tail:expr)* $(,)?) => {
        1 + $crate::count_exprs!( $($tail),* )
    };
}

/// Macro to construct an array of `Term<N>` instances.
#[macro_export]
macro_rules! terms {
    ( $( [ $coeff:expr, [ $( $func:expr ),* $(,)? ] ] ),* $(,)? ) => {
        [
            $(
                $crate::term::Term::new($coeff, [ $( $func ),* ])
            ),*
        ]
    };
}

/// Defines a compile-time polynomial with inferred variable count.
/// Supports both single-term and multi-term forms.
///
/// ## Examples
///
/// ```
/// use const_poly::{Polynomial, const_poly, VarFunction::*};
///
/// const POLY1: Polynomial<2> = const_poly!([1.0, Sin, Cos]);
///
/// const POLY2: Polynomial<2> = const_poly!({
///     [1.0, Identity, Identity],
///     [2.0, Sin, Pow(2)]
/// });
/// ```
#[macro_export]
macro_rules! const_poly {
    // Single-term form: [ coeff, func1, func2, ... ]
    ( [ $coeff:expr $(, $func:expr )* $(,)? ] ) => {{
        const __N: usize = $crate::count_exprs!( $( $func ),* );
        const __TERMS: [ $crate::term::Term<__N>; 1 ] = [
            $crate::term::Term::new($coeff, [ $( $func ),* ])
        ];
        $crate::polynomial::Polynomial::<__N>::new(&__TERMS)
    }};

    // Multi-term form: { [coeff, func1, func2, ...], [coeff2, func1, ...], ... }
    ({
        [ $coeff_first:expr $(, $func_first:expr )* ]
        $(, [ $coeff_rest:expr $(, $func_rest:expr )* ] )* $(,)?
    }) => {{
        const __N: usize = $crate::count_exprs!( $( $func_first ),* );
        const __TERMS: &[ $crate::term::Term<__N> ] = &[
            $crate::term::Term::new($coeff_first, [ $( $func_first ),* ]),
            $( $crate::term::Term::new($coeff_rest, [ $( $func_rest ),* ]) ),*
        ];
        $crate::polynomial::Polynomial::<__N>::new(__TERMS)
    }};
}
