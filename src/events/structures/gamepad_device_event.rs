use crate::{SDL_EventType, SDL_JoystickID};

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_Init, SDL_JoyDeviceEvent};

/// Gamepad device event structure (event.gdevice.*)
///
/// # Remarks
/// Joysticks that are supported gamepads receive both an [`SDL_JoyDeviceEvent`] and an
/// [`SDL_GamepadDeviceEvent`].
///
/// SDL will send [`SDL_EventType::GamepadAdded`] events for joysticks that are already plugged in
/// during [`SDL_Init`] and are recognized as gamepads. It will also send events for joysticks that
/// get gamepad mappings at runtime.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_GamepadDeviceEvent {
    /// [`SDL_EventType::GamepadAdded`], [`SDL_EventType::GamepadRemoved`], or
    /// [`SDL_EventType::GamepadRemapped`], [`SDL_EventType::GamepadUpdateComplete`] or
    /// [`SDL_EventType::GamepadSteamHandleUpdated`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,
}

impl Default for SDL_GamepadDeviceEvent {
    fn default() -> Self {
        SDL_GamepadDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
        }
    }
}
