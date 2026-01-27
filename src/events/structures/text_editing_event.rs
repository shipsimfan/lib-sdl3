use crate::{SDL_EventType, SDL_WindowID};
use std::{ffi::c_char, ptr::null};

/// Keyboard text editing event structure (event.edit.*)
///
/// # Remarks
/// The start cursor is the position, in UTF-8 characters, where new typing will be inserted into
/// the editing text. The length is the number of UTF-8 characters that will be replaced by new
/// typing.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_TextEditingEvent {
    /// [`SDL_EventType::TextEditing`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with keyboard focus, if any
    pub window_id: SDL_WindowID,

    /// The editing text
    pub text: *const c_char,

    /// The start cursor of selected editing text, or -1 if not set
    pub start: i32,

    /// The length of selected editing text, or -1 if not set
    pub length: i32,
}

impl Default for SDL_TextEditingEvent {
    fn default() -> Self {
        SDL_TextEditingEvent {
            r#type: SDL_EventType::TextEditing,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            text: null(),
            start: 0,
            length: 0,
        }
    }
}
