use crate::{SDL_EventType, SDL_JoystickID};

/// Joystick button event structure (event.jbutton.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyButtonEvent {
    /// [`SDL_EventType::JoystickButtonDown`] or [`SDL_EventType::JoystickButtonUp`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The joystick button index
    pub button: u8,

    /// `true` if the button is pressed
    pub down: bool,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,
}

impl Default for SDL_JoyButtonEvent {
    fn default() -> Self {
        SDL_JoyButtonEvent {
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
