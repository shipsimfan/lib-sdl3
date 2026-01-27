use crate::{SDL_EventType, SDL_WindowID};
use std::ffi::c_float;

/// Pinch event structure (event.pinch.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PinchFingerEvent {
    /// [`SDL_EventType::PinchBegin`] or [`SDL_EventType::PinchUpdate`] or
    /// [`SDL_EventType::PinchEnd`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The scale change since the last [`SDL_EventType::PinchUpdate`]. Scale < 1 is "zoom out".
    /// Scale > 1 is "zoom in".
    pub scale: c_float,

    /// The window underneath the finger, if any
    pub window_id: SDL_WindowID,
}

impl Default for SDL_PinchFingerEvent {
    fn default() -> Self {
        SDL_PinchFingerEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            scale: 0.0,
            window_id: 0,
        }
    }
}
