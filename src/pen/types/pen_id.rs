/// SDL pen instance IDs.
///
/// # Remarks
/// Zero is used to signify an invalid/null device.
///
/// These show up in pen events when SDL sees input from them. They remain consistent as long as
/// SDL can recognize a tool to be the same pen; but if a pen's digitizer table is physically
/// detached from the computer, it might get a new ID when reconnected, as SDL won't know it's the
/// same device.
///
/// These IDs are only stable within a single run of a program; the next time a program is run, the
/// pen's ID will likely be different, even if the hardware hasn't been disconnected, etc.
#[allow(non_camel_case_types)]
pub type SDL_PenID = u32;
