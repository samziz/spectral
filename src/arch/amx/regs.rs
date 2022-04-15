/// Refers to a row (register) in the `x` register set.
///
/// The row index must be in range `0..8`.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XVec(pub usize);

/// A vector register in the `y` register set, in range 0..8.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YVec(pub usize);

/// A vector register in the `z` matrix (register set),
/// which has 64 registers 0..64.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ZVec(pub usize);

/// A bit offset in the `x` register set, which has 8
/// 64-bit registers. The range is thus 512.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XBits(pub usize);

/// A bit offset in the `y` register set, which has 8
/// 64-bit registers. The range is thus 512.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YBits(pub usize);

/// A bit offset in the `z` register set, which has 64
/// 64-bit registers. The range is thus 4096.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ZBits(pub usize);
