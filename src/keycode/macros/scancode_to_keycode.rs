// #define SDL_SCANCODE_TO_KEYCODE(X)  (X | SDLK_SCANCODE_MASK)

#[allow(missing_docs)]
#[macro_export]
macro_rules! SDL_SCANCODE_TO_KEYCODE {
    ($x: expr) => {
        $x | $crate::SDLK_SCANCODE_MASK
    };
}
