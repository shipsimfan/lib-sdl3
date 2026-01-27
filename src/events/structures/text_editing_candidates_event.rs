use crate::{SDL_EventType, SDL_WindowID};
use std::{ffi::c_char, ptr::null};

/// Keyboard IME candidates event structure (event.edit_candidates.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_TextEditingCandidatesEvent {
    /// [`SDL_EventType::TextEditingCandidates`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The window with keyboard focus, if any
    pub window_id: SDL_WindowID,

    /// The list of candidates, or [`null`] if there are no candidates available
    pub candidates: *const *const c_char,

    /// The number of strings in `candidates`
    pub num_candidates: i32,

    /// The index of the selected candidate, or -1 if no candidate is selected
    pub selected_candidate: i32,

    /// `true` if the list is horizontal, `false` if it's vertical
    pub horizontal: bool,

    #[allow(missing_docs)]
    pub padding1: u8,

    #[allow(missing_docs)]
    pub padding2: u8,

    #[allow(missing_docs)]
    pub padding3: u8,
}

impl Default for SDL_TextEditingCandidatesEvent {
    fn default() -> Self {
        SDL_TextEditingCandidatesEvent {
            r#type: SDL_EventType::TextEditingCandidates,
            reserved: 0,
            timestamp: 0,
            window_id: 0,
            candidates: null(),
            num_candidates: 0,
            selected_candidate: 0,
            horizontal: false,
            padding1: 0,
            padding2: 0,
            padding3: 0,
        }
    }
}
