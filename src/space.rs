use alloc::boxed::Box;
use alloc::vec::Vec;
use core::num::NonZeroU16 as NzU16;

/// An ordered set on which mathematical ops are defined.
/// Column major for storage, and e.g. when iterating.
pub struct Tensor<T> {
    data: Option<Vec<T>>,
    /// Dimensionality of the tensor.
    // 8x nz u16s for dimension lens - fits in 2 words,
    // enforces nonzeroity, and easy to expand.
    dims: [NzU16; 8],
}

/// A raw multidimensional array of a tensor's contents.
pub type TensorData<T> = Box<[T]>;

/// ## Accessors
impl<T> Tensor<T> {
    pub fn data(&self) -> Option<Vec<T>> {
        self.data
    }
}

/// ## Shape methods
impl<T> Tensor<T> {
    /// Get the dimensions of this tensor.
    pub fn dims(&self) -> [NzU16; 8] {
        self.dims
    }

    /// Get the horizontal length of this tensor.
    pub fn hlen(&self) -> usize {
        u16::from(self.dims[1]) as usize
    }

    /// Get the vertical length of this tensor.
    pub fn vlen(&self) -> usize {
        u16::from(self.dims[0]) as usize
    }

    /// Get the length for a numbered, **zero-indexed** dimension.
    pub fn len_for(&self, d: usize) -> u16 {
        self.dims[d].into()
    }
}

impl<T> Tensor<T> {}
