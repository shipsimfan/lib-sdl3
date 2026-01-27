use crate::{SDL_EventType, SDL_JoystickID};

/// Joystick axis motion event structure (event.jaxis.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyAxisEvent {
    /// [`SDL_EventType::JoystickAxisMotion`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The joystick axis index
    pub axis: u8,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,

    #[allow(missing_docs)]
    pub padding3: u8,

    /// The axis value (range: -32768 to 32767)
    pub value: i16,

    #[allow(missing_docs)]
    pub padding4: u8,
}

impl Default for SDL_JoyAxisEvent {
    fn default() -> Self {
        SDL_JoyAxisEvent {
            r#type: SDL_EventType::JoystickAxisMotion,
            reserved: 0,
            timestamp: 0,
            which: 0,
            axis: 0,
            padding1: 0,
            padding2: 0,
            padding3: 0,
            value: 0,
            padding4: 0,
        }
    }
}
