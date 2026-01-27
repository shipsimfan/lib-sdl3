#![allow(missing_docs)]

use crate::{SDL_MouseButtonFlags, SDL_BUTTON_MASK};

pub const SDL_BUTTON_LEFT: SDL_MouseButtonFlags = 1;
pub const SDL_BUTTON_MIDDLE: SDL_MouseButtonFlags = 2;
pub const SDL_BUTTON_RIGHT: SDL_MouseButtonFlags = 3;
pub const SDL_BUTTON_X1: SDL_MouseButtonFlags = 4;
pub const SDL_BUTTON_X2: SDL_MouseButtonFlags = 5;

pub const SDL_BUTTON_LMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK!(SDL_BUTTON_LEFT);
pub const SDL_BUTTON_MMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK!(SDL_BUTTON_MIDDLE);
pub const SDL_BUTTON_RMASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK!(SDL_BUTTON_RIGHT);
pub const SDL_BUTTON_X1MASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK!(SDL_BUTTON_X1);
pub const SDL_BUTTON_X2MASK: SDL_MouseButtonFlags = SDL_BUTTON_MASK!(SDL_BUTTON_X2);
