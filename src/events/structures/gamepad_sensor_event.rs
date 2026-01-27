use crate::{SDL_EventType, SDL_JoystickID};
use std::ffi::c_float;

/// Gamepad sensor event structure (event.gsensor.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_GamepadSensorEvent {
    /// [`SDL_EventType::GamepadSensorUpdate`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The joystick instance id
    pub which: SDL_JoystickID,

    /// The type of the sensor, one of the values of [`SDL_SensorType`]
    pub sensor: i32,

    /// Up to 3 values from the sensor
    pub data: [c_float; 3],

    /// The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the system clock
    pub sensor_timestamp: u64,
}

impl Default for SDL_GamepadSensorEvent {
    fn default() -> Self {
        SDL_GamepadSensorEvent {
            r#type: SDL_EventType::GamepadSensorUpdate,
            reserved: 0,
            timestamp: 0,
            which: 0,
            sensor: 0,
            data: [0.0; 3],
            sensor_timestamp: 0,
        }
    }
}
