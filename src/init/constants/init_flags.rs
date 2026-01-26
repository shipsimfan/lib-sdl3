use crate::SDL_InitFlags;

/// audio subsystem; automatically initializes the events subsystem
pub const SDL_INIT_AUDIO: SDL_InitFlags = 0x00000010;

/// video subsystem; automatically initializes the events subsystem, should be initialized on the
/// main thread.
pub const SDL_INIT_VIDEO: SDL_InitFlags = 0x00000020;

/// joystick subsystem; automatically initializes the events subsystem
pub const SDL_INIT_JOYSTICK: SDL_InitFlags = 0x00000200;

/// haptic (force feedback) subsystem
pub const SDL_INIT_HAPTIC: SDL_InitFlags = 0x00001000;

/// gamepad subsystem; automatically initializes the joystick subsystem
pub const SDL_INIT_GAMEPAD: SDL_InitFlags = 0x00002000;

/// events subsystem
pub const SDL_INIT_EVENTS: SDL_InitFlags = 0x00004000;

/// sensor subsystem; automatically initializes the events subsystem
pub const SDL_INIT_SENSOR: SDL_InitFlags = 0x00008000;

/// camera subsystem; automatically initializes the events subsystem
pub const SDL_INIT_CAMERA: SDL_InitFlags = 0x00010000;
