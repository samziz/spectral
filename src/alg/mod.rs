//! This module reexports all the trait impls in submodules such
//! as [`arith`]. It is concerned with the core linear algebraic
//! operations.

mod arith;

pub use arith::*;
