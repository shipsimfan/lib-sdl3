use crate::{SDL_CameraID, SDL_EventType};

/// Camera device event structure (event.cdevice.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_CameraDeviceEvent {
    /// [`SDL_EventType::CameraDeviceAdded`], [`SDL_EventType::CameraDeviceRemoved`],
    /// [`SDL_EventType::CameraDeviceApproved`], [`SDL_EventType::CameraDeviceDenied`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// [`SDL_CameraID`] for the device being added or removed or changing
    pub which: SDL_CameraID,
}

impl Default for SDL_CameraDeviceEvent {
    fn default() -> Self {
        SDL_CameraDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
        }
    }
}
