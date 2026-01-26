use crate::{SDL_EventType, SDL_WindowID};

/// Window state change event data (event.window.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_WindowEvent {
    /// `SDL_EventType::Window*`
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The associated window
    pub window_id: SDL_WindowID,

    /// event dependent data
    pub data1: i32,

    /// event dependent data
    pub data2: i32,
}

impl Default for SDL_WindowEvent {
    fn default() -> Self {
        SDL_WindowEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            data1: 0,
            data2: 0,
        }
    }
}
