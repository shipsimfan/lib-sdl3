use crate::{SDL_EventType, SDL_FingerID, SDL_TouchID, SDL_WindowID};
use std::ffi::c_float;

/// Touch finger event structure (event.tfinger.*)
///
/// # Remarks
/// Coordinates in this event are normalized. `x` and `y` are normalized to a range between 0.0 and
/// 1.0, relative to the window, so (0,0) is the top left and (1,1) is the bottom right. Delta
/// coordinates `dx` and `dy` are normalized in the ranges of -1.0 (traversed all the way from the
/// bottom or right to all the way up or left) to 1.0 (traversed all the way from the top or left
/// to all the way down or right).
///
/// Note that while the coordinates are normalized, they are not clamped, which means in some
/// circumstances you can get a value outside of this range. For example, a renderer using logical
/// presentation might give a negative value when the touch is in the letterboxing. Some platforms
/// might report a touch outside of the window, which will also be outside of the range.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_TouchFingerEvent {
    /// [`SDL_EventType::FingerDown`], [`SDL_EventType::FingerUp`],
    /// [`SDL_EventType::FingerMotion`], or [`SDL_EventType::FingerCanceled`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The touch device id
    pub touch_id: SDL_TouchID,

    #[allow(missing_docs)]
    pub finger_id: SDL_FingerID,

    /// Normalized in the range 0...1
    pub x: c_float,

    /// Normalized in the range 0...1
    pub y: c_float,

    /// Normalized in the range -1...1
    pub dx: c_float,

    /// Normalized in the range -1...1
    pub dy: c_float,

    /// Normalized in the range 0...1
    pub pressure: c_float,

    /// The window underneath the finger, if any
    pub window_id: SDL_WindowID,
}

impl Default for SDL_TouchFingerEvent {
    fn default() -> Self {
        SDL_TouchFingerEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            touch_id: 0,
            finger_id: 0,
            x: 0.0,
            y: 0.0,
            dx: 0.0,
            dy: 0.0,
            pressure: 0.0,
            window_id: 0,
        }
    }
}
