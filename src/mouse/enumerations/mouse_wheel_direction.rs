#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_MouseWheelDirection {
    /// The scroll direction is normal
    Normal,

    /// The scroll direction is flipped / natural
    Flipped,
}
