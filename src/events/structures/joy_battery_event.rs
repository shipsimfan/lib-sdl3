use crate::{SDL_EventType, SDL_JoystickID, SDL_PowerState};
use std::ffi::c_int;

/// Joystick battery level change event structure (event.jbattery.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyBatteryEvent {
    /// [`SDL_EventType::JoystickBatteryUpdated`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The joystick battery state
    pub state: SDL_PowerState,

    /// The joystick battery percent charge remaining
    pub percent: c_int,
}

impl Default for SDL_JoyBatteryEvent {
    fn default() -> Self {
        SDL_JoyBatteryEvent {
            r#type: SDL_EventType::JoystickBatteryUpdated,
            reserved: 0,
            timestamp: 0,
            which: 0,
            state: SDL_PowerState::Unknown,
            percent: 0,
        }
    }
}
