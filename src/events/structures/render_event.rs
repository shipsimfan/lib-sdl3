use crate::{SDL_EventType, SDL_WindowID};

/// Renderer event structure (event.render.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_RenderEvent {
    /// [`SDL_EventType::RenderTargetsReset`], [`SDL_EventType::RenderDeviceReset`],
    /// [`SDL_EventType::RenderDeviceLost`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window containing the renderer in question
    pub window_id: SDL_WindowID,
}

impl Default for SDL_RenderEvent {
    fn default() -> Self {
        SDL_RenderEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
        }
    }
}
