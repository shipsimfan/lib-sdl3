// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PixelFormat;

/// A structure that represents a color as RGBA components.
///
/// # Remarks
/// The bits of this structure can be directly reinterpreted as an integer-packed color which uses
/// the [`SDL_PixelFormat::RGBA32`] format ([`SDL_PixelFormat::ABGR8888`] on little-endian systems
/// and [`SDL_PixelFormat::RGBA8888`] on big-endian systems).
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Color {
    #[allow(missing_docs)]
    pub r: u8,

    #[allow(missing_docs)]
    pub g: u8,

    #[allow(missing_docs)]
    pub b: u8,

    #[allow(missing_docs)]
    pub a: u8,
}

impl Default for SDL_Color {
    fn default() -> Self {
        SDL_Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}
