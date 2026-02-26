mod create_window;
mod destroy_window;
mod get_window_id;
mod get_window_size;
mod set_window_size;

pub use create_window::SDL_CreateWindow;
pub use destroy_window::SDL_DestroyWindow;
pub use get_window_id::SDL_GetWindowID;
pub use get_window_size::SDL_GetWindowSize;
pub use set_window_size::SDL_SetWindowSize;
