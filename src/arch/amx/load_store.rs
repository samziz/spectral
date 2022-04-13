use super::{
    regs::{XRow, YRow, ZRow},
    AmxOps,
};

/// A parameter of AMX's load and store instructions.
#[derive(Copy, Clone)]
#[repr(u8)]
enum MemSize {
    /// 64 bytes.
    _64 = 0,
    /// 128 bytes.
    _128 = 1,
}

impl MemArgs {
    #[inline]
    fn encode(offset: u64, size: MemSize) -> u64 {
        debug_assert!(offset < 64);

        (offset << 56) | ((size as u64) << 62)
    }
}

/// Register row types supporting 512- and 1024-bit operations.
///
/// [`Amx`] should be used as a wrapper, rather than calling this directly.
///
/// [`Amx`]: crate::Amx
pub trait LoadStore {
    /// Load 512 bits (64 bytes) from memory to the register.
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T);

    /// Store 512 bits (64 bytes) to memory from the register.
    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T);

    /// Load 1024 bits (128 bytes) from memory to the register. `ptr` must be
    /// aligned to 128-byte boundaries.
    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T);

    /// Store 1024 bits (128 bytes) to memory from the register. `ptr` must
    /// be aligned to 128-byte boundaries.
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T);
}

impl LoadStore for XRow {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.stx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    #[inline(always)]
    #[track_caller]
    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.stx(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

impl LoadStore for YRow {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldy(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.sty(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldy(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.sty(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

impl LoadStore for ZRow {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.ldz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.stz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_64,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.ldz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }

    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.stz(
            MemArgs {
                reg_offset: index as u64,
                size: MemSize::_128,
            }
            .encode(),
            ptr as *mut (),
        );
    }
}

/// Load 512 bits (64 bytes) from memory to `z[index][0..64]` with interleaving.
///
/// `index` must be in range `0..64`.
pub unsafe fn load512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *const T,
    ZRow(index): ZRow,
) {
    debug_assert!(index < 64);
    ops.ldzi(
        MemArgs {
            reg_offset: index as u64,
            size: MemSize::_64,
        }
        .encode(),
        ptr as *mut (),
    );
}

/// Store 512 bits (64 bytes) `z[index][0..64]` to memory with interleaving.
///
/// `index` must be in range `0..64`.
pub unsafe fn store512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *mut T,
    ZRow(index): ZRow,
) {
    debug_assert!(index < 64);
    ops.stzi(
        MemArgs {
            reg_offset: index as u64,
            size: MemSize::_64,
        }
        .encode(),
        ptr as *mut (),
    );
}
