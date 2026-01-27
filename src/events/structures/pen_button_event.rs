use crate::{SDL_EventType, SDL_PenID, SDL_PenInputFlags, SDL_WindowID};
use std::ffi::c_float;

/// Pressure-sensitive pen button event structure (event.pbutton.*)
///
/// # Remarks
/// This is for buttons on the pen itself that the user might click. The pen itself pressing down
/// to draw triggers a [`SDL_EventType::PenDown`] event instead.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PenButtonEvent {
    /// [`SDL_EventType::PenButtonDown`] or [`SDL_EventType::PenButtonUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with mouse focus, if any
    pub window_id: SDL_WindowID,

    /// The pen instance id
    pub which: SDL_PenID,

    /// Complete pen input state at time of event
    pub pen_state: SDL_PenInputFlags,

    /// X coordinate, relative to window
    pub x: c_float,

    /// Y coordinate, relative to window
    pub y: c_float,

    /// The pen button index (first button is 1).
    pub button: u8,

    /// `true` if the button is pressed
    pub down: bool,
}

impl Default for SDL_PenButtonEvent {
    fn default() -> Self {
        SDL_PenButtonEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            pen_state: 0,
            x: 0.0,
            y: 0.0,
            button: 0,
            down: false,
        }
    }
}
