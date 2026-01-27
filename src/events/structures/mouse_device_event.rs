use crate::{SDL_EventType, SDL_MouseID};

/// Mouse device event structure (event.mdevice.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_MouseDeviceEvent {
    /// [`SDL_EventType::MouseAdded`] or [`SDL_EventType::MouseRemoved`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The mouse instance id
    pub which: SDL_MouseID,
}

impl Default for SDL_MouseDeviceEvent {
    fn default() -> Self {
        SDL_MouseDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
        }
    }
}
