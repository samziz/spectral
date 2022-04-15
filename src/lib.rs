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
//! If you're unsure, then this is not for you.

#![no_std]
#![feature(asm)]
#![feature(asm_const)]
#![feature(inline_const)]
#![feature(stdsimd)]
#![feature(thread_local)]

#[cfg(feature = "iter")] pub mod iter;

mod arch;

pub struct Matrix<const N: usize, const M: usize>();

pub fn alloc_matrix<const N: usize, const M: usize>(matrix: Matrix<N, M>) -> Matrix<N, M> {
    matrix
}

#[test]
fn it_runs_without_error() {
    let _in = Matrix::<3, 3>();
    let _out = alloc_matrix(_in);
    assert_eq!(&_in as *const _, &_out as *const _,);
}
