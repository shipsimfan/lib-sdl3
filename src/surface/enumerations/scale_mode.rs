/// The scaling mode.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_ScaleMode {
    #[allow(missing_docs)]
    Invalid = -1,

    /// nearest pixel sampling
    Nearest,

    /// linear filtering
    Linear,

    /// nearest pixel sampling with improved scaling for pixel art
    PixelArt,
}
