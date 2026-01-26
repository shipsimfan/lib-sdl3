/// Fields shared by every event
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_CommonEvent {
    /// Event type, shared with all events, Uint32 to cover user events which are not in the SDL_EventType enumeration
    pub r#type: u32,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,
}

impl Default for SDL_CommonEvent {
    fn default() -> Self {
        SDL_CommonEvent {
            r#type: 0,
            reserved: 0,
            timestamp: 0,
        }
    }
}
