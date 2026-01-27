// #define SDL_BUTTON_MASK(X)  (1u << ((X)-1))

#[macro_export]
#[allow(missing_docs)]
macro_rules! SDL_BUTTON_MASK {
    ($x: expr) => {
        1 << ($x - 1)
    };
}
