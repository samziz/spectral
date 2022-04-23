//! A library for fast, approximate algebraic computations on tensors.
//! In contrast to existing libraries, our focus is on exploiting the
//! advanced vector processing instructions of modern CPUs.
//!
//! You should use this if:
//!
//! - You are targeting Intel, AMD, or Apple hardware.
//! - Most of your work is pure computation on vectors or matrices.
//! - You want to exploit advanced extensions: SIMD, NEON, AMX, etc.
//!
//! **If you don't know if this is what you need, then it's not.**

//! Spectral is built with `no_std` so a user can build with `no_std`.
//! It's as simple as that. For context on forgoing `std`, see [here][0].
//!
//! [0]: https://docs.rust-embedded.org/book/intro/no-std.html
#![no_std]
//! We allow `incomplete_features` in order to unblock the unstable
//! feature `generic_const_exprs` (of which more below).
#![allow(incomplete_features)]
//! It does rely on 7 features, 6 for asm & const generics, 1 being
//! `thread_local` to export that macro from [`core`]. All are perf
//! or ergonomics wins anyway.
#![feature(asm)]
#![feature(asm_const)]
#![feature(core_intrinsics)]
#![feature(generic_const_exprs)]
#![feature(inline_const)]
#![feature(thread_local)]
#![feature(trait_alias)]

/// We enable the `alloc` crate, as well as `core`, so that the std
/// lib is available
extern crate alloc;

pub mod alg;
#[cfg(feature = "iter")] pub mod iter;

mod arch;
mod invar;
mod space;

/// Trait impls of mathematical operations over tensors.
pub use alg::*;
/// Algebraic types on which all other logic operates.
pub use space::{Matrix, Tensor, Vector};
