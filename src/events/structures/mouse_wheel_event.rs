use crate::{SDL_EventType, SDL_MouseID, SDL_MouseWheelDirection, SDL_WindowID};
use std::ffi::c_float;

/// Mouse wheel event structure (event.wheel.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_MouseWheelEvent {
    /// [`SDL_EventType::MouseWheel`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with mouse focus, if any
    pub window_id: SDL_WindowID,

    /// The mouse instance id in relative mode or 0
    pub which: SDL_MouseID,

    /// The amount scrolled horizontally, positive to the right and negative to the left
    pub x: c_float,

    /// The amount scrolled vertically, positive away from the user and negative toward the user
    pub y: c_float,

    /// Set to one of the `SDL_MOUSEWHEEL_*` defines. When FLIPPED the values in X and Y will be
    /// opposite. Multiply by -1 to change them back
    pub direction: SDL_MouseWheelDirection,

    /// X coordinate, relative to window
    pub mouse_x: c_float,

    /// Y coordinate, relative to window
    pub mouse_y: c_float,

    /// The amount scrolled horizontally, accumulated to whole scroll "ticks" (added in 3.2.12)
    pub integer_x: i32,

    /// The amount scrolled vertically, accumulated to whole scroll "ticks" (added in 3.2.12)
    pub integer_y: i32,
}

impl Default for SDL_MouseWheelEvent {
    fn default() -> Self {
        SDL_MouseWheelEvent {
            r#type: SDL_EventType::MouseWheel,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            x: 0.0,
            y: 0.0,
            direction: SDL_MouseWheelDirection::Normal,
            mouse_x: 0.0,
            mouse_y: 0.0,
            integer_x: 0,
            integer_y: 0,
        }
    }
}
