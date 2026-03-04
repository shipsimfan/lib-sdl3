use crate::SDL_PixelFormat;

/// Details about the format of a pixel.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_PixelFormatDetails {
    #[allow(missing_docs)]
    pub format: SDL_PixelFormat,

    #[allow(missing_docs)]
    pub bits_per_pixel: u8,

    #[allow(missing_docs)]
    pub bytes_per_pixel: u8,

    #[allow(missing_docs)]
    pub padding: [u8; 2],

    #[allow(missing_docs)]
    pub r_mask: u32,

    #[allow(missing_docs)]
    pub g_mask: u32,

    #[allow(missing_docs)]
    pub b_mask: u32,

    #[allow(missing_docs)]
    pub a_mask: u32,

    #[allow(missing_docs)]
    pub r_bits: u8,

    #[allow(missing_docs)]
    pub g_bits: u8,

    #[allow(missing_docs)]
    pub b_bits: u8,

    #[allow(missing_docs)]
    pub a_bits: u8,

    #[allow(missing_docs)]
    pub r_shift: u8,

    #[allow(missing_docs)]
    pub g_shift: u8,

    #[allow(missing_docs)]
    pub b_shift: u8,

    #[allow(missing_docs)]
    pub a_shift: u8,
}

impl Default for SDL_PixelFormatDetails {
    fn default() -> Self {
        SDL_PixelFormatDetails {
            format: SDL_PixelFormat::Unknown,
            bits_per_pixel: 0,
            bytes_per_pixel: 0,
            padding: [0; 2],
            r_mask: 0,
            g_mask: 0,
            b_mask: 0,
            a_mask: 0,
            r_bits: 0,
            g_bits: 0,
            b_bits: 0,
            a_bits: 0,
            r_shift: 0,
            g_shift: 0,
            b_shift: 0,
            a_shift: 0,
        }
    }
}
