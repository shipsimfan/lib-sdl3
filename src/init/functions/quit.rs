// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_QuitSubSystem;

#[link(name = "SDL3")]
unsafe extern "C" {
    /// Clean up all initialized subsystems.
    ///
    /// # Remarks
    /// You should call this function even if you have already shutdown each initialized subsystem
    /// with [`SDL_QuitSubSystem`]. It is safe to call this function even in the case of errors in
    /// initialization.
    ///
    /// You can use this function with `atexit` to ensure that it is run when your application is
    /// shutdown, but it is not wise to do this from a library or other dynamically loaded code.
    pub fn SDL_Quit();
}
