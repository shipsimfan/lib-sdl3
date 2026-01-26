/// Return values for optional main callbacks.
///
/// # Remarks
/// Returning [`SDL_AppResult::Success`] or [`SDL_AppResult::Failure`] from [`SDL_AppInit`],
/// [`SDL_AppEvent`], or [`SDL_AppIterate`] will terminate the program and report success/failure
/// to the operating system. What that means is platform-dependent. On Unix, for example, on
/// success, the process error code will be zero, and on failure it will be 1. This interface
/// doesn't allow you to return specific exit codes, just whether there was an error generally or
/// not.
///
/// Returning [`SDL_AppResult::Continue`] from these functions will let the app continue to run.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_AppResult {
    Continue,
    Success,
    Failure,
}
