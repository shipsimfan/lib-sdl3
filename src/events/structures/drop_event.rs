use crate::{SDL_EventType, SDL_WindowID};
use std::{
    ffi::{c_char, c_float},
    ptr::null,
};

/// An event used to drop text or request a file open by the system (event.drop.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_DropEvent {
    /// [`SDL_EventType::DropBegin`] or [`SDL_EventType::DropFile`] or [`SDL_EventType::DropText`]
    /// or [`SDL_EventType::DropComplete`] or [`SDL_EventType::DropPosition`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window that was dropped on, if any
    pub window_id: SDL_WindowID,

    /// X coordinate, relative to window (not on begin)
    pub x: c_float,

    /// Y coordinate, relative to window (not on begin)
    pub y: c_float,

    /// The source app that sent this drop event, or [`null`] if that isn't available
    pub source: *const c_char,

    /// The text for [`SDL_EventType::DropText`] and the file name for [`SDL_EventType::DropFile`],
    /// [`null`] for other events
    pub data: *const c_char,
}

impl Default for SDL_DropEvent {
    fn default() -> Self {
        SDL_DropEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            x: 0.0,
            y: 0.0,
            source: null(),
            data: null(),
        }
    }
}
