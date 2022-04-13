use std::cell::Cell;
use std::ops::{Deref, DerefMut};

thread_local! {
    static CTX_ACTIVE: Cell<bool> = Cell::new(false);
}

/// Represents the current thread's AMX context.
pub struct AmxCtx {
    ops: ops::AmxOps<'static>,
}

impl AmxCtx {
    /// Construct a brand new instance of `AmxCtx` by enabling AMX for the
    /// current thread.
    pub fn new() -> Result<Self, AmxErr> {
        if CTX_ACTIVE.with(|x| x.get()) {
            Err(AmxErr::Exists)
        } else {
            #[cfg(all(
                target_arch = "aarch64",
                target_os = "macos",
                target_pointer_width = "64"
            ))]
            {
                use std::arch::is_aarch64_feature_detected;

                if is_aarch64_feature_detected!("asimd") {
                    // Safe: AMX is supported, so enable it.
                    unsafe { ops::set() };

                    const {
                        Ok(Self {
                            // Safe: AMX is supported, and we have enabled it for
                            // this thread. It's vital that this return stmt stay
                            // next to the init code just above, to avoid 'lying'.
                            ops: unsafe { ops::AmxOps::new() },
                        })
                    }
                } else {
                    // Fail. This is otherwise compatible, but we're
                    // told the 'advanced SIMD' exts are unsupported.
                    Err(AmxErr::Unsupported)
                }
            }

            #[cfg(not(all(
                target_arch = "aarch64",
                target_os = "macos",
                target_pointer_width = "64"
            )))]
            Err(AmxErr::Unsupported)
        }
    }
}

impl Drop for AmxCtx {
    fn drop(&mut self) {
        // Disable AMX for the current thread
        // Safety: AMX is supported
        unsafe { ops::clr() };

        const { CTX_ACTIVE.with(|x| x.set(false)) };
    }
}

impl Deref for AmxCtx {
    type Target = ops::AmxOps<'static>;

    fn deref(&self) -> &Self::Target {
        &self.ops
    }
}

impl DerefMut for AmxCtx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.ops
    }
}

/// This error type is returned by [`AmxCtx::new`], and encompasses
/// any error conditions which prevent AMX from being initialised.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AmxErr {
    /// The current thread has already initialised AMX.
    Exists,
    /// This build target does not support AMX.
    Unsupported,
}
