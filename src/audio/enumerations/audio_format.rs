/// Audio format.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_AudioFormat {
    /// Unspecified audio format
    Unknown = 0x0000,

    /// Unsigned 8-bit samples
    U8 = 0x0008,

    /// Signed 8-bit samples
    S8 = 0x8008,

    /// Signed 16-bit samples
    S16Le = 0x8010,

    /// As above, but big-endian byte order
    S16Be = 0x9010,

    /// 32-bit integer samples
    S32Le = 0x8020,

    /// As above, but big-endian byte order
    S32Be = 0x9020,

    /// 32-bit floating point samples
    F32Le = 0x8120,

    /// As above, but big-endian byte order
    F32Be = 0x9120,
}
