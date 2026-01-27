use crate::{SDL_EventType, SDL_JoystickID};

/// Gamepad button event structure (event.gbutton.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_GamepadButtonEvent {
    /// [`SDL_EventType::GamepadButtonDown`] or [`SDL_EventType::GamepadButtonUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The gamepad button ([`SDL_GamepadButton`])
    pub button: u8,

    /// `true` if the button is pressed
    pub down: bool,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,
}

impl Default for SDL_GamepadButtonEvent {
    fn default() -> Self {
        SDL_GamepadButtonEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
            button: 0,
            down: false,
            padding1: 0,
            padding2: 0,
        }
    }
}
