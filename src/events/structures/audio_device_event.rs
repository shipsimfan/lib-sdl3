use crate::{SDL_AudioDeviceID, SDL_EventType};

/// Audio device event structure (event.adevice.*)
///
/// # Remarks
/// Note that SDL will send a [`SDL_EventType::AudioDeviceAdded`] event for every device it
/// discovers during initialization. After that, this event will only arrive when a device is
/// hotplugged during the program's run.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_AudioDeviceEvent {
    /// [`SDL_EventType::AudioDeviceAdded`], or [`SDL_EventType::AudioDeviceRemoved`], or
    /// [`SDL_EventType::AudioDeviceFormatChanged`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// [`SDL_AudioDeviceID`] for the device being added or removed or changing
    pub which: SDL_AudioDeviceID,

    /// `false` if a playback device, `true` if a recording device.
    pub recording: bool,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,

    #[allow(missing_docs)]
    pub padding3: u8,
}

impl Default for SDL_AudioDeviceEvent {
    fn default() -> Self {
        SDL_AudioDeviceEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            which: 0,
            recording: false,
            padding1: 0,
            padding2: 0,
            padding3: 0,
        }
    }
}
