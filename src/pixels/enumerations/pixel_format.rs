/// Pixel format.
///
/// # Remarks
/// SDL's pixel formats have the following naming convention:
///  - Names with a list of components and a single bit count, such as RGB24 and ABGR32, define a
///    platform-independent encoding into bytes in the order specified. For example, in RGB24 data,
///    each pixel is encoded in 3 bytes (red, green, blue) in that order, and in ABGR32 data, each
///    pixel is encoded in 4 bytes (alpha, blue, green, red) in that order. Use these names if the
///    property of a format that is important to you is the order of the bytes in memory or on
///    disk.
///  - Names with a bit count per component, such as ARGB8888 and XRGB1555, are "packed" into an
///    appropriately-sized integer in the platform's native endianness. For example, ARGB8888 is a
///    sequence of 32-bit integers; in each integer, the most significant bits are alpha, and the
///    least significant bits are blue. On a little-endian CPU such as x86, the least significant
///    bits of each integer are arranged first in memory, but on a big-endian CPU such as s390x,
///    the most significant bits are arranged first. Use these names if the property of a format
///    that is important to you is the meaning of each bit position within a native-endianness
///    integer.
///  - In indexed formats such as [`SDL_PixelFormat::Index4Lsb`], each pixel is represented by
///    encoding an index into the palette into the indicated number of bits, with multiple pixels
///    packed into each byte if appropriate. In Lsb formats, the first (leftmost) pixel is stored
///    in the least-significant bits of the byte; in Msb formats, it's stored in the
///    most-significant bits. Index8 does not need Lsb/Msb variants, because each pixel exactly
///    fills one byte.
///
/// The 32-bit byte-array encodings such as RGBA32 are aliases for the appropriate 8888 encoding
/// for the current platform. For example, RGBA32 is an alias for ABGR8888 on little-endian CPUs
/// like x86, or an alias for RGBA8888 on big-endian CPUs.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_PixelFormat {
    #[allow(missing_docs)]
    Unknown = 0,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index1, SDL_BITMAPORDER_4321, 0, 1, 0)`
    Index1Lsb = 0x11100100,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index1, SDL_BITMAPORDER_1234, 0, 1, 0)`
    Index1Msb = 0x11200100,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index2, SDL_BITMAPORDER_4321, 0, 2, 0)`
    Index2Lsb = 0x1C100200,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index2, SDL_BITMAPORDER_1234, 0, 2, 0)`
    Index2Msb = 0x1C200200,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index4, SDL_BITMAPORDER_4321, 0, 4, 0)`
    Index4Lsb = 0x12100400,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index4, SDL_BITMAPORDER_1234, 0, 4, 0)`
    Index4Msb = 0x12200400,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_Index8, 0, 0, 8, 1)`
    Index8 = 0x13000801,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED8, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_332, 8, 1)`
    RGB332 = 0x14110801,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_4444, 12, 2)`
    XRGB4444 = 0x15120C02,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_4444, 12, 2)`
    XBGR4444 = 0x15520C02,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_1555, 15, 2)`
    XRGB1555 = 0x15130F02,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_1555, 15, 2)`
    XBGR1555 = 0x15530F02,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_4444, 16, 2)`
    ARGB4444 = 0x15321002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_4444, 16, 2)`
    RGBA4444 = 0x15421002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_4444, 16, 2)`
    ABGR4444 = 0x15721002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_4444, 16, 2)`
    BGRA4444 = 0x15821002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_1555, 16, 2)`
    ARGB1555 = 0x15331002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_5551, 16, 2)`
    RGBA5551 = 0x15441002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_1555, 16, 2)`
    ABGR1555 = 0x15731002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_5551, 16, 2)`
    BGRA5551 = 0x15841002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_565, 16, 2)`
    RGB565 = 0x15151002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED16, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_565, 16, 2)`
    BGR565 = 0x15551002,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_RGB, 0, 24, 3)`
    RGB24 = 0x17101803,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU8, SDL_ARRAYORDER_BGR, 0, 24, 3)`
    BGR24 = 0x17401803,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_8888, 24, 4)`
    XRGB8888 = 0x16161804,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBX, SDL_PACKEDLAYOUT_8888, 24, 4)`
    RGBX8888 = 0x16261804,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_8888, 24, 4)`
    XBGR8888 = 0x16561804,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRX, SDL_PACKEDLAYOUT_8888, 24, 4)`
    BGRX8888 = 0x16661804,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_8888, 32, 4)`
    ARGB8888 = 0x16362004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_RGBA, SDL_PACKEDLAYOUT_8888, 32, 4)`
    RGBA8888 = 0x16462004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_8888, 32, 4)`
    ABGR8888 = 0x16762004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_BGRA, SDL_PACKEDLAYOUT_8888, 32, 4)`
    BGRA8888 = 0x16862004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XRGB, SDL_PACKEDLAYOUT_2101010, 32, 4)`
    XRGB2101010 = 0x16172004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_XBGR, SDL_PACKEDLAYOUT_2101010, 32, 4)`
    XBGR2101010 = 0x16572004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ARGB, SDL_PACKEDLAYOUT_2101010, 32, 4)`
    ARGB2101010 = 0x16372004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_PACKED32, SDL_PACKEDORDER_ABGR, SDL_PACKEDLAYOUT_2101010, 32, 4)`
    ABGR2101010 = 0x16772004,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGB, 0, 48, 6)`
    RGB48 = 0x18103006,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGR, 0, 48, 6)`
    BGR48 = 0x18403006,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_RGBA, 0, 64, 8)`
    RGBA64 = 0x18204008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ARGB, 0, 64, 8)`
    ARGB64 = 0x18304008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_BGRA, 0, 64, 8)`
    BGRA64 = 0x18504008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYU16, SDL_ARRAYORDER_ABGR, 0, 64, 8)`
    ABGR64 = 0x18604008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGB, 0, 48, 6)`
    RGB48Float = 0x1A103006,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGR, 0, 48, 6)`
    BGR48Float = 0x1A403006,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_RGBA, 0, 64, 8)`
    RGBA64Float = 0x1A204008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ARGB, 0, 64, 8)`
    ARGB64Float = 0x1A304008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_BGRA, 0, 64, 8)`
    BGRA64Float = 0x1A504008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF16, SDL_ARRAYORDER_ABGR, 0, 64, 8)`
    ABGR64Float = 0x1A604008,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGB, 0, 96, 12)`
    RGB96Float = 0x1B10600C,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGR, 0, 96, 12)`
    BGR96Float = 0x1B40600C,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_RGBA, 0, 128, 16)`
    RGBA128Float = 0x1B208010,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ARGB, 0, 128, 16)`
    ARGB128Float = 0x1B308010,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_BGRA, 0, 128, 16)`
    BGRA128Float = 0x1B508010,

    /// `SDL_DEFINE_PIXELFORMAT!(SDL_PIXELTYPE_ARRAYF32, SDL_ARRAYORDER_ABGR, 0, 128, 16)`
    ABGR128Float = 0x1B608010,

    /// Planar mode: Y + V + U  (3 planes)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('Y', 'V', '1', '2')`
    YV12 = 0x32315659,

    /// Planar mode: Y + U + V  (3 planes)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('I', 'Y', 'U', 'V')`
    IYUV = 0x56555949,

    /// Packed mode: Y0+U0+Y1+V0 (1 plane)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('Y', 'U', 'Y', '2')`
    YUY2 = 0x32595559,

    /// Packed mode: U0+Y0+V0+Y1 (1 plane)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('U', 'Y', 'V', 'Y')`
    UYVY = 0x59565955,

    /// Packed mode: Y0+V0+Y1+U0 (1 plane)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('Y', 'V', 'Y', 'U')`
    YVYU = 0x55595659,

    /// Planar mode: Y + U/V interleaved  (2 planes)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('N', 'V', '1', '2')`
    NV12 = 0x3231564E,

    /// Planar mode: Y + V/U interleaved  (2 planes)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('N', 'V', '2', '1')`
    NV21 = 0x3132564E,

    /// Planar mode: Y + U/V interleaved  (2 planes)
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('P', '0', '1', '0')`
    P010 = 0x30313050,

    /// Android video texture format
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('O', 'E', 'S'` ' ')
    ExternalOes = 0x2053454F,

    /// Motion JPEG
    ///
    /// `SDL_DEFINE_PIXELFOURCC!('M', 'J', 'P'` 'G')
    MJpg = 0x47504A4D,
}
