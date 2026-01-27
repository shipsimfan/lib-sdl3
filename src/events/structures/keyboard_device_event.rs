use crate::{SDL_EventType, SDL_KeyboardID};

/// Keyboard device event structure (event.kdevice.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_KeyboardDeviceEvent {
    /// [`SDL_EventType::KeyboardAdded`] or [`SDL_EventType::KeyboardRemoved`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The keyboard instance id
    pub which: SDL_KeyboardID,
}

impl Default for SDL_KeyboardDeviceEvent {
    fn default() -> Self {
        SDL_KeyboardDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
        }
    }
}
