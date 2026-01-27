use crate::SDL_WindowID;
use std::{ffi::c_void, ptr::null_mut};

// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_EventType;

/// A user-defined event type (event.user.*)
///
/// # Remarks
/// This event is unique; it is never created by SDL, but only by the application. The event can be
/// pushed onto the event queue using [`SDL_PushEvent`]. The contents of the structure members are
/// completely up to the programmer; the only requirement is that '''type''' is a value obtained
/// from [`SDL_RegisterEvents`].
///
/// # Code Examples
/// ```no_run
/// extern static my_event_code: u32;
/// extern static significant_data: *mut std::ffi::c_void;
/// extern static some_other_data: *mut std::ffi::c_void;
///
/// let my_event_type = SDL_RegisterEvents(1);
/// if my_event_type != 0 {
///     let event = SDL_Event::default();
///     SDL_zero(event);
///     event.type = my_event_type;
///     event.user.code = my_event_code;
///     event.user.data1 = significant_data;
///     event.user.data2 = some_other_data;
///     SDL_PushEvent(&event);
/// }
/// ```
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_UserEvent {
    /// [`SDL_EventType::User`] through [`SDL_EventType::Last`], [`u32`] because these are not in
    /// the [`SDL_EventType`] enumeration
    pub r#type: u32,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The associated window if any
    pub window_id: SDL_WindowID,

    /// User defined event code
    pub code: i32,

    /// User defined data pointer
    pub data1: *mut c_void,

    /// User defined data pointer
    pub data2: *mut c_void,
}

impl Default for SDL_UserEvent {
    fn default() -> Self {
        SDL_UserEvent {
            r#type: 0,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            code: 0,
            data1: null_mut(),
            data2: null_mut(),
        }
    }
}
