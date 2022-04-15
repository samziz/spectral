/// A vector register in the `x` register set, 0..8. Each has
/// 64 bits, 512 across all 8. Like both others, this set may
/// be addressed individually or as a 512-bit chunk.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XVec(pub usize);

/// A vector register in the `y` register set, 0..8. Each has
/// 64 bits, 512 across all 8. Like both others, this set may
/// be addressed individually or as a 512-bit chunk.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YVec(pub usize);

/// A vector register in the `z` register set, 0..64. Each has
/// 64 bits, 4096 across all 64. Like both others, the set may
/// be addressed individually or as a 4096-bit chunk. Note the
/// `y` register set is considerably (8x) larger than x and y.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ZVec(pub usize);

/// A bit offset in the `x` register set, treated as a singly
/// addressable mass. There are 512 bits across 8 individual
/// registers.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct XBits(pub usize);

/// A bit offset in the `y` register set, treated as a singly
/// addressable mass. There are 512 bits across 8 individual
/// registers.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct YBits(pub usize);

/// A bit offset in the `z` register set, treated as a singly
/// addressable mass. There are 4096 bits across 64 individual
/// registers.
#[derive(Default, Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ZBits(pub usize);
