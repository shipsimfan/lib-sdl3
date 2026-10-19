use std::{ffi::c_char, ptr::null};

/// An entry for filters for file dialogs.
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub struct SDL_DialogFileFilter {
    /// `name` is a user-readable label for the filter (for example, "Office document").
    pub name: *const c_char,

    /// `pattern` is a semicolon-separated list of file extensions (for example, "doc;docx"). File
    /// extensions may only contain alphanumeric characters, hyphens, underscores and periods.
    /// Alternatively, the whole string can be a single asterisk ("*"), which serves as an
    /// "All files" filter.
    pub pattern: *const c_char,
}

impl Default for SDL_DialogFileFilter {
    fn default() -> Self {
        SDL_DialogFileFilter {
            name: null(),
            pattern: null(),
        }
    }
}
