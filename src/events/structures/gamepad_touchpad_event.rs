use crate::{SDL_EventType, SDL_JoystickID};
use std::ffi::c_float;

/// Gamepad touchpad event structure (event.gtouchpad.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_GamepadTouchpadEvent {
    /// [`SDL_EventType::GamepadTouchpadDown`] or [`SDL_EventType::GamepadTouchpadMotion`] or
    /// [`SDL_EventType::GamepadTouchpadUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The index of the touchpad
    pub touchpad: i32,

    /// The index of the finger on the touchpad
    pub finger: i32,

    /// Normalized in the range 0...1 with 0 being on the left
    pub x: c_float,

    /// Normalized in the range 0...1 with 0 being at the top
    pub y: c_float,

    /// Normalized in the range 0...1
    pub pressure: c_float,
}

impl Default for SDL_GamepadTouchpadEvent {
    fn default() -> Self {
        SDL_GamepadTouchpadEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
            touchpad: 0,
            finger: 0,
            x: 0.0,
            y: 0.0,
            pressure: 0.0,
        }
    }
}
