use alloc::vec::Vec;

use super::Tensor;

pub struct Matrix<T>(Tensor<T>);

impl<T> Matrix<T> {
    /// Create a new [`Matrix`] from a 2D [`Vec`], parsing each slice
    /// **as a column**. Note: This consumes the vector you pass in.
    pub fn from_cols(md_arr: Vec<Vec<T>>) -> Self {
        Matrix(Tensor {
            data: Some(
                md_arr
                    .into_iter()
                    .flat_map(|m| m.into_iter())
                    .collect(),
            ),
            dims: [
                md_arr.first().map_or(0, |m| m.len()) as u16,
                md_arr.len() as u16,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        })
    }

    /// Create a new [`Matrix`] from a 2D [`Vec`], parsing each slice
    /// **as a row**. Note: This consumes the vector you pass in.
    pub fn from_rows(md_arr: Vec<Vec<T>>) -> Self {
        Matrix(Tensor {
            data: Some(
                // Loop through 2D array in _column_ order. For each row
                // index, for each col index, yield the num at the index.
                (0..md_arr.first().map_or(0, |m| m.len()))
                    .flat_map(|c| (0..md_arr.len()).map(|r| md_arr[r][c]))
                    .collect(),
            ),
            dims: [
                md_arr.len() as u16,
                md_arr.first().map_or(0, |m| m.len()) as u16,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        })
    }
}
