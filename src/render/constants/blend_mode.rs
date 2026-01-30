use crate::SDL_BlendMode;

/// no blending: dstRGBA = srcRGBA
pub const SDL_BLENDMODE_NONE: SDL_BlendMode = 0x00000000;

/// alpha blending: dstRGB = (srcRGB * srcA) + (dstRGB * (1-srcA)), dstA = srcA + (dstA * (1-srcA))
pub const SDL_BLENDMODE_BLEND: SDL_BlendMode = 0x00000001;

/// pre-multiplied alpha blending: dstRGBA = srcRGBA + (dstRGBA * (1-srcA))
pub const SDL_BLENDMODE_BLEND_PREMULTIPLIED: SDL_BlendMode = 0x00000010;

/// additive blending: dstRGB = (srcRGB * srcA) + dstRGB, dstA = dstA
pub const SDL_BLENDMODE_ADD: SDL_BlendMode = 0x00000002;

/// pre-multiplied additive blending: dstRGB = srcRGB + dstRGB, dstA = dstA
pub const SDL_BLENDMODE_ADD_PREMULTIPLIED: SDL_BlendMode = 0x00000020;

/// color modulate: dstRGB = srcRGB * dstRGB, dstA = dstA
pub const SDL_BLENDMODE_MOD: SDL_BlendMode = 0x00000004;

/// color multiply: dstRGB = (srcRGB * dstRGB) + (dstRGB * (1-srcA)), dstA = dstA
pub const SDL_BLENDMODE_MUL: SDL_BlendMode = 0x00000008;

#[allow(missing_docs)]
pub const SDL_BLENDMODE_INVALID: SDL_BlendMode = 0x7FFFFFFF;
