use crate::SDL_InitFlags;

// rustdoc imports
#[allow(unused_imports)]
use crate::{
    SDL_GetError, SDL_InitSubSystem, SDL_Quit, SDL_QuitSubSystem, SDL_SetAppMetadata,
    SDL_SetAppMetadataProperty, SDL_INIT_AUDIO, SDL_INIT_CAMERA, SDL_INIT_EVENTS, SDL_INIT_GAMEPAD,
    SDL_INIT_HAPTIC, SDL_INIT_JOYSTICK, SDL_INIT_SENSOR, SDL_INIT_VIDEO,
};

#[link(name = "SDL3")]
extern "C" {
    /// Initialize the SDL library.
    ///
    /// # Parameters
    ///  * `flags` - subsystem initialization flags.
    ///
    /// # Return Value
    /// Returns `true` on success or `false` on failure; call [`SDL_GetError`] for more
    /// information.
    ///
    /// # Remarks
    /// [`SDL_Init`] simply forwards to calling [`SDL_InitSubSystem`]. Therefore, the two may be
    /// used interchangeably. Though for readability of your code [`SDL_InitSubSystem`] might be
    /// preferred.
    ///
    /// The file I/O (for example: [`SDL_IOFromFile`]) and threading ([`SDL_CreateThread`])
    /// subsystems are initialized by default. Message boxes ([`SDL_ShowSimpleMessageBox`]) also
    /// attempt to work without initializing the video subsystem, in hopes of being useful in
    /// showing an error dialog when [`SDL_Init`] fails. You must specifically initialize other
    /// subsystems if you use them in your application.
    ///
    /// Logging (such as [`SDL_Log`]) works without initialization, too.
    ///
    /// `flags` may be any of the following OR'd together:
    ///  - [`SDL_INIT_AUDIO`]: audio subsystem; automatically initializes the events subsystem
    ///  - [`SDL_INIT_VIDEO`]: video subsystem; automatically initializes the events subsystem,
    ///                        should be initialized on the main thread.
    ///  - [`SDL_INIT_JOYSTICK`]: joystick subsystem; automatically initializes the events
    ///                           subsystem
    ///  - [`SDL_INIT_HAPTIC`]: haptic (force feedback) subsystem
    ///  - [`SDL_INIT_GAMEPAD`]: gamepad subsystem; automatically initializes the joystick
    ///                          subsystem
    ///  - [`SDL_INIT_EVENTS`]: events subsystem
    ///  - [`SDL_INIT_SENSOR`]: sensor subsystem; automatically initializes the events subsystem
    ///  - [`SDL_INIT_CAMERA`]: camera subsystem; automatically initializes the events subsystem
    ///
    /// Subsystem initialization is ref-counted, you must call [`SDL_QuitSubSystem`] for each
    /// [`SDL_InitSubSystem`] to correctly shutdown a subsystem manually (or call [`SDL_Quit`] to
    /// force shutdown). If a subsystem is already loaded then this call will increase the
    /// ref-count and return.
    ///
    /// Consider reporting some basic metadata about your application before calling [`SDL_Init`],
    /// using either [`SDL_SetAppMetadata`] or [`SDL_SetAppMetadataProperty`].
    pub fn SDL_Init(flags: SDL_InitFlags) -> bool;
}
