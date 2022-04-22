//! Contains the algebraic structures used for this lib. [`Tensor`] is
//! the composable base, while [`Matrix`] and co are convenience types.

/// An ordered set on which mathematical ops are defined. [`Vector`],
/// [`Matrix`], etc are the high-level interfaces built on this.
pub struct Tensor<T, const D: usize>
where
    [(); D - 1]:,
{
    data: Option<TensorData<T, D>>,
}

/// A raw multidimensional array of a tensor's contents.
pub type TensorData<T, const D: usize>
where
    [(); D - 1]:,
= [[T; D - 1]; D];
