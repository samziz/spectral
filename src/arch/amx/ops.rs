use core::{arch::asm, mem::MaybeUninit};

/// A register set exposed by AMX, i.e. a matrix:
///
/// * `X`: 8x64 matrix, 512 bytes total.
/// * `Y`: 8x64 matrix, 512 bytes total.
/// * `Z`: 64x64 matrix, 4096 bytes total.
///
/// These are generally addressed by register when storing (reading)
/// or loading (writing) data, but are generally addressed as whole
/// matrices when operating on mathematically.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum RegSet {
    X,
    Y,
    Z,
}

impl super::AmxHandle {
    /// ## Write ('load') ops
    ///
    /// These let the programmer copy information from main memory (in the
    /// form of heap-allocated Rust objects, or stack-allocated objects to
    /// be `alloca`ed) _to_ the AMX coprocessor.
    ///
    /// The 'load' terminology is relative to the coprocessor, not what is
    /// being passed in. (Compare the base ARM `LD64B`-type instructions.)

    /// ### Atomic write operations

    /// Write ('load') 64 bytes to register set `x`/`y`/`z` as a vector of
    /// shape 64x1. For `x` or `y`, the `reg` must be 0-7, for they have 8
    /// registers/rows each. For `z`, which has 64 not 8, it must be 0-63.
    pub fn set_vector(&self, set: RegSet, reg: u64, ptr: *const [u8]) {
        let op = match set {
            RegSet::X => 0,
            RegSet::Y => 1,
            RegSet::Z => 4,
        };
        let operand = Self::fmt_offset_ptr::<64>(reg, (ptr.cast::<u64>()) as u64);

        unsafe { Self::emit_op(op, operand) };
    }

    /// ### Non-atomic write operations

    /// Write 512 bytes to register set `x`/`y`, or 4096 to register set
    /// `z`, filling the entire matrix. Each register is 64 bytes width,
    /// for all 3 register sets. `x` and `y` have 8 rows, `z` has 64.
    pub fn set_matrix(&self, set: RegSet, data: &[u8]) {
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
        .for_each(|i| self.set_vector(set, i, &data[(i * 8) as usize..((i + 1) * 8) as usize] as *const [u8]))
    }

    /// ## Read ('store') ops
    ///
    /// These let the programmer copy information from an AMX coprocessor
    /// to main memory.
    ///
    /// 'Store' is worded relative to the coprocessor (cf 'load', above).

    /// Read ('store') 64 bytes from register set `x`/`y`/`z`, to a 64x1
    /// byte vector. For `x` and `y`, `reg` must 0..7. For `z`, 0..63.
    pub fn get_vector(&self, set: RegSet, reg: u64) -> [u8; 64] {
        let mut buf: [u8; 64] = unsafe { MaybeUninit::uninit().assume_init() };
        let ptr: *mut [u8; 64] = &mut buf;

        let operand = Self::fmt_offset_ptr::<64>(reg, ptr as u64);
        let op = match set {
            RegSet::X => 2,
            RegSet::Y => 3,
            RegSet::Z => 5,
        };

        unsafe {
            Self::emit_op(op, operand);
        }

        buf
    }

