use crate::SDL_Keymod;

/// no modifier is applicable.
pub const SDL_KMOD_NONE: SDL_Keymod = 0x0000;

/// the left Shift key is down.
pub const SDL_KMOD_LSHIFT: SDL_Keymod = 0x0001;

/// the right Shift key is down.
pub const SDL_KMOD_RSHIFT: SDL_Keymod = 0x0002;

/// the Level 5 Shift key is down.
pub const SDL_KMOD_LEVEL5: SDL_Keymod = 0x0004;

/// the left Ctrl (Control) key is down.
pub const SDL_KMOD_LCTRL: SDL_Keymod = 0x0040;

/// the right Ctrl (Control) key is down.
pub const SDL_KMOD_RCTRL: SDL_Keymod = 0x0080;

/// the left Alt key is down.
pub const SDL_KMOD_LALT: SDL_Keymod = 0x0100;

/// the right Alt key is down.
pub const SDL_KMOD_RALT: SDL_Keymod = 0x0200;

/// the left GUI key (often the Windows key) is down.
pub const SDL_KMOD_LGUI: SDL_Keymod = 0x0400;

/// the right GUI key (often the Windows key) is down.
pub const SDL_KMOD_RGUI: SDL_Keymod = 0x0800;

/// the Num Lock key (may be located on an extended keypad) is down.
pub const SDL_KMOD_NUM: SDL_Keymod = 0x1000;

/// the Caps Lock key is down.
pub const SDL_KMOD_CAPS: SDL_Keymod = 0x2000;

/// the !AltGr key is down.
pub const SDL_KMOD_MODE: SDL_Keymod = 0x4000;

/// the Scroll Lock key is down.
pub const SDL_KMOD_SCROLL: SDL_Keymod = 0x8000;

/// Any Ctrl key is down.
pub const SDL_KMOD_CTRL: SDL_Keymod = SDL_KMOD_LCTRL | SDL_KMOD_RCTRL;

/// Any Shift key is down.
pub const SDL_KMOD_SHIFT: SDL_Keymod = SDL_KMOD_LSHIFT | SDL_KMOD_RSHIFT;

/// Any Alt key is down.
pub const SDL_KMOD_ALT: SDL_Keymod = SDL_KMOD_LALT | SDL_KMOD_RALT;

/// Any GUI key is down.
pub const SDL_KMOD_GUI: SDL_Keymod = SDL_KMOD_LGUI | SDL_KMOD_RGUI;
