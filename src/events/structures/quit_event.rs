use crate::SDL_EventType;

/// The "quit requested" event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_QuitEvent {
    /// [`SDL_EventType::Quit`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,
}

impl Default for SDL_QuitEvent {
    fn default() -> Self {
        SDL_QuitEvent {
            r#type: SDL_EventType::Quit,
            reserved: 0,
            timestamp: 0,
        }
    }
}
