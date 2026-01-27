use crate::{SDL_EventType, SDL_PenAxis, SDL_PenID, SDL_PenInputFlags, SDL_WindowID};
use std::ffi::c_float;

/// Pressure-sensitive pen pressure / angle event structure (event.paxis.*)
///
/// # Remarks
/// You might get some of these events even if the pen isn't touching the tablet.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PenAxisEvent {
    /// [`SDL_EventType::PenAxis`]
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

    /// Axis that has changed
    pub axis: SDL_PenAxis,

    /// New value of axis
    pub value: c_float,
}

impl Default for SDL_PenAxisEvent {
    fn default() -> Self {
        SDL_PenAxisEvent {
            r#type: SDL_EventType::PenAxis,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            pen_state: 0,
            x: 0.0,
            y: 0.0,
            axis: SDL_PenAxis::Pressure,
            value: 0.0,
        }
    }
}
