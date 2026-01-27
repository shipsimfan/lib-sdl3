use crate::{SDL_EventType, SDL_JoystickID};

/// Joystick trackball motion event structure (event.jball.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_JoyBallEvent {
    /// [`SDL_EventType::JoystickBallMotion`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The joystick trackball index
    pub ball: u8,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,

    #[allow(missing_docs)]
    pub padding3: u8,

    /// The relative motion in the X direction
    xrel: i16,

    /// The relative motion in the Y direction
    yrel: i16,
}

impl Default for SDL_JoyBallEvent {
    fn default() -> Self {
        SDL_JoyBallEvent {
            r#type: SDL_EventType::JoystickBallMotion,
            reserved: 0,
            timestamp: 0,
            which: 0,
            ball: 0,
            padding1: 0,
            padding2: 0,
            padding3: 0,
            xrel: 0,
            yrel: 0,
        }
    }
}
