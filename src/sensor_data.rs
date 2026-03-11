use crate::{
    registers::{BMM350_LSB_TO_DEGC, BMM350_LSB_TO_UT_XY, BMM350_LSB_TO_UT_Z, BMM350_TEMP_OFFSET},
    types::{Sensor3DData, Sensor3DDataScaled},
    MagCompensation,
};

/// Standard gravity in m/s^2
pub const GRAVITY: f32 = 9.81;

impl Sensor3DData {
    /// Convert raw magnetometer data to uT
    ///
    /// # Arguments
    ///
    /// * `scale` - The full scale value in uT
    pub(crate) fn to_ut(&self, mag_comp: &MagCompensation) -> Sensor3DDataScaled {
        let c = &mag_comp;

        let mut d = [
            self.x as f32 * BMM350_LSB_TO_UT_XY,
            self.y as f32 * BMM350_LSB_TO_UT_XY,
            self.z as f32 * BMM350_LSB_TO_UT_Z,
            self.t as f32 * BMM350_LSB_TO_DEGC - BMM350_TEMP_OFFSET,
        ];

        // Temperature compensation
        d[3] = (1.0 + c.dut_sensit_coef.t_sens) * d[3] + c.dut_offset_coef.t_offs;

        let offset = [
            c.dut_offset_coef.offset_x,
            c.dut_offset_coef.offset_y,
            c.dut_offset_coef.offset_z,
        ];
        let sens = [
            c.dut_sensit_coef.sens_x,
            c.dut_sensit_coef.sens_y,
            c.dut_sensit_coef.sens_z,
        ];
        let tco = [c.dut_tco.tco_x, c.dut_tco.tco_y, c.dut_tco.tco_z];
        let tcs = [c.dut_tcs.tcs_x, c.dut_tcs.tcs_y, c.dut_tcs.tcs_z];
        let dt = d[3] - c.dut_t0;

        for i in 0..3 {
            d[i] *= 1.0 + sens[i];
            d[i] += offset[i];
            d[i] += tco[i] * dt;
            d[i] /= 1.0 + tcs[i] * dt;
        }

        // Cross-axis correction
        let denom = 1.0 - c.cross_axis.cross_y_x * c.cross_axis.cross_x_y;
        let cx = (d[0] - c.cross_axis.cross_x_y * d[1]) / denom;
        let cy = (d[1] - c.cross_axis.cross_y_x * d[0]) / denom;
        let cz = d[2]
            + (d[0] * (c.cross_axis.cross_y_x * c.cross_axis.cross_z_y - c.cross_axis.cross_z_x)
                - d[1]
                    * (c.cross_axis.cross_z_y - c.cross_axis.cross_x_y * c.cross_axis.cross_z_x))
                / denom;

        Sensor3DDataScaled {
            x: cx,
            y: cy,
            z: cz,
            t: d[3],
        }
    }
}
