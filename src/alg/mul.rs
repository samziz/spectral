use crate::struc::Tensor;

pub trait Multiply {}

impl<const D: usize> core::ops::Mul for Tensor<u8, D>
where
    [(); D - 1]:,
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {}
}

pub fn mul_by_vec(v: ) {

}
