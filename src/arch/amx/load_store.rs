use super::{
    regs::{XVec, YVec, ZVec},
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

/// Register row types supporting 512- and 1024-bit operations.
///
/// [`Amx`] should be used as a wrapper, rather than calling this directly.
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

impl LoadStore for XVec {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldx(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.stx(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldx(encode(index as u64, MemSize::_128), ptr as *mut ());
    }

    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.stx(encode(index as u64, MemSize::_128), ptr as *mut ());
    }
}

impl LoadStore for YVec {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldy(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.sty(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.ldy(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 8);
        ops.sty(encode(index as u64, MemSize::_128), ptr as *mut ());
    }
}

impl LoadStore for ZVec {
    unsafe fn load512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.ldz(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn store512<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.stz(encode(index as u64, MemSize::_64), ptr as *mut ());
    }

    unsafe fn load1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *const T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.ldz(encode(index as u64, MemSize::_128), ptr as *mut ());
    }

    unsafe fn store1024_aligned<T>(&self, ops: &mut (impl AmxOps + ?Sized), ptr: *mut T) {
        let index = self.0;
        debug_assert!(index < 64);
        ops.stz(encode(index as u64, MemSize::_128), ptr as *mut ());
    }
}

/// Load 512 bits (64 bytes) from memory to `z[index][0..64]` with interleaving.
///
/// `index` must be in range `0..64`.
pub unsafe fn load512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *const T,
    ZVec(index): ZVec,
) {
    debug_assert!(index < 64);
    ops.ldzi(encode(index as u64, MemSize::_64), ptr as *mut ());
}

/// Store 512 bits (64 bytes) `ptr[offset][0..64]` to memory with interleaving.
///
/// `index` must be in range `0..64`.
pub unsafe fn store512_z_interleaved<T>(
    ops: &mut (impl AmxOps + ?Sized),
    ptr: *mut T,
    ZVec(index): ZVec,
) {
    debug_assert!(index < 64);
    ops.stzi(encode(index as u64, MemSize::_64), ptr as *mut ());
}
