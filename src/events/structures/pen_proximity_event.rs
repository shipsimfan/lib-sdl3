use crate::{SDL_EventType, SDL_PenID, SDL_WindowID};

/// Pressure-sensitive pen proximity event structure (event.pproximity.*)
///
/// # Remarks
/// When a pen becomes visible to the system (it is close enough to a tablet, etc), SDL will send
/// an [`SDL_EventType::PenProximityIn`] event with the new pen's ID. This ID is valid until the
/// pen leaves proximity again (has been removed from the tablet's area, the tablet has been
/// unplugged, etc). If the same pen reenters proximity again, it will be given a new ID.
///
/// Note that "proximity" means "close enough for the tablet to know the tool is there." The pen
/// touching and lifting off from the tablet while not leaving the area are handled by
/// [`SDL_EventType::PenDown`] and [`SDL_EventType::PenUp`].
///
/// Not all platforms have a window associated with the pen during proximity events. Some wait
/// until motion/button/etc events to offer this info.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_PenProximityEvent {
    /// [`SDL_EventType::PenProximityIn`] or [`SDL_EventType::PenProximityOut`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with mouse focus, if any
    pub window_id: SDL_WindowID,

    /// The pen instance id
    pub which: SDL_PenID,
}

impl Default for SDL_PenProximityEvent {
    fn default() -> Self {
        SDL_PenProximityEvent {
            r#type: SDL_EventType::First,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            which: 0,
        }
    }
}
