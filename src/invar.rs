use core::{marker, ops};

/// A [`Scalar`] is a type on which the basic arithmetic operations
/// apply. Mathematically it is a field. Technically it is a number
/// type, in Rust the standard `u`, `i`, and `f` types, though it
/// will also apply to any additional fields you use or depend on.
pub(crate) trait Scalar =
    ops::Add + ops::Sub + ops::Div + ops::Mul<Output = Self> + marker::Copy + marker::Sized;

pub(crate) trait Float {}
impl Float for f32 {}
impl Float for f64 {}

pub(crate) trait Int {}
impl Int for i8 {}
impl Int for i16 {}
impl Int for i32 {}
impl Int for i64 {}
impl Int for i128 {}
