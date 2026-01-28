/// The basic state for the system's power supply.
///
/// # Remarks
/// These are results returned by [`SDL_GetPowerInfo`].
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SDL_PowerState {
    /// error determining power status
    Error = -1,

    /// cannot determine power status
    Unknown,

    /// Not plugged in, running on the battery
    OnBattery,

    /// Plugged in, no battery available
    NoBattery,

    /// Plugged in, charging battery
    Charging,

    /// Plugged in, battery charged
    Charged,
}
