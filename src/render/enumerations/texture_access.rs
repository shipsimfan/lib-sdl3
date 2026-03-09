/// The access pattern allowed for a texture.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_TextureAccess {
    /// Changes rarely, not lockable
    Static,

    /// Changes frequently, lockable
    Streaming,

    /// Texture can be used as a render target
    Target,
}
