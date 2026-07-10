use std::ffi::c_void;

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null_mut;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Free allocated memory.
    ///
    /// # Parameters
    ///  * `mem` - a pointer to allocated memory, or [`null_mut`].
    ///
    /// # Remarks
    /// The pointer is no longer valid after this call and cannot be dereferenced anymore.
    ///
    /// If `mem` is [`null_mut`], this function does nothing.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_free(mem: *mut c_void);
}
