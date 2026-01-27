use crate::{SDL_EventType, SDL_MouseID, SDL_WindowID};
use std::ffi::c_float;

/// Mouse button event structure (event.button.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_MouseButtonEvent {
    /// [`SDL_EventType::MouseButtonDown`] or [`SDL_EventType::MouseButtonUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with mouse focus, if any
    pub window_id: SDL_WindowID,

    /// The mouse instance id in relative mode, [`SDL_TOUCH_MOUSEID`] for touch events, or 0
    pub which: SDL_MouseID,

    /// The mouse button index
    pub button: u8,

    /// `true` if the button is pressed
    pub down: bool,

    /// 1 for single-click, 2 for double-click, etc.
    pub clicks: u8,

    #[allow(missing_docs)]
    pub padding: u8,

    /// X coordinate, relative to window
    pub x: c_float,

    /// Y coordinate, relative to window
    pub y: c_float,
}

impl Default for SDL_MouseButtonEvent {
    fn default() -> Self {
        SDL_MouseButtonEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            button: 0,
            down: false,
            clicks: 0,
            padding: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}
