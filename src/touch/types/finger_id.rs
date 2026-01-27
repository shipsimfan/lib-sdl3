/// A unique ID for a single finger on a touch device.
///
/// # Remarks
/// This ID is valid for the time the finger (stylus, etc) is touching and will be unique for all
/// fingers currently in contact, so this ID tracks the lifetime of a single continuous touch. This
/// value may represent an index, a pointer, or some other unique ID, depending on the platform.
///
/// The value 0 is an invalid ID.
#[allow(non_camel_case_types)]
pub type SDL_FingerID = u64;
