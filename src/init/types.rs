// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_Init, SDL_QuitSubSystem};

/// Initialization flags for [`SDL_Init`] and/or [`SDL_QuitSubSystem`]
///
/// # Remarks
/// These are the flags which may be passed to [`SDL_Init`]. You should specify the subsystems
/// which you will be using in your application.
#[allow(non_camel_case_types)]
pub type SDL_InitFlags = u32;
