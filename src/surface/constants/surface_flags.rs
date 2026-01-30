use crate::SDL_SurfaceFlags;

/// Surface uses preallocated pixel memory
pub const SDL_SURFACE_PREALLOCATED: SDL_SurfaceFlags = 0x00000001;

/// Surface needs to be locked to access pixels
pub const SDL_SURFACE_LOCK_NEEDED: SDL_SurfaceFlags = 0x00000002;

/// Surface is currently locked
pub const SDL_SURFACE_LOCKED: SDL_SurfaceFlags = 0x00000004;

/// Surface uses pixel memory allocated with [`SDL_aligned_alloc`]
pub const SDL_SURFACE_SIMD_ALIGNED: SDL_SurfaceFlags = 0x00000008;
