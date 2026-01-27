use crate::{SDL_EventType, SDL_JoystickID};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_Init;

/// Joystick device event structure (event.jdevice.*)
///
/// # Remarks
/// SDL will send [`SDL_EventType::JoystickAdded`] events for devices that are already plugged in
/// during [`SDL_Init`].
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyDeviceEvent {
    /// [`SDL_EventType::JoystickAdded`] or [`SDL_EventType::JoystickRemoved`] or
    /// [`SDL_EventType::JoystickUpdateComplete`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,
}

impl Default for SDL_JoyDeviceEvent {
    fn default() -> Self {
        SDL_JoyDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
        }
    }
}