    /// Get a 512-byte, 8x64 matrix from register set `x`/`y` by name,
    /// one of the two smaller matrices. This loops over `get_vector`,
    /// but only allocates once, for a small time saving.
    pub fn get_matrix_512(&self, set: RegSet) -> [u8; 512] {
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
            Self::emit_op(
                op,
                // Safe: Bump `ptr` by 64 each time. 512 (`ptr` alloc size) / 8 (iters) = 64.
                Self::fmt_offset_ptr::<64>(reg, ptr.offset((reg * 64) as isize) as u64),
            );
        });

        buf
    }

    /// Get a 4096-byte, 64x64 matrix from register set `z`, the largest
    /// register set.
    pub fn get_matrix_4096(&self) -> [u8; 4096] {
        let mut buf: [u8; 4096] = unsafe { MaybeUninit::uninit().assume_init() };
        let ptr: *mut [u8; 4096] = &mut buf;

        (0..64).for_each(|reg| unsafe {
            Self::emit_op(
                5,
                // Safe: Bump `ptr` by 64 each time. 4096 (`ptr` alloc size) / 64 (iters) = 64.
                Self::fmt_offset_ptr::<64>(reg, unsafe { ptr.offset((reg * 64) as isize) } as u64),
            )
        });

        buf
    }

    /// ## Mathematical operations
    ///
    /// These operations take an input AMX register/matrix and write the
    /// output to another, or to the same, AMX register/matrix. Commonly
    /// they write to the `z` register, since outputs often >= inputs.

    /// Does matrix multiplication of `x` and `y`, writing the resulting
    /// matrix to `z`. The data are interpreted as 16-bit floats: 32x16.
    pub fn matrix_mul_f16(&self) {
        // Safe: This operation will simply result in an empty matrix if
        // the input matrices are empty. No possible input is invalid.
        unsafe { Self::emit_op(21, 0) }
    }

    /// Does matrix multiplication of `x` and `y`, writing the resulting
    /// matrix to `z`. The data are interpreted as 16-bit ints: 32x16.
    pub fn matrix_mul_i16(&self) {
        // Safe: This operation will simply result in an empty matrix if
        // the input matrices are empty. No possible input is invalid.
        unsafe { Self::emit_op(20, 0) }
    }

    /// Does matrix multiplication of `x` and `y`, *adding* the resulting
    /// matrix to `z`. The data are interpreted as 16-bit floats: 32x16.
    pub fn matrix_mul_add_f16(&self) {
        // Safe: This operation will simply result in an empty matrix if
        // the input matrices are empty. No possible input is invalid.
        unsafe { Self::emit_op(15, 0) }
    }

    /// Does matrix multiplication of `x` and `y`, *adding* the resulting
    /// matrix to `z`. The data are interpreted as 16-bit ints: 32x16.
    pub fn matrix_mul_add_i16(&self) {
        // Safe: This operation will simply result in an empty matrix if
        // the input matrices are empty. No possible input is invalid.
        unsafe { Self::emit_op(14, 0) }
    }

    /// # Housekeeping operations
    ///
    /// This includes the complementary `set` & `clr` operations required
    /// to initialise and de-initialise the AMX coprocessor respectively.

    /// Enables the AMX coprocessor. Must be called per thread before use.
    /// We don't swallow the `unsafe`, because there are possible invalid
    /// input conditions, which the caller is responsible for checking.
    pub(super) unsafe fn set() {
        Self::emit_op(17, 0)
    }

    /// Disables the AMX coprocessor. Must be called per thread after use.
    /// We don't swallow the `unsafe`, because there are possible invalid
    /// input conditions, which the caller is responsible for checking.
    pub unsafe fn clr(&self) {
        Self::emit_op(17, 1)
    }

    /// # Private functions
    ///
    /// These are the inner, private functions on which the rest of the
    /// `ops` module relies.

    /// Emit an AMX instruction, using an input register to accept the `op`
    /// (AMX opcode) and `operand` (the value, if applicable, else zero).
    unsafe fn emit_op(op: u8, operand: u64) {
        asm!(
            // The convention is: `0x00201000 | ((op & 0x1F) << 5) | (operand & 0x1F)`.
            // Note that this differs because we have to encode `operand` as (what the
            // processor interprets as) a hexadecimal number.
            // https://gist.github.com/dougallj/7a75a3be1ec69ca550e7c36dc75e0d6f#file-aarch64_amx-py-L53.
            ".word 0x00201000 + ({op} << 5) + (0{operand} & 0xf) + (0{operand} >> 4) * 10",
            op = in(reg) op,
            operand = in(reg) operand,
            options(nostack, preserves_flags),
        );
    }

    /// Emit an AMX instruction where `op` and `operand` are immediates:
    /// i.e. compile-time constants.
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
}
