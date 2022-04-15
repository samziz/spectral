mod load_store;
mod lookup_t;
mod ops;
mod regs;

use core::cell::Cell;

use self::load_store::LoadStore;
use self::regs::ZVec;

// AMX must be enabled before use, but should only be enabled one
// time per thread. We check this before initialising an instance
// of [`AmxHandle`], to enforce this invariant.
#[thread_local]
static AMX_ENABLED: Cell<bool> = Cell::new(false);

/// A handle represents an initialised AMX instance in this thread.
/// It is scoped to a particular thread and thus specifically does
/// not implement either [`Send`] or [`Sync`].
pub struct AmxHandle;

/// This error type is returned by [`Amx::new`], and represents any
/// error conditions which prevent us from initialising AMX.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AmxErr {
    /// This thread has already initialised AMX.
    Exists,
    /// This target does not support AMX.
    Unsupported,
}

impl AmxHandle {
    /// Construct a brand new instance of [`AmxHandle`] by enabling AMX
    /// for the current thread. Note: This is the only way to obtain an
    /// [`AmxHandle`], which guarantees the init-each-thread invariant.
    pub fn enable() -> Result<Self, AmxErr> {
        if cfg!(not(all(
            target_arch = "aarch64",
            target_os = "macos",
            target_pointer_width = "64"
        ))) {
            // Target is not compatible. Return an Err().
            Err(AmxErr::Unsupported)
        } else {
            use std::arch::is_aarch64_feature_detected;

            if !is_aarch64_feature_detected!("asimd") {
                // Fail. This is otherwise compatible, but we're
                // told the 'advanced SIMD' exts are unsupported.
                Err(AmxErr::Unsupported)
            } else {
                if AMX_ENABLED.with(|x| x.get()) {
                    // AMX is already enabled. Return an Err().
                    Err(AmxErr::Exists)
                } else {
                    // Safe: We finally know that AMX is supported and
                    // not already enabled ITT, so enable it.
                    unsafe { ops::set() };

                    Ok(Self)
                }
            }
        }
    }

    /// Disable AMX for the current thread. This must be private, and
    /// must be an instance method, so we can count on the invariant
    /// that it cannot be called without AMX having been initialised.
    fn disable(self) {
        // Safe: AMX is supported & has been initialised (see above).
        unsafe { ops::clr() };
        AMX_ENABLED.with(|x| x.set(false));
    }
}

impl Drop for AmxHandle {
    fn drop(&mut self) {
        self.disable();
    }
}

/// A high-level wrapper for AMX instructions.
///
/// On a supported system, construct an [`AmxCtx`] by calling [`AmxCtx::new`].
/// `AmxCtx` derefs to [`amx::nativeops::AmxOps`], which implements [`AmxOps`]​
/// (the low-level wrapper), which has a blanket impl of `Amx`.
pub trait Amx: AmxOps {
    /// Load 512 bits (64 bytes) from memory to the specified register row.
    unsafe fn load512<T>(&mut self, offset: usize, ptr: *const T) {
        debug_assert!(index < 8);

        ops::ldx(encode(offset, size), ptr);
    }

    /// Load 1024 bits (128 bytes) from memory to the specified register row
    /// and the subsequent one. The specified pointer must be aligned to
    /// 128-byte boundaries.
    unsafe fn load1024_aligned<T>(&mut self, ptr: *const T, row: impl LoadStore) {
        row.load1024_aligned(self, ptr);
    }

    /// Store 512 bits (64 bytes) from the specified register row's contents
    /// to main memory.
    unsafe fn store512<T>(&mut self, ptr: *mut T, row: impl LoadStore) {
        row.store512(self, ptr);
    }

    /// Store 1024 bits (128 bytes) the specified register row and the
    /// subsequent one's contents to memory.  The specified pointer must be
    /// aligned to  128-byte boundaries.
    unsafe fn store1024_aligned<T>(&mut self, ptr: *mut T, row: impl LoadStore) {
        row.store1024_aligned(self, ptr);
    }

    /// Load 512 bits (64 bytes) from memory to `z[index][0..64]` with interleaving.
    ///
    /// `index` must be in range `0..64`.
    unsafe fn load512_interleaved<T>(&mut self, ptr: *const T, row: ZVec) {
        load_store::load512_z_interleaved(self, ptr, row);
    }

    /// Store 512 bits (64 bytes) `z[index][0..64]` to memory with interleaving.
    ///
    /// `index` must be in range `0..64`.
    unsafe fn store512_interleaved<T>(&mut self, ptr: *mut T, row: ZVec) {
        load_store::store512_z_interleaved(self, ptr, row);
    }

