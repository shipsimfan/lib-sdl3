use crate::SDL_EventType;
use std::{ffi::c_char, ptr::null};

/// An event triggered when the clipboard contents have changed (event.clipboard.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_ClipboardEvent {
    /// [`SDL_EventType::ClipboardUpdate`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// are we owning the clipboard (internal update)
    pub owner: bool,

    /// number of mime types
    pub num_mime_types: i32,

    /// current mime types
    pub mime_types: *const *const c_char,
}

impl Default for SDL_ClipboardEvent {
    fn default() -> Self {
        SDL_ClipboardEvent {
            r#type: SDL_EventType::ClipboardUpdate,
            reserved: 0,
            timestamp: 0,
            owner: false,
            num_mime_types: 0,
            mime_types: null(),
        }
    }
}
