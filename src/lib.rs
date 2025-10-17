#![no_std]

pub mod function_approximations;
pub mod polynomial;
pub mod term;

#[macro_use]
pub mod macros;

#[cfg(test)]
extern crate static_assertions; //for const_assert

// Re-export key types
pub use crate::polynomial::Polynomial;
pub use crate::term::Term;

pub use crate::term::VarFunction;