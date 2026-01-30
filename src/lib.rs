//! SDL3 raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod audio;
pub mod camera;
pub mod error;
pub mod events;
pub mod init;
pub mod joystick;
pub mod keyboard;
pub mod keycode;
pub mod log;
pub mod mouse;
pub mod pen;
pub mod pixels;
pub mod power;
pub mod rect;
pub mod render;
pub mod scancode;
pub mod sensor;
pub mod stdinc;
pub mod surface;
pub mod timer;
pub mod touch;
pub mod video;

pub use audio::*;
pub use camera::*;
pub use error::*;
pub use events::*;
pub use init::*;
pub use joystick::*;
pub use keyboard::*;
pub use keycode::*;
pub use log::*;
pub use mouse::*;
pub use pen::*;
pub use pixels::*;
pub use power::*;
pub use rect::*;
pub use render::*;
pub use scancode::*;
pub use sensor::*;
pub use stdinc::*;
pub use surface::*;
pub use timer::*;
pub use touch::*;
pub use video::*;
