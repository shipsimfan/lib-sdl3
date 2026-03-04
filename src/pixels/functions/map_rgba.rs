use crate::{SDL_Palette, SDL_PixelFormatDetails};

// rustdoc imports
#[allow(unused_imports)]
use std::ptr::null;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Map an RGBA quadruple to a pixel value for a given pixel format.
    ///
    /// # Parameters
    ///  * `format` - a pointer to [`SDL_PixelFormatDetails`] describing the pixel format.
    ///  * `palette` - an optional palette for indexed formats, may be [`null`].
    ///  * `r` - the red component of the pixel in the range 0-255.
    ///  * `g` - the green component of the pixel in the range 0-255.
    ///  * `b` - the blue component of the pixel in the range 0-255.
    ///  * `a` - the alpha component of the pixel in the range 0-255.
    ///
    /// # Return Value
    /// Returns a pixel value.
    ///
    /// # Remarks
    /// This function maps the RGBA color value to the specified pixel format and returns the pixel
    /// value best approximating the given RGBA color value for the given pixel format.
    ///
    /// If the specified pixel format has no alpha component the alpha value will be ignored (as it
    /// will be in formats with a palette).
    ///
    /// If the format has a palette (8-bit) the index of the closest matching color in the palette
    /// will be returned.
    ///
    /// If the pixel format bpp (color depth) is less than 32-bpp then the unused upper bits of the
    /// return value can safely be ignored (e.g., with a 16-bpp format the return value can be
    /// assigned to a Uint16, and similarly a Uint8 for an 8-bpp format).
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread, as long as the palette is not modified.
    pub fn SDL_MapRGBA(
        format: *const SDL_PixelFormatDetails,
        palette: *const SDL_Palette,
        r: u8,
        g: u8,
        b: u8,
        a: u8,
    ) -> u32;
}