    /// Read the whole contents of `x`.
    fn read_x(&mut self) -> [u8; 512] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..8 {
            // Safe: Writing in a memory region within `ret`.
            unsafe {
                self.store512(
                    (ret.as_mut_ptr() as *mut u8).offset(i as isize * 64),
                    regs::XVec(i),
                )
            };
        }
        // Safe: All elements are initialized.
        unsafe { ret.assume_init() }
    }

    /// Read the whole contents of `y`.
    fn read_y(&mut self) -> [u8; 512] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..8 {
            // Safe: Writing in a memory region within `ret`.
            unsafe {
                self.store512(
                    (ret.as_mut_ptr() as *mut u8).offset(i as isize * 64),
                    regs::YVec(i),
                )
            };
        }
        // Safe: All elements are initialized.
        unsafe { ret.assume_init() }
    }

    /// Read the whole contents of `z`.
    fn read_z(&mut self) -> [u8; 4096] {
        let mut ret = std::mem::MaybeUninit::uninit();
        for i in 0..64 {
            // Safe: Writing in a memory region within `ret`.
            unsafe {
                self.store512(
                    (ret.as_mut_ptr() as *mut u8).offset(i as isize * 64),
                    ZVec(i),
                )
            };
        }
        // Safe: All elements are initialized.
        unsafe { ret.assume_init() }
    }

    /// Calculate the outer product of `x: [i16; 32]` and `y: [i16; 32]` and write
    /// the output to every second row of `z: [[i16; 32]; 64]`.
    ///
    /// If `x_offset_bytes` and/or `y_offset_bytes` are `None`, their respective
    /// registers will be excluded from the op (not performing multiplication).
    ///
    /// `z_index` must be in range `0..64`. Only the least significant bits will
    /// be taken into consideration.
    fn outer_product_i16_xy_to_z(
        &mut self,
        x_offset_bytes: Option<regs::XBits>,
        y_offset_bytes: Option<regs::YBits>,
        z_index: ZVec,
        accumulate: bool,
    ) {
        let z_index = z_index.0;
        debug_assert!(x_offset_bytes.unwrap_or_default().0 < 0x200);
        debug_assert!(y_offset_bytes.unwrap_or_default().0 < 0x200);
        debug_assert!(z_index < 64);
        // @todo widening (i32 output)
        // @todo vector output (reducing)
        self.mac16(
            (y_offset_bytes.unwrap_or_default().0
                | (x_offset_bytes.unwrap_or_default().0 << 10)
                | (z_index << 20)
                | (((!accumulate) as usize) << 27)
                | ((x_offset_bytes.is_none() as usize) << 28)
                | ((y_offset_bytes.is_none() as usize) << 29)) as u64,
        );
    }

    /// Perform (reverse) table lookup.
    fn lut(
        &mut self,
        input: impl lookup_t::LutIn,
        table: regs::XVec,
        output: impl lookup_t::LutOut,
        ty: impl lookup_t::LookupT,
    ) {
        lookup_t::lut(self, input, table, output, ty);
    }
}

impl<T: AmxOps + ?Sized> Amx for T {}

/// Exposes all AMX instructions except `set` and `clr` as trait methods.
///
/// Load and store operations receive a pointer by the additional parameter to
/// allow emulation on a system with a different pointer size.
pub unsafe trait AmxOps {
    /// Unsafe functions.

    unsafe fn ldx(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldy(&mut self, x: u64, ptr: *mut ());
    unsafe fn stx(&mut self, x: u64, ptr: *mut ());
    unsafe fn sty(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldz(&mut self, x: u64, ptr: *mut ());
    unsafe fn stz(&mut self, x: u64, ptr: *mut ());
    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ());
    unsafe fn stzi(&mut self, x: u64, ptr: *mut ());

    /// Safe functions.

    fn extrx(&mut self, x: u64);
    fn extry(&mut self, x: u64);
    fn fma64(&mut self, x: u64);
    fn fms64(&mut self, x: u64);
    fn fma32(&mut self, x: u64);
    fn fms32(&mut self, x: u64);
    fn mac16(&mut self, x: u64);
    fn fma16(&mut self, x: u64);
    fn fms16(&mut self, x: u64);
    fn vecint(&mut self, x: u64);
    fn vecfp(&mut self, x: u64);
    fn matint(&mut self, x: u64);
    fn matfp(&mut self, x: u64);
    fn genlut(&mut self, x: u64);
}
