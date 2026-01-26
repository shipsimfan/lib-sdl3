//! SDL3 raw bindings

#![deny(missing_docs)]
#![deny(rustdoc::private_intra_doc_links)]
#![deny(rustdoc::unescaped_backticks)]
#![deny(rustdoc::redundant_explicit_links)]
#![warn(rustdoc::broken_intra_doc_links)]

pub mod error;
pub mod events;
pub mod init;
pub mod log;
pub mod video;

pub use error::*;
pub use events::*;
pub use init::*;
pub use log::*;
pub use video::*;
