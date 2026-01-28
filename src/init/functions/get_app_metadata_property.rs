use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_SetAppMetadata, SDL_SetAppMetadataProperty};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Get metadata about your app.
    ///
    /// # Parameters
    ///  * `name` - the name of the metadata property to get.
    ///
    /// # Return Value
    /// Returns the current value of the metadata property, or the default if it is not set,
    /// [`null`] for properties with no default.
    ///
    /// # Remarks
    /// This returns metadata previously set using [`SDL_SetAppMetadata`] or
    /// [`SDL_SetAppMetadataProperty`]. See [`SDL_SetAppMetadataProperty`] for the list of
    /// available properties and their meanings.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, although the string returned is not
    /// protected and could potentially be freed if you call [`SDL_SetAppMetadataProperty`] to set
    /// that property from another thread.
    pub fn SDL_GetAppMetadataProperty(name: *const c_char) -> *const c_char;
}
