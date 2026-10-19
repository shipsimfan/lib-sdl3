use std::ffi::{c_char, c_int, c_void};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_GetError;
#[allow(unused_imports)]
use std::ptr::null;

/// Callback used by file dialog functions.
///
/// # Parameters
///  * `userdata` - an app-provided pointer, for the callback's use.
///  * `filelist` - the file(s) chosen by the user.
///  * `filter` - index of the selected filter.
///
/// # Remarks
/// The specific usage is described in each function.
///
/// If `filelist` is:
///  - [`null`], an error occurred. Details can be obtained with [`SDL_GetError`]
///  - A pointer to [`null`], the user either didn't choose any file or canceled the dialog
///  - A pointer to non-[`null`], the user chose one or more files. The argument is a
///    null-terminated array of pointers to UTF-8 encoded strings, each containing a path.
///
/// The `filelist` argument should not be freed; it will automatically be freed when the callback
/// returns.
///
/// The `filter` argument is the index of the filter that was selected, or -1 if no filter was
/// selected or if the platform or method doesn't support fetching the selected filter.
///
/// In Android, the filelist are `content://` URIs. They should be opened using [`SDL_IOFromFile`]
/// with appropriate modes. This applies both to open and save file dialog.
#[allow(non_camel_case_types)]
pub type SDL_DialogFileCallback =
    unsafe extern "C" fn(userdata: *mut c_void, filelist: *const *const c_char, filter: c_int);
