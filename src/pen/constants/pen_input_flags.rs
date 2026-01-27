use crate::SDL_PenInputFlags;

/// pen is pressed down
pub const SDL_PEN_INPUT_DOWN: SDL_PenInputFlags = 1 << 0;

/// button 1 is pressed
pub const SDL_PEN_INPUT_BUTTON_1: SDL_PenInputFlags = 1 << 1;

/// button 2 is pressed
pub const SDL_PEN_INPUT_BUTTON_2: SDL_PenInputFlags = 1 << 2;

/// button 3 is pressed
pub const SDL_PEN_INPUT_BUTTON_3: SDL_PenInputFlags = 1 << 3;

/// button 4 is pressed
pub const SDL_PEN_INPUT_BUTTON_4: SDL_PenInputFlags = 1 << 4;

/// button 5 is pressed
pub const SDL_PEN_INPUT_BUTTON_5: SDL_PenInputFlags = 1 << 5;

/// eraser tip is used
pub const SDL_PEN_INPUT_ERASER_TIP: SDL_PenInputFlags = 1 << 30;

/// pen is in proximity (since SDL 3.4.0)
pub const SDL_PEN_INPUT_IN_PROXIMITY: SDL_PenInputFlags = 1 << 31;
