use crate::{SDL_EventType, SDL_WindowID};
use std::{ffi::c_char, ptr::null};

/// Keyboard text input event structure (event.text.*)
///
/// # Remarks
/// This event will never be delivered unless text input is enabled by calling
/// [`SDL_StartTextInput`]. Text input is disabled by default!
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_TextInputEvent {
    /// [`SDL_EventType::TextInput`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with keyboard focus, if any
    pub window_id: SDL_WindowID,

    /// The input text, UTF-8 encoded
    pub text: *const c_char,
}

impl Default for SDL_TextInputEvent {
    fn default() -> Self {
        SDL_TextInputEvent {
            r#type: SDL_EventType::TextInput,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            text: null(),
        }
    }
}
