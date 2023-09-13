use core::ffi::c_void;
use core::ptr;

/// Raw display handle for OpenHarmony OS.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OHOSDisplayHandle {}

impl OHOSDisplayHandle {
     pub fn empty() -> Self {
        Self {}
    }
}

/// Raw window handle for OpenHarmony  NDK.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OHOSWindowHandle {
    /// A pointer to an `ANativeWindow`.
    pub a_native_window: *mut c_void,
}

impl OHOSWindowHandle {
      pub fn empty() -> Self {
        Self {
            a_native_window: ptr::null_mut(),
        }
    }
}
