use crate::{SDL_EventType, SDL_JoystickID};

/// Joystick hat position change event structure (event.jhat.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyHatEvent {
    /// [`SDL_EventType::JoystickHatMotion`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The joystick hat index
    pub hat: u8,

    /// The hat position value.
    pub value: u8,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,
}
