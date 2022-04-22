use super::bus;

/// A register set exposed by AMX, i.e. a matrix:
///
/// * `X`: 8x64 matrix, 512 bytes total.
/// * `Y`: 8x64 matrix, 512 bytes total.
/// * `Z`: 64x64 matrix, 4096 bytes total.
///
/// These are generally addressed by register when storing (reading)
/// or loading (writing) data, but are generally addressed as whole
/// matrices when operating on mathematically.
#[derive(Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum RegSet {
    X,
    Y,
    Z,
}

impl RegSet {
    const fn from_u8(u: u8) -> Self {
        match u {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            x => panic!("value {} not representable as RegSet", x),
        }
    }
}

pub struct XRegs;
impl Reg64x8<0> for ZRegs {}

pub struct F16X;
impl F16Ops<0> for F16X {}

pub struct YRegs;
impl Reg64x8<1> for ZRegs {}

trait Reg64x8<const R: u8> {
    /// Returns the matrix contents of this register set, as a 64x8
    /// 2D byte array. See [`super::ops::get_matrix_512`] for more.
    fn get_matrix(data: &[u8]) -> [u8; 512] {
        bus::get_matrix_512(const { RegSet::from_u8(R) })
    }

    /// Set the contents of this register set as a 64x8 matrix, from
    /// a 2D byte array. See [`super::ops::set_matrix`] for more.
    fn set_matrix(data: &[u8]) {
        bus::set_matrix(const { RegSet::from_u8(R) }, data);
    }
}

trait F16Ops<const R: u8> {
    /// Multiply this register by a given vector register `y`, treating
    /// both as 8x 16bit float vectors. The result is written to `x`.
    fn vec_mul_in_place() {
        let R2: u8 = const {
            match R {
                1 => 2,
                2 => 1,
            }
        };

        bus::matrix_mul_f16()
    }
}

trait I16Ops<const R: u8> {
    /// Multiply this register by a given vector register `y`, with
    /// the result stored in
    fn vec_mul_in_place() {
        let R2: u8 = const {
            match R {
                1 => 2,
                2 => 1,
            }
        };

        bus::matrix_mul_i16()
    }
}

pub struct ZRegs();
impl Reg64x64<2> for ZRegs {}

trait Reg64x64<const R: u8> {
    /// Returns the matrix contents of this register set, as a 64x64
    /// 2D byte array. See [`super::ops::get_matrix_4096`] for more.
    fn get_matrix(data: &[u8]) -> [u8; 4096] {
        bus::get_matrix_4096()
    }

    /// Set the contents of this register set as a 64x64 matrix, from
    /// a 2D byte array. See [`super::ops::set_matrix`] for more.
    fn set_matrix(data: &[u8]) {
        bus::set_matrix(const { RegSet::from_u8(R) }, data);
    }
}
