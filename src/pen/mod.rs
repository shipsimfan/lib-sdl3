//! SDL pen event handling.
//!
//! SDL provides an API for pressure-sensitive pen (stylus and/or eraser) handling, e.g., for input
//! and drawing tablets or suitably equipped mobile / tablet devices.
//!
//! To get started with pens, simply handle pen events:
//!  - [`SDL_EventType::PenProximityIn`], [`SDL_EventType::PenProximityOut`]
//!    ([`SDL_PenProximityEvent`])
//!  - [`SDL_EventType::PenDown`], [`SDL_EventType::PenUp`] ([`SDL_PenTouchEvent`])
//!  - [`SDL_EventType::PenMotion`] ([`SDL_PenMotionEvent`])
//!  - [`SDL_EventType::PenButtonDown`], [`SDL_EventType::PenButtonUp`] ([`SDL_PenButtonEvent`])
//!  - [`SDL_EventType::PenAxis`] ([`SDL_PenAxisEvent`])
//!
//! Pens may provide more than simple touch input; they might have other axes, such as pressure,
//! tilt, rotation, etc.
//!
//! When a pen starts providing input, SDL will assign it a unique [`SDL_PenID`], which will remain
//! for the life of the process, as long as the pen stays connected. A pen leaving proximity (being
//! taken far enough away from the digitizer tablet that it no longer reponds) and then coming back
//! should fire proximity events, but the [`SDL_PenID`] should remain consistent. Unplugging the
//! digitizer and reconnecting may cause future input to have a new [`SDL_PenID`], as SDL may not
//! know that this is the same hardware.
//!
//! Please note that various platforms vary wildly in how (and how well) they support pen input. If
//! your pen supports some piece of functionality but SDL doesn't seem to, it might actually be the
//! operating system's fault. For example, some platforms can manage multiple devices at the same
//! time, but others will make any connected pens look like a single logical device, much how all
//! USB mice connected to a computer will move the same system cursor. Other platforms might not
//! support pen buttons, or the distance axis, etc. Very few platforms can even report what
//! functionality the pen supports in the first place, so best practices is to either build UI to
//! let the user configure their pens, or be prepared to handle new functionality for a pen the
//! first time an event is reported.

// rustdoc imports
#[allow(unused_imports)]
use crate::{SDL_EventType, SDL_PenAxisEvent};

mod constants;
mod enumerations;
mod types;

pub use constants::*;
pub use enumerations::*;
pub use types::*;
