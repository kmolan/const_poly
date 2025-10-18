/// Macro to count the number of expressions (used to infer N and M).
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

/// Macro that expands to a `Polynomial<N, M>` expression with inferred type.
#[macro_export]
macro_rules! const_poly {
    // Single term form: [ coeff, func1, func2, ... ]
    ( [ $coeff:expr $(, $func:expr )* $(,)? ] ) => {{
        const __N: usize = $crate::count_exprs!( $( $func ),* );
        const __M: usize = 1;
        $crate::polynomial::Polynomial::<__N, __M>::new([
            $crate::term::Term::new($coeff, [ $( $func ),* ])
        ])
    }};

    // Multi-term form: { [ coeff, func1, func2, ... ], [ coeff2, func1, ... ], ... }
    ({
        [ $coeff_first:expr $(, $func_first:expr )* ]
        $(, [ $coeff_rest:expr $(, $func_rest:expr )* ] )* $(,)?
    }) => {{
        const __N: usize = $crate::count_exprs!( $( $func_first ),* );
        const __M: usize = 1 + $crate::count_exprs!( $( [ $coeff_rest $(, $func_rest )* ] ),* );

        $crate::polynomial::Polynomial::<__N, __M>::new($crate::terms! {
            [ $coeff_first, [ $( $func_first ),* ] ],
            $( [ $coeff_rest, [ $( $func_rest ),* ] ] ),*
        })
    }};
}
