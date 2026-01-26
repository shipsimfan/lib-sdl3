/// The predefined log priorities
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_LogPriority {
    Invalid,
    Trace,
    Verbose,
    Debug,
    Info,
    Warn,
    Error,
    Critical,
    Count,
}
