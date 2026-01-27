use crate::{SDL_EventType, SDL_PenID, SDL_PenInputFlags, SDL_WindowID};
use std::ffi::c_float;

/// Pressure-sensitive pen motion event structure (event.pmotion.*)
///
/// # Remarks
/// Depending on the hardware, you may get motion events when the pen is not touching a tablet, for
/// tracking a pen even when it isn't drawing. You should listen for [`SDL_EventType::PenDown`] and
/// [`SDL_EventType::PenUp`] events, or `check pen_state & SDL_PEN_INPUT_DOWN` to decide if a pen
/// is "drawing" when dealing with pen motion.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PenMotionEvent {
    /// [`SDL_EventType::PenMotion`]
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
}

impl Default for SDL_PenMotionEvent {
    fn default() -> Self {
        SDL_PenMotionEvent {
            r#type: SDL_EventType::PenMotion,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
            pen_state: 0,
            x: 0.0,
            y: 0.0,
        }
    }
}
