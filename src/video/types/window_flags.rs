// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_CreateWindow, SDL_WINDOW_RESIZABLE};

/// The flags on a window.
///
/// # Remarks
/// These cover a lot of true/false, or on/off, window state. Some of it is immutable after being
/// set through [`SDL_CreateWindow`], some of it can be changed on existing windows by the app, and
/// some of it might be altered by the user or system outside of the app's control.
///
/// When creating windows with [`SDL_WINDOW_RESIZABLE`], SDL will constrain resizable windows to
/// the dimensions recommended by the compositor to fit it within the usable desktop space,
/// although some compositors will do this automatically without intervention as well. Use
/// [`SDL_SetWindowResizable`] after creation instead if you wish to create a window with a
/// specific size.
#[allow(non_camel_case_types)]
pub type SDL_WindowFlags = u64;
