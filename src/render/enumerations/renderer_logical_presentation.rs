/// How the logical size is mapped to the output.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_RendererLogicalPresentation {
    /// There is no logical size in effect
    Disabled,

    /// The rendered content is stretched to the output resolution
    Stretch,

    /// The rendered content is fit to the largest dimension and the other dimension is letterboxed
    /// with the clear color
    Letterbox,

    /// The rendered content is fit to the smallest dimension and the other dimension extends
    /// beyond the output bounds
    Overscan,

    /// The rendered content is scaled up by integer multiples to fit the output resolution
    IntegerScale,
}
