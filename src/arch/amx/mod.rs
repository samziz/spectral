mod ops;

use core::cell::Cell;

/// This module is a low-level wrapper over the M1's AMX coprocessor,
/// for fast large linear algebra over vectors and matrices. Its use
/// is simple: obtain an [`AmxHandle`] by calling `acquire()`.

/// AMX must be enabled before use, but should only be enabled one
/// time per thread. We check this before initialising an instance
/// of [`AmxHandle`], to enforce this invariant.
#[thread_local]
static HANDLE: Cell<Option<AmxHandle>> = Cell::new(None);

/// An error returned by [`AmxHandle::get`], representing failure
/// modes which prevent us from initialising AMX.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[non_exhaustive]
pub enum AmxErr {
    /// The target triple does not support AMX. Unless otherwise
    /// specified, this is the machine compiling the code.
    Incompatible,
}

/// A handle represents an initialised AMX instance in this thread.
/// It is scoped to a particular thread and thus specifically does
/// not implement [`Send`] or [`Sync`]. This is a zero-sized type,
/// and therefore does not consume any memory besides a flicked bit
/// within the [`Option<T>`] that inevitably contains it.
///
/// - [`self::ops`] implements the instructions.
pub struct AmxHandle;

impl AmxHandle {
    /// Obtain an [`AmxHandle`] by enabling AMX for this thread. This
    /// [`AmxHandle`] can then be used to run AMX instructions. This
    /// ensures that the only way to use the AMX processor is via the
    /// path that enables it - and checks it wasn't already enabled.
    pub fn get() -> Result<Self, AmxErr> {
        #[cfg(not(all(target_arch = "aarch64", target_os = "macos", target_pointer_width = "64")))]
        {
            // Target is not compatible. Return an Err().
            Err(AmxErr::Incompatible)
        }

        #[cfg(all(target_arch = "aarch64", target_os = "macos", target_pointer_width = "64"))]
        {
            if let Some(handle) = HANDLE.take() {
                // Return an AmxHandle to prove the above block was
                // executed exactly once (see doc comment).
                Ok(Self)
            } else {
                // Safe: We finally know that AMX is supported, and
                // not already enabled ITT, so enable it.
                unsafe { Self::set() };
                HANDLE.set(Some(Self));

                Ok(Self)
            }
        }
    }

    /// Disable AMX for the current thread. This must be private, and
    /// must be an instance method, so we can count on the invariant
    /// that it cannot be called without AMX having been initialised.
    fn disable(self) {
        // Unset `AMX_ENABLED`, so a new handle may be created. (This
        // one cannot now be used, as `self` is consumed by this fn.)
        HANDLE.set(false);

        // Safe: AMX is supported and handle initialised: see above.
        unsafe { self.clr() };
    }
}
