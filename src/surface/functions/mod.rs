mod create_surface;
mod destroy_surface;
mod lock_surface;
mod unlock_surface;

pub use create_surface::SDL_CreateSurface;
pub use destroy_surface::SDL_DestroySurface;
pub use lock_surface::SDL_LockSurface;
pub use unlock_surface::SDL_UnlockSurface;
