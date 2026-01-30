use crate::{SDL_PixelFormat, SDL_SurfaceFlags};
use std::{
    ffi::{c_int, c_void},
    ptr::null_mut,
};

/// A collection of pixels used in software blitting.
///
/// # Remarks
/// Pixels are arranged in memory in rows, with the top row first. Each row occupies an amount of
/// memory given by the pitch (sometimes known as the row stride in non-SDL APIs).
///
/// Within each row, pixels are arranged from left to right until the width is reached. Each pixel
/// occupies a number of bits appropriate for its format, with most formats representing each pixel
/// as one or more whole bytes (in some indexed formats, instead multiple pixels are packed into
/// each byte), and a byte order given by the format. After encoding all pixels, any remaining
/// bytes to reach the pitch are used as padding to reach a desired alignment, and have undefined
/// contents.
///
/// When a surface holds YUV format data, the planes are assumed to be contiguous without padding
/// between them, e.g. a 32x32 surface in NV12 format with a pitch of 32 would consist of 32x32
/// bytes of Y plane followed by 32x16 bytes of UV plane.
///
/// When a surface holds MJPG format data, pixels points at the compressed JPEG image and pitch is
/// the length of that data.
#[repr(C)]
#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct SDL_Surface {
    /// The flags of the surface, read-only
    pub flags: SDL_SurfaceFlags,

    /// The format of the surface, read-only
    pub format: SDL_PixelFormat,

    /// The width of the surface, read-only.
    pub w: c_int,

    /// The height of the surface, read-only.
    pub h: c_int,

    /// The distance in bytes between rows of pixels, read-only
    pub pitch: c_int,

    ///  A pointer to the pixels of the surface, the pixels are writeable if non-[`null_mut`]
    pub pixels: *mut c_void,

    /// Application reference count, used when freeing surface
    pub refcount: c_int,

    /// Reserved for internal use
    pub reserved: *mut c_void,
}

impl Default for SDL_Surface {
    fn default() -> Self {
        SDL_Surface {
            flags: 0,
            format: SDL_PixelFormat::Unknown,
            w: 0,
            h: 0,
            pitch: 0,
            pixels: null_mut(),
            refcount: 0,
            reserved: null_mut(),
        }
    }
}
