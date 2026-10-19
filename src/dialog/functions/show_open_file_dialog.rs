use crate::{SDL_DialogFileCallback, SDL_DialogFileFilter, SDL_Window};
use std::ffi::{c_char, c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PumpEvents;
#[allow(unused_imports)]
use std::ptr::{null, null_mut};

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Displays a dialog that lets the user select a file on their filesystem.
    ///
    /// # Parameters
    ///  * `callback` - a function pointer to be invoked when the user selects a file and accepts,
    ///                 or cancels the dialog, or an error occurs.
    ///  * `userdata` - an optional pointer to pass extra data to the callback when it will be
    ///                 invoked.
    ///  * `window` - the window that the dialog should be modal for, may be [`null_mut`]. Not all
    ///               platforms support this option.
    ///  * `filters` - a list of filters, may be [`null`]. Not all platforms support this option,
    ///                and platforms that do support it may allow the user to ignore the filters.
    ///                If non-[`null`], it must remain valid at least until the callback is
    ///                invoked.
    ///  * `nfilters` - the number of filters. Ignored if `filters` is [`null`].
    ///  * `default_location` - the default folder or file to start the dialog at, may be [`null`].
    ///                         Not all platforms support this option.
    ///  * `allow_many` - if non-zero, the user will be allowed to select multiple entries. Not all
    ///                   platforms support this option.
    ///
    /// # Remarks
    /// This is an asynchronous function; it will return immediately, and the result will be passed
    /// to the callback.
    ///
    /// The callback will be invoked with a null-terminated list of files the user chose. The list
    /// will be empty if the user canceled the dialog, and it will be [`null`] if an error
    /// occurred.
    ///
    /// Note that the callback may be called from a different thread than the one the function was
    /// invoked on.
    ///
    /// Depending on the platform, the user may be allowed to input paths that don't yet exist.
    ///
    /// On Linux, dialogs may require XDG Portals, which requires DBus, which requires an
    /// event-handling loop. Apps that do not use SDL to handle events should add a call to
    /// [`SDL_PumpEvents`] in their main loop.
    ///
    /// # Thread Safety
    /// This function should be called only from the main thread. The callback may be invoked from
    /// the same thread or from a different one, depending on the OS's constraints.
    pub fn SDL_ShowOpenFileDialog(
        callback: SDL_DialogFileCallback,
        userdata: *mut c_void,
        window: *mut SDL_Window,
        filters: *const SDL_DialogFileFilter,
        nfilters: c_int,
        default_location: *const c_char,
        allow_many: bool,
    );
}
