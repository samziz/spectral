#![cfg_attr(feature = "doc_cfg", feature(doc_cfg))]

mod init;
mod load_store;
mod lookup_t;
mod ops;
mod regs;

use load_store::LoadStore;

use self::regs::ZRow;

/// A high-level wrapper for AMX instructions.
///
/// On a supported system, construct an [`AmxCtx`] by calling [`AmxCtx::new`].
/// `AmxCtx` derefs to [`amx::nativeops::AmxOps`], which implements [`AmxOps`]​
/// (the low-level wrapper), which has a blanket impl of `Amx`.
pub trait Amx: AmxOps {
    /// Load 512 bits (64 bytes) from memory to the specified register row.
    unsafe fn load512<T>(&mut self, ptr: *const T, row: impl LoadStore) {
        row.load512(self, ptr);
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
    unsafe fn load512_interleaved<T>(&mut self, ptr: *const T, row: ZRow) {
        load_store::load512_z_interleaved(self, ptr, row);
    }

    /// Store 512 bits (64 bytes) `z[index][0..64]` to memory with interleaving.
    ///
    /// `index` must be in range `0..64`.
    unsafe fn store512_interleaved<T>(&mut self, ptr: *mut T, row: ZRow) {
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
                    regs::XRow(i),
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
                    regs::YRow(i),
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
                    ZRow(i),
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
        x_offset_bytes: Option<regs::XBytes>,
        y_offset_bytes: Option<regs::YBytes>,
        z_index: ZRow,
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
        table: regs::XRow,
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

// Safe: Just forwarding the calls.
unsafe impl<T: ?Sized + AmxOps> AmxOps for &'_ mut T {
    /// Unsafe functions.

    unsafe fn ldx(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldx(x, ptr)
    }
    unsafe fn ldy(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldy(x, ptr)
    }
    unsafe fn stx(&mut self, x: u64, ptr: *mut ()) {
        (**self).stx(x, ptr)
    }
    unsafe fn sty(&mut self, x: u64, ptr: *mut ()) {
        (**self).sty(x, ptr)
    }
    unsafe fn ldz(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldz(x, ptr)
    }
    unsafe fn stz(&mut self, x: u64, ptr: *mut ()) {
        (**self).stz(x, ptr)
    }
    unsafe fn ldzi(&mut self, x: u64, ptr: *mut ()) {
        (**self).ldzi(x, ptr)
    }
    unsafe fn stzi(&mut self, x: u64, ptr: *mut ()) {
        (**self).stzi(x, ptr)
    }

    /// Safe functions.

    fn extrx(&mut self, x: u64) {
        (**self).extrx(x)
    }
    fn extry(&mut self, x: u64) {
        (**self).extry(x)
    }
    fn fma64(&mut self, x: u64) {
        (**self).fma64(x)
    }
    fn fms64(&mut self, x: u64) {
        (**self).fms64(x)
    }
    fn fma32(&mut self, x: u64) {
        (**self).fma32(x)
    }
    fn fms32(&mut self, x: u64) {
        (**self).fms32(x)
    }
    fn mac16(&mut self, x: u64) {
        (**self).mac16(x)
    }
    fn fma16(&mut self, x: u64) {
        (**self).fma16(x)
    }
    fn fms16(&mut self, x: u64) {
        (**self).fms16(x)
    }
    fn vecint(&mut self, x: u64) {
        (**self).vecint(x)
    }
    fn vecfp(&mut self, x: u64) {
        (**self).vecfp(x)
    }
    fn matint(&mut self, x: u64) {
        (**self).matint(x)
    }
    fn matfp(&mut self, x: u64) {
        (**self).matfp(x)
    }
    fn genlut(&mut self, x: u64) {
        (**self).genlut(x)
    }
}
