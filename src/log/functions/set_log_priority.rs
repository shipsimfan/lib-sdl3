use crate::SDL_LogPriority;
use std::ffi::c_int;

#[link(name = "SDL3")]
extern "C" {
    /// Set the priority of a particular log category.
    ///
    /// # Parameters
    ///  * `category` - the category to assign a priority to.
    ///  * `priority` - the [`SDL_LogPriority`] to assign.
    ///
    /// # Thread Safety
    /// It is safe to call this function from any thread.
    pub fn SDL_SetLogPriority(category: c_int, priority: SDL_LogPriority);
}
