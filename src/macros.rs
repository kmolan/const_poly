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
    ({
        $( [ $coeff:expr, [ $( $func:expr ),* $(,)? ] ] ),* $(,)?
    }) => {{
        const __N: usize = {
            let _ = [ $( [ $( $func ),* ] ),* ];
            $crate::count_exprs!( $( $($func),* ),* ) / $crate::count_exprs!( $( [ $( $func ),* ] ),* )
        };
        const __M: usize = $crate::count_exprs!( $( [ $( $func ),* ] ),* );

        $crate::polynomial::Polynomial::<__N, __M>::new($crate::terms! {
            $( [ $coeff, [ $( $func ),* ] ] ),*
        })
    }};
}
