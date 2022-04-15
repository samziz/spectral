use core::arch::asm;
use core::marker::PhantomData;

#[repr(u8)]
pub enum RegSet {
    X,
    Y,
    Z,
}

/// Load 64 bits from register `x`/`y`, to a 64x64bit array.
pub fn ldx(reg: u64, data: &[u8; 64]) -> () {
    let ptr: *const [u64; 64] = &data;
    let operand = fmt_offset_ptr::<64>(reg, ptr as u64);
    unsafe { op_in::<0>(operand) };
}

pub unsafe fn ldy(reg: u64, data: &[u8; 64]) {
    let ptr: *const [u64; 64] = &data;
    let operand = fmt_offset_ptr::<64>(reg, ptr as u64);
    unsafe { op_in::<1>(operand) };
}

pub fn stx(reg: u64) -> [u8; 64] {
    let ptr: *mut [u64; 64] = &mut [0; 64];

    let operand = fmt_offset_ptr::<64>(reg, ptr as u64);

    unsafe {
        op_in::<2>(operand);
    }

    ptr
}

pub unsafe fn sty(v: u64) {
    op_in::<3>(v);
}

pub unsafe fn ldz(v: u64) {
    op_in::<4>(v);
}

pub unsafe fn stz(v: u64) {
    op_in::<5>(v);
}

pub unsafe fn ldzi(v: u64) {
    op_in::<6>(v);
}

pub unsafe fn stzi(v: u64) {
    op_in::<7>(v);
}

pub unsafe fn extrx(v: u64) {
    op_in::<8>(v);
}

pub unsafe fn extry(v: u64) {
    op_in::<9>(v);
}

pub unsafe fn fma64(v: u64) {
    op_in::<10>(v);
}

pub unsafe fn fms64(v: u64) {
    op_in::<11>(v);
}

pub unsafe fn fma32(v: u64) {
    op_in::<12>(v);
}

pub unsafe fn fms32(v: u64) {
    op_in::<13>(v);
}

pub unsafe fn mac16(v: u64) {
    op_in::<14>(v);
}

pub unsafe fn fma16(v: u64) {
    op_in::<15>(v);
}

pub unsafe fn fms16(v: u64) {
    op_in::<16>(v);
}

pub unsafe fn set() {
    op_imm::<17, 0>();
}

pub unsafe fn clr() {
    op_imm::<17, 1>();
}

pub unsafe fn vecint(v: u64) {
    op_in::<18>(v);
}

pub unsafe fn vecfp(v: u64) {
    op_in::<19>(v);
}

pub unsafe fn matint(v: u64) {
    op_in::<20>(v);
}

pub unsafe fn matfp(v: u64) {
    op_in::<21>(v);
}

pub unsafe fn genlut(v: u64) {
    op_in::<22>(v);
}

/// Exposes the target processor's AMX support by implementing [`AmxOps`] trait.
///
/// [`AmxOps`]: crate::ops::AmxOps
///
/// The lifetime parameter can be used to restrict the use of `&AmxOps` to
/// a scope where AMX is enabled. To make this technique effective, this type
/// does not implement `Clone`. (GAT would make this more effective.)
///
/// This type is not `Send`-able because AMX states, including whether it's
/// enabled, are thread-local.
pub struct AmxOps<'a>(PhantomData<(&'a mut (), *mut ())>);

impl<'a> AmxOps<'a> {
    /// Allocate a new `AmxOps` struct. To use this, its invariants must obtain:
    ///
    ///   - The target processor must actually support AMX.
    ///   - AMX has been initialised in this thread.
    ///
    /// This is handled by calling [`Amx::new`].
    #[inline]
    pub const unsafe fn new() -> Self {
        Self(PhantomData)
    }

    /// Reborrow `self`, constructing a new `AmxOps` with a narrower lifetime.
    #[inline]
    pub const fn borrow_mut(&mut self) -> AmxOps<'_> {
        Self(PhantomData)
    }
}

/// # Primitive functions
///
/// These are the inner, private functions on which the rest of the
/// `ops` module relies.

/// Emit an AMX instruction with an input register.
fn op_in<const OP: u8>(operand: u64) {
    asm!(
        // The convention is: `0x00201000 | ((op & 0x1F) << 5) | (operand & 0x1F)`.
        // Note that this differs because we have to encode `operand` as (what the
        // processor interprets as) a hexadecimal number.
        // https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f#file-aarch64_amx-py-L53.
        ".word 0x00201000 + ({op} << 5) + (0{operand} & 0xf) + (0{operand} >> 4) * 10",
        op = const OP,
        operand = in(reg) operand,
        options(nostack, preserves_flags),
    );
}

/// Emit an AMX instruction with a 5-bit immediate.
fn op_imm<const OP: u8, const OPERAND: u8>() {
    asm!(
        ".word 0x00201000 + ({op} << 5) + {operand}",
        op = const OP,
        operand = const OPERAND,
        options(nostack, preserves_flags),
    );
}

/// Encode the offset and size into one 64bit int, as is required by
/// the undocumented AMX API:
fn fmt_offset<const SIZE: u64>(offset: u64) -> u64 {
    debug_assert!(offset < 64);

    (offset << 56) | (SIZE << 62)
}

/// Encode the offset and size AND pointer into one 64bit int, as is
/// required by the undocumented AMX API:
fn fmt_offset_ptr<const SIZE: u64>(offset: u64, ptr: u64) -> u64 {
    debug_assert!(offset < 64);

    (offset << 56) | (SIZE << 62) | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF)
}
