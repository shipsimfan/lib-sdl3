use crate::{SDL_EventType, SDL_PenID, SDL_PenInputFlags, SDL_WindowID};
use std::ffi::c_float;

/// Pressure-sensitive pen touched event structure (event.ptouch.*)
///
/// # Remarks
/// These events come when a pen touches a surface (a tablet, etc), or lifts off from one.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PenTouchEvent {
    /// [`SDL_EventType::PenDown`] or [`SDL_EventType::PenUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with pen focus, if any
    pub window_id: SDL_WindowID,

    /// The pen instance id
    pub which: SDL_PenID,

    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,

    /// X coordinate, relative to window
    pub x: c_float,

    /// Y coordinate, relative to window
    pub y: c_float,

    /// `true` if eraser end is used (not all pens support this).
    pub eraser: bool,

    /// `true` if the pen is touching or `false` if the pen is lifted off
    pub down: bool,
}

impl Default for SDL_PenTouchEvent {
    fn default() -> Self {
        SDL_PenTouchEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            pen_state: 0,
            x: 0.0,
            y: 0.0,
            eraser: false,
            down: false,
        }
    }
}
