mod clear_error;
mod get_error;
mod out_of_memory;
mod set_error;

pub use clear_error::SDL_ClearError;
pub use get_error::SDL_GetError;
pub use out_of_memory::SDL_OutOfMemory;
pub use set_error::SDL_SetError;
