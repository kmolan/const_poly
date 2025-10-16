#![no_std]

mod function_approximations;
pub mod polynomial;
pub mod term;

#[cfg(test)]
extern crate static_assertions; //for const_assert

#[cfg(test)]
mod function_approximations_tests;