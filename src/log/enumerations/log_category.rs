// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_LogPriority;

/// The predefined log categories
///
/// # Remarks
/// By default the application and gpu categories are enabled at the [`SDL_LogPriority::Info`]
/// level, the assert category is enabled at the [`SDL_LogPriority::Warn`] level, test is enabled
/// at the [`SDL_LogPriority::Verbose`] level and all other categories are enabled at the
/// [`SDL_LogPriority::Error`] level.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_LogCategory {
    Application,
    Error,
    Assert,
    System,
    Audio,
    Video,
    Render,
    Input,
    Test,
    Gpu,
    Reserved2,
    Reserved3,
    Reserved4,
    Reserved5,
    Reserved6,
    Reserved7,
    Reserved8,
    Reserved9,
    Reserved10,
    Custom,
}
