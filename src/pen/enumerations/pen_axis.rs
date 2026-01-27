// rustdoc imports
#[allow(unused_imports)]
use crate::SDL_PenAxisEvent;

/// Pen axis indices.
///
/// # Remarks
/// These are the valid values for the axis field in [`SDL_PenAxisEvent`]. All axes are either
/// normalised to 0..1 or report a (positive or negative) angle in degrees, with 0.0 representing
/// the centre. Not all pens/backends support all axes: unsupported axes are always zero.
///
/// To convert angles for tilt and rotation into vector representation, use [`SDL_sinf`] on the
/// [`SDL_PenAxis::XTilt`], [`SDL_PenAxis::YTilt`], or [`SDL_PenAxis::Rotation`] component, for
/// example: `SDL_sinf(xtilt * SDL_PI_F / 180.0)`.
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum SDL_PenAxis {
    /// Pen pressure.  Unidirectional: 0 to 1.0
    Pressure,

    /// Pen horizontal tilt angle.  Bidirectional: -90.0 to 90.0 (left-to-right).
    XTilt,

    /// Pen vertical tilt angle.  Bidirectional: -90.0 to 90.0 (top-to-down).
    YTilt,

    /// Pen distance to drawing surface.  Unidirectional: 0.0 to 1.0
    Distance,

    /// Pen barrel rotation.  Bidirectional: -180 to 179.9 (clockwise, 0 is facing up, -180.0 is
    /// facing down).
    Rotation,

    /// Pen finger wheel or slider (e.g., Airbrush Pen).  Unidirectional: 0 to 1.0
    Slider,

    /// Pressure from squeezing the pen ("barrel pressure").
    TangentialPressure,

    /// Total known pen axis types in this version of SDL. This number may grow in future releases!
    Count,
}
