/// This is a unique ID for a keyboard for the time it is connected to the system, and is never
/// reused for the lifetime of the application.
///
/// # Remarks
/// If the keyboard is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
#[allow(non_camel_case_types)]
pub type SDL_KeyboardID = u32;
