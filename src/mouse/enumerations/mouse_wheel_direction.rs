/// Scroll direction types for the Scroll event
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_MouseWheelDirection {
    /// The scroll direction is normal
    Normal,

    /// The scroll direction is flipped / natural
    Flipped,
}
