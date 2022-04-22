//! This module defines private functions (i.e. private to mod `amx`)
//! used for enqueueing instructions onto the address bus, to execute
//! on the AMX coprocessor.

use core::{arch::asm, mem::MaybeUninit};

use super::regs::RegSet;

/// Write 64 bytes to a vector register in set x/y (0-7) or z (0-63).
pub(super) fn set_vector(set: RegSet, reg: u64, ptr: *const [u8]) {
    let op = match set {
        RegSet::X => 0,
        RegSet::Y => 1,
        RegSet::Z => 4,
    };
    let operand = fmt_offset_ptr::<64>(reg, (ptr.cast::<u64>()) as u64);

    unsafe { emit_op(op, operand) };
}

/// Write 512 bytes to regset X/Y, or 4096 to Z. This is *not* atomic,
/// but iterates over all the vector registers: 8 for X/Y & 64 for Z.
pub fn set_matrix(set: RegSet, data: &[u8]) {
    match set {
        RegSet::X | RegSet::Y => {
            debug_assert!(data.len() == 512, "data must be [u8; 512] but was {}", data.len());
            0..8
        }
        RegSet::Z => {
            debug_assert!(data.len() == 4096, "data must be [u8; 4096] but was {}", data.len());
            0..64
        }
    }
    .for_each(|i| set_vector(set, i, &data[(i * 8) as usize..((i + 1) * 8) as usize] as *const [u8]))
}

/// Read 64 bytes from a vector register in set x/y (0-7) or z (0-63).
pub fn get_vector(set: RegSet, reg: u64) -> [u8; 64] {
    let mut buf: [u8; 64] = unsafe { MaybeUninit::uninit().assume_init() };
    let ptr: *mut [u8; 64] = &mut buf;

    let operand = fmt_offset_ptr::<64>(reg, ptr as u64);
    let op = match set {
        RegSet::X => 2,
        RegSet::Y => 3,
        RegSet::Z => 5,
    };

    unsafe {
        emit_op(op, operand);
    }

    buf
}

/// Read 512 bytes to regset X/Y, or 4096 from Z. This is *not* atomic,
/// but iterates over all the vector registers: 8 for X/Y & 64 for Z.
pub fn get_matrix_512(set: RegSet) -> [u8; 512] {
    let mut buf: [u8; 512] = unsafe { MaybeUninit::uninit().assume_init() };
    let ptr: *mut [u8; 512] = &mut buf;

    let op = match set {
        RegSet::X => 2,
        RegSet::Y => 3,
        RegSet::Z => {
            #[cfg(feature = "debug")]
            panic!("passed invalid regset `z` to `get_matrix_512`");
            return [0; 512];
        }
    };

    (0..8).for_each(|reg| unsafe {
        emit_op(
            op,
            // Safe: Bump `ptr` by 64 each time. 512 (`ptr` alloc size) / 8 (iters) = 64.
            fmt_offset_ptr::<64>(reg, ptr.offset((reg * 64) as isize) as u64),
        );
    });

    buf
}

/// Read a 4096-byte 64x64 matrix from regset Z, the largest of the 3.
pub fn get_matrix_4096() -> [u8; 4096] {
    let mut buf: [u8; 4096] = unsafe { MaybeUninit::uninit().assume_init() };
    let ptr: *mut [u8; 4096] = &mut buf;

    (0..64).for_each(|reg| unsafe {
        emit_op(
            5,
            // Safe: Bump `ptr` by 64 each time. 4096 (`ptr` alloc size) / 64 (iters) = 64.
            fmt_offset_ptr::<64>(reg, unsafe { ptr.offset((reg * 64) as isize) } as u64),
        )
    });

    buf
}

/// ## Mathematical ops
/// These ops take one/more register as input and one/more as output.

/// Matrix multiplies X and Y as float16, writing the product to `z`.
pub fn matrix_mul_f16() {
    // Safe: This operation will simply result in an empty matrix if
    // the input matrices are empty. No possible input is invalid.
    unsafe { emit_op(21, 0) }
}

/// Matrix multiplies X and Y as int16, writing the product to `z`.
pub fn matrix_mul_i16() {
    // Safe: This operation will simply result in an empty matrix if
    // the input matrices are empty. No possible input is invalid.
    unsafe { emit_op(20, 0) }
}

/// Matrix multiplies X and Y as float16, adding the product to `z`.
pub fn matrix_mul_add_f16() {
    // Safe: This operation will simply result in an empty matrix if
    // the input matrices are empty. No possible input is invalid.
    unsafe { emit_op(15, 0) }
}

/// Matrix multiplies X and Y as int16, adding the product to `z`.
pub fn matrix_mul_add_i16() {
    // Safe: This operation will simply result in an empty matrix if
    // the input matrices are empty. No possible input is invalid.
    unsafe { emit_op(14, 0) }
}

/// # Configuration ops

/// Enables the AMX coprocessor. Unsafe: Caller must manage state.
pub(super) unsafe fn set() {
    emit_op(17, 0)
}

/// Disables the AMX coprocessor. Unsafe: Caller must manage state.
pub unsafe fn clr() {
    emit_op(17, 1)
}

/// Enqueue an AMX instruction, passing `op` and `operand` via regs.
unsafe fn emit_op(op: u8, operand: u64) {
    asm!(
        // The convention is: `0x00201000 | ((op & 0x1F) << 5) | (operand & 0x1F)`.
        // Note: Formatting is strange, but means params parse correctly as numbers.
        // https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f#file-aarch64_amx-py-L53.
        ".word 0x00201000 + ({op} << 5) + (0{operand} & 0xf) + (0{operand} >> 4) * 10",
        op = in(reg) op,
        operand = in(reg) operand,
        options(nostack, preserves_flags),
    );
}

/// Enqueue an AMX instruction with immediate (constant) parameters.
unsafe fn op_imm<const OP: u8, const OPERAND: u8>() {
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
