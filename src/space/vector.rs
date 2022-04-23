use alloc::vec::Vec;

use super::Tensor;

pub struct Vector<T>(Tensor<T>);

impl<T> Vector<T> {
    /// Create a new [`Vector`] from a plain Rust [`Vec`]. Note: This
    /// consumes the vector that you pass in.
    pub fn from(arr: Vec<T>) -> Self {
        Vector(Tensor {
            data: Some(arr),
            dims: [arr.len() as u16, 0, 0, 0, 0, 0, 0, 0],
        })
    }
}
