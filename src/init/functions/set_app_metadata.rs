use std::ffi::c_char;

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_GetError, SDL_Init, SDL_SetAppMetadataProperty};
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
extern "C" {
    /// Specify basic metadata about your app.
    ///
    /// # Parameters
    ///  * `appname` - The name of the application ("My Game 2: Bad Guy's Revenge!").
    ///  * `appversion` - The version of the application ("1.0.0beta5" or a git hash, or whatever
    ///                   makes sense).
    ///  * `appidentifier` - A unique string in reverse-domain format that identifies this app
    ///                      ("com.example.mygame2").
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// You can optionally provide metadata about your app to SDL. This is not required, but
    /// strongly encouraged.
    ///
    /// There are several locations where SDL can make use of metadata (an "About" box in the macOS
    /// menu bar, the name of the app can be shown on some audio mixers, etc). Any piece of
    /// metadata can be left as [`null`], if a specific detail doesn't make sense for the app.
    ///
    /// This function should be called as early as possible, before [`SDL_Init`]. Multiple calls to
    /// this function are allowed, but various state might not change once it has been set up with
    /// a previous call to this function.
    ///
    /// Passing a [`null`] removes any previous metadata.
    ///
    /// This is a simplified interface for the most important information. You can supply
    /// significantly more detailed metadata with [`SDL_SetAppMetadataProperty`].
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_SetAppMetadata(
        appname: *const c_char,
        appversion: *const c_char,
        appidentifier: *const c_char,
    ) -> bool;
}
