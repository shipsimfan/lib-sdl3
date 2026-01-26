use crate::{SDL_DisplayID, SDL_EventType};

/// Display state change event data (event.display.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_DisplayEvent {
    /// `SDL_EventType::Display*`
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The associated display
    pub display_id: SDL_DisplayID,

    /// event dependent data
    pub data1: i32,

    /// event dependent data
    pub data2: i32,
}

impl Default for SDL_DisplayEvent {
    fn default() -> Self {
        SDL_DisplayEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            display_id: 0,
            data1: 0,
            data2: 0,
        }
    }
}
