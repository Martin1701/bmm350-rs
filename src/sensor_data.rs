use crate::types::{Sensor3DData, Sensor3DDataScaled};

/// Standard gravity in m/s^2
pub const GRAVITY: f32 = 9.81;

impl Sensor3DData {
    /// Convert raw sensor data to scaled values
    ///
    /// # Arguments
    ///
    /// * `scale` - The full scale value in uT
    fn to_scaled(&self, scale: f32) -> Sensor3DDataScaled {
        let half_scale = scale / 2.0;
        Sensor3DDataScaled {
            x: Self::lsb_to_scaled(self.x, scale, half_scale),
            y: Self::lsb_to_scaled(self.y, scale, half_scale),
            z: Self::lsb_to_scaled(self.z, scale, half_scale),
            temperature: Self::lsb_to_scaled(self.temperature, scale, half_scale), //TODO fix
        }
    }

    /// Convert raw LSB value to scaled value
    ///
    /// # Arguments
    ///
    /// * `val` - Raw LSB value
    /// * `scale` - The full scale value
    /// * `half_scale` - Half of the full scale value
    fn lsb_to_scaled(val: i32, scale: f32, half_scale: f32) -> f32 {
        (scale * val as f32) / half_scale
    }

    /// Convert raw magnetometer data to uT
    ///
    /// # Arguments
    ///
    /// * `scale` - The full scale value in uT
    pub fn to_ut(&self, scale: f32) -> Sensor3DDataScaled {
        self.to_scaled(scale)
    }
}
