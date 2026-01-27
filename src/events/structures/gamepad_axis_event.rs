use crate::{SDL_EventType, SDL_JoystickID};

/// Gamepad axis motion event structure (event.gaxis.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_GamepadAxisEvent {
    /// [`SDL_EventType::GamepadAxisMotion`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The gamepad axis ([`SDL_GamepadAxis`])
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
    pub padding4: u16,
}

impl Default for SDL_GamepadAxisEvent {
    fn default() -> Self {
        SDL_GamepadAxisEvent {
            r#type: SDL_EventType::GamepadAxisMotion,
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
