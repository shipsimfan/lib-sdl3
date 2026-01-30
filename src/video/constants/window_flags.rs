use crate::SDL_WindowFlags;

/// window is in fullscreen mode
pub const SDL_WINDOW_FULLSCREEN: SDL_WindowFlags = 0x0000000000000001;

/// window usable with OpenGL context
pub const SDL_WINDOW_OPENGL: SDL_WindowFlags = 0x0000000000000002;

/// window is occluded
pub const SDL_WINDOW_OCCLUDED: SDL_WindowFlags = 0x0000000000000004;

/// window is neither mapped onto the desktop nor shown in the taskbar/dock/window list;
/// [`SDL_ShowWindow`] is required for it to become visible
pub const SDL_WINDOW_HIDDEN: SDL_WindowFlags = 0x0000000000000008;

/// no window decoration
pub const SDL_WINDOW_BORDERLESS: SDL_WindowFlags = 0x0000000000000010;

/// window can be resized
pub const SDL_WINDOW_RESIZABLE: SDL_WindowFlags = 0x0000000000000020;

/// window is minimized
pub const SDL_WINDOW_MINIMIZED: SDL_WindowFlags = 0x0000000000000040;

/// window is maximized
pub const SDL_WINDOW_MAXIMIZED: SDL_WindowFlags = 0x0000000000000080;

/// window has grabbed mouse input
pub const SDL_WINDOW_MOUSE_GRABBED: SDL_WindowFlags = 0x0000000000000100;

/// window has input focus
pub const SDL_WINDOW_INPUT_FOCUS: SDL_WindowFlags = 0x0000000000000200;

/// window has mouse focus
pub const SDL_WINDOW_MOUSE_FOCUS: SDL_WindowFlags = 0x0000000000000400;

/// window not created by SDL
pub const SDL_WINDOW_EXTERNAL: SDL_WindowFlags = 0x0000000000000800;

/// window is modal
pub const SDL_WINDOW_MODAL: SDL_WindowFlags = 0x0000000000001000;

/// window uses high pixel density back buffer if possible
pub const SDL_WINDOW_HIGH_PIXEL_DENSITY: SDL_WindowFlags = 0x0000000000002000;

/// window has mouse captured (unrelated to [`SDL_WINDOW_MOUSE_GRABBED`])
pub const SDL_WINDOW_MOUSE_CAPTURE: SDL_WindowFlags = 0x0000000000004000;

/// window has relative mode enabled
pub const SDL_WINDOW_MOUSE_RELATIVE_MODE: SDL_WindowFlags = 0x0000000000008000;

/// window should always be above others
pub const SDL_WINDOW_ALWAYS_ON_TOP: SDL_WindowFlags = 0x0000000000010000;

/// window should be treated as a utility window, not showing in the task bar and window list
pub const SDL_WINDOW_UTILITY: SDL_WindowFlags = 0x0000000000020000;

/// window should be treated as a tooltip and does not get mouse or keyboard focus, requires a
/// parent window
pub const SDL_WINDOW_TOOLTIP: SDL_WindowFlags = 0x0000000000040000;

/// window should be treated as a popup menu, requires a parent window
pub const SDL_WINDOW_POPUP_MENU: SDL_WindowFlags = 0x0000000000080000;

/// window has grabbed keyboard input
pub const SDL_WINDOW_KEYBOARD_GRABBED: SDL_WindowFlags = 0x0000000000100000;

/// window is in fill-document mode (Emscripten only), since SDL 3.4.0
pub const SDL_WINDOW_FILL_DOCUMENT: SDL_WindowFlags = 0x0000000000200000;

/// window usable for Vulkan surface
pub const SDL_WINDOW_VULKAN: SDL_WindowFlags = 0x0000000010000000;

/// window usable for Metal view
pub const SDL_WINDOW_METAL: SDL_WindowFlags = 0x0000000020000000;

/// window with transparent buffer
pub const SDL_WINDOW_TRANSPARENT: SDL_WindowFlags = 0x0000000040000000;

/// window should not be focusable
pub const SDL_WINDOW_NOT_FOCUSABLE: SDL_WindowFlags = 0x0000000080000000;
