use core::ffi::c_void;
use core::ptr::NonNull;

/// Raw display handle for OpenHarmony OS.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OHOSDisplayHandle {}

impl OHOSDisplayHandle {
    /// Create a new empty display handle.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use raw_window_handle::OHOSDisplayHandle;
    /// let handle = OHOSDisplayHandle::new();
    /// ```
    pub fn new() -> Self {
        Self {}
    }
}

/// Raw window handle for OpenHarmony  NDK.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct OHOSWindowHandle {
    /// A pointer to an `ANativeWindow`.
    pub a_native_window: NonNull<c_void>,
}

impl OHOSWindowHandle {
    /// Create a new handle to an `ANativeWindow`.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// # use core::ptr::NonNull;
    /// # use raw_window_handle::OHOSWindowHandle;
    /// # type ANativeWindow = ();
    /// #
    /// let ptr: NonNull<ANativeWindow>;
    /// # ptr = NonNull::from(&());
    /// let handle = OHOSWindowHandle::new(ptr.cast());
    /// ```
    pub fn new(a_native_window: NonNull<c_void>) -> Self {
        Self { a_native_window }
    }
}
