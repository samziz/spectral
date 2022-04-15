use std::arch::asm;
use std::marker::PhantomData;

/// Emit an AMX instruction with an input register.
pub unsafe fn op_in<const OP: u8>(operand: u64) {
    asm!(
        //
        // 0x00201000 | ((op & 0x1F) << 5) | (operand & 0x1F)
        //
        // See the unofficial docs:
        // https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f#file-aarch64_amx-py-L53
        //
        // Notes:
        // - '{op} << 5': Shift opcode to the eighth and final bit.
        // - for 'operand', we need to sneak it in as a num, so we rebuild it with:
        //   - '& 0xf': mod 16 / remainder over 16 (16= 10 in hex, so to speak)
        //   - '>> 4': div 16 to nearest int
        //   - take (mod16 * 10[i.e. 16]) + div16 to get the original
        ".word 0x00201000 + ({op} << 5) + (0{operand} & 0xf) + (0{operand} >> 4) * 10",
        op = const OP,
        operand = in(reg) operand,
        options(nostack, preserves_flags),
    );
}

/// Emit an AMX instruction with a 5-bit immediate.
pub unsafe fn op_imm<const OP: u8, const OPERAND: u8>() {
    asm!(
        ".word 0x00201000 + ({op} << 5) + {operand}",
        op = const OP,
        operand = const OPERAND,
        options(nostack, preserves_flags),
    );
}

pub unsafe fn ldx(v: u64) {
    op_in::<0>(v);
}

pub unsafe fn ldy(v: u64) {
    op_in::<1>(v);
}

pub unsafe fn stx(v: u64) {
    op_in::<2>(v);
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

unsafe fn ldx(&mut self, v: u64, ptr: *mut ()) {
    ldx(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn ldy(&mut self, v: u64, ptr: *mut ()) {
    ldy(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn stx(&mut self, v: u64, ptr: *mut ()) {
    stx(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn sty(&mut self, v: u64, ptr: *mut ()) {
    sty(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn ldz(&mut self, v: u64, ptr: *mut ()) {
    ldz(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn stz(&mut self, v: u64, ptr: *mut ()) {
    stz(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn ldzi(&mut self, v: u64, ptr: *mut ()) {
    ldzi(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
unsafe fn stzi(&mut self, v: u64, ptr: *mut ()) {
    stzi(v | (ptr as u64 & 0x00FF_FFFF_FFFF_FFFF));
}
fn extrx(&mut self, v: u64) {
    unsafe { extrx(v) };
}
fn extry(&mut self, v: u64) {
    unsafe { extry(v) };
}
fn fma64(&mut self, v: u64) {
    unsafe { fma64(v) };
}
fn fms64(&mut self, v: u64) {
    unsafe { fms64(v) };
}
fn fma32(&mut self, v: u64) {
    unsafe { fma32(v) };
}
fn fms32(&mut self, v: u64) {
    unsafe { fms32(v) };
}
fn mac16(&mut self, v: u64) {
    unsafe { mac16(v) };
}
fn fma16(&mut self, v: u64) {
    unsafe { fma16(v) };
}
fn fms16(&mut self, v: u64) {
    unsafe { fms16(v) };
}
fn vecint(&mut self, v: u64) {
    unsafe { vecint(v) };
}
fn vecfp(&mut self, v: u64) {
    unsafe { vecfp(v) };
}
fn matint(&mut self, v: u64) {
    unsafe { matint(v) };
}
fn matfp(&mut self, v: u64) {
    unsafe { matfp(v) };
}
fn genlut(&mut self, v: u64) {
    unsafe { genlut(v) };
}
