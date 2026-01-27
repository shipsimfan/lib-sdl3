/// A unique ID for a touch device.
///
/// # Remarks
/// This ID is valid for the time the device is connected to the system, and is never reused for
/// the lifetime of the application.
///
/// The value 0 is an invalid ID.
#[allow(non_camel_case_types)]
pub type SDL_TouchID = u64;
