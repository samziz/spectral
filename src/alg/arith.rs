use core::{intrinsics, ops};

use crate::invar::{Float, Int, Scalar};
use crate::space::Tensor;

impl<S> core::ops::Add for Tensor<S>
where
    S: ops::Add<Output = S>,
{
    type Output = Self;

    /// Returns a tensor of the same proportions as the LHS, not the RHS,
    /// consistent with the principles of linear algebra. If the RHS has
    /// fewer dimensions than the LHS, RHS will be repeated for each of
    /// those dimensions; this has some memory implications, but minor.
    fn add(self, rhs: Self) -> Self::Output {
        // Naive implementation. We attempt to exploit processor features before this.
        if let (Some(lhs_d), Some(rhs_d)) = (self.data(), rhs.data()) {
            let (lhs_d) = match self.data() {
                Some(d) => d,
                None => panic!("could not obtain `.data` on lhs"),
            };

            let (rhs_d) = match rhs.data() {
                Some(d) => d,
                None => panic!("could not obtain `.data` on rhs"),
            };

            Tensor::<S> {
                data: Some(
                    lhs_d
                        .iter()
                        .zip(rhs_d.iter().cycle())
                        .map(|(&s1, &s2)| s1 + s2)
                        .collect(),
                ),
                dims: self.dims(),
            }
        } else {
            unreachable!()
        }
    }
}

impl<S> core::ops::Mul for Tensor<S>
where
    S: ops::Mul<Output = S>,
{
    type Output = Self;

    /// Returns a tensor of the same proportions as the LHS, not the RHS,
    /// consistent with the principles of linear algebra. If the RHS has
    /// fewer dimensions than the LHS, RHS will be repeated for each of
    /// those dimensions; this has some memory implications, but minor.
    fn mul(self, rhs: Self) -> Self::Output {
        // Naive implementation. We attempt to exploit processor features before this.
        if let (Some(lhs_d), Some(rhs_d)) = (self.data(), rhs.data()) {
            let (lhs_d) = match self.data() {
                Some(d) => d,
                None => panic!("missing tensor data on lhs"),
            };

            let (rhs_d) = match rhs.data() {
                Some(d) => d,
                None => panic!("missing tensor data on rhs"),
            };

            Tensor::<S> {
                data: Some(
                    lhs_d
                        .iter()
                        .zip(rhs_d.iter().cycle())
                        .map(|(&s1, &s2)| s1 * s2)
                        .collect(),
                ),
                dims: self.dims(),
            }
        } else {
            unreachable!()
        }
    }
}
