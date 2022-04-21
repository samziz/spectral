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

//! Spectral is written with `no_std` so that you don't need to use
//! std. It also tends to light up most heap allocations in luminol.
#![no_std]
//! It does rely on 5 features, 4 to do with asm and const generics,
//! and `thread_local` to enable that macro in `core` (the std lib).
//! All of these are performance aids, and are encouraged anyway.
#![feature(asm)]
#![feature(asm_const)]
#![feature(inline_const)]
#![feature(thread_local)]

#[cfg(feature = "iter")] pub mod iter;

mod arch;

/// A matrix capable of performant algebraic operations. Its actual
/// implementation will differ based on the target triple.
pub struct Matrix {
    /// Height of the matrix (`r` = rows).
    h: usize,
    /// Width of the matrix (`c` = columns).
    w: usize,
}

/// The actual contents of a matrix are defined as a separate type,
/// optional on the matrix. This is done for several reasons: first
/// because it allows us to allocate the matrix data solely in high
/// performance registers like SIMD/AMX if need be (whereas Rust by
/// default allocates in main memory), but secondly also because we
/// may want to store the matrix in a more compressed format, using
/// some kind of [decomposition][0].
///
/// [0]: https://en.wikipedia.org/wiki/Matrix_decomposition
pub struct MatrixArrayedData<const H: usize, const W: usize> {
    rows: [[u64; H]; W],
}

impl Matrix {
    /// Get a new [`Matrix`], for N (height) and M (height). If these
    /// values are known at compile time, prefer `new_const`.
    pub fn new(n: usize, m: usize) -> Self {
        Matrix { h: n, w: m }
    }

    /// Get a new [`Matrix`] at compile time, iff you know N (height)
    /// and M (width) statically. If so, prefer this over `new`.
    pub const fn new_const<const N: usize, const M: usize>() -> Self {
        Matrix { h: N, w: M }
    }

    /// ## Operations

    /// Multiply this matrix by another matrix, or by a vector. We do
    /// not implement [`std::ops::Mul`] because it cannot guarantee
    /// static dispatch.
    pub fn multiply(y: Matrix) {
        let amx = crate::arch::amx::AmxHandle::get()
            .unwrap_or_else(|e| panic!("failed to acquire AMX handle: {:?}", e));
        
        let amx_a = amx.set_matrix(, data)
        amx.matrix_mul_f16()
    }

    pub fn get_amx_register(size: ) {

    }
}

#[test]
fn it_runs_without_error() {
    let x = Matrix::new(50, 50);
    let y = Matrix::new(50, 50);
    assert_eq!(&_in as *const _, &_out as *const _,);
}
