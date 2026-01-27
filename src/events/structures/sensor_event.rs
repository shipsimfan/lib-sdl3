use crate::{SDL_EventType, SDL_SensorID};
use std::ffi::c_float;

/// Sensor event structure (event.sensor.*)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_camel_case_types)]
pub struct SDL_SensorEvent {
    /// [`SDL_EventType::SensorUpdate`]
    pub r#type: SDL_EventType,

    #[allow(missing_docs)]
    pub reserved: u32,

    /// In nanoseconds, populated using [`SDL_GetTicksNS`]
    pub timestamp: u64,

    /// The instance ID of the sensor
    pub which: SDL_SensorID,

    /// Up to 6 values from the sensor - additional values can be queried using
    /// [`SDL_GetSensorData`]
    pub data: [c_float; 6],

    /// The timestamp of the sensor reading in nanoseconds, not necessarily synchronized with the
    /// system clock
    pub sensor_timestamp: u64,
}

impl Default for SDL_SensorEvent {
    fn default() -> Self {
        SDL_SensorEvent {
            r#type: SDL_EventType::SensorUpdate,
            reserved: 0,
            timestamp: 0,
            which: 0,
            data: [0.0; 6],
            sensor_timestamp: 0,
        }
    }
}
