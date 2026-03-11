use core::fmt::Debug;

/// Possible errors that can occur when interacting with the BMM350
#[derive(Debug)]
pub enum Error<E> {
    /// Communication error
    Comm(E),
    /// Invalid device (wrong chip ID)
    InvalidDevice,
    /// Invalid configuration
    InvalidConfig,
    /// Timeout error
    Timeout,
    /// OTP Error
    OtpError,
    /// OTP timeout
    OtpTimeout,

    ResetUnfinished,
}

/// Magnetometer power modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerMode {
    /// Suspend mode
    Suspend = 0x00,
    /// Normal mode
    Normal = 0x01,
    /// Forced mode
    Forced = 0x03,
    /// Forced mode fast
    ForcedFast = 0x04,
    FluxGuideReset = 0x05,
    FluxGuideResetFast = 0x06,
    BitReset = 0x07,
    BrFast = 0x08,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DataRate {
    ODR400Hz = 0x02,
    ODR200Hz = 0x03,
    ODR100Hz = 0x04,
    ODR50Hz = 0x05,
    ODR25Hz = 0x06,
    ODR12_5Hz = 0x07,
    ODR6_25Hz = 0x08,
    ODR3_125Hz = 0x09,
    ODR1_5625Hz = 0x0A,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerformanceMode {
    UltraLowPower = 0x00,
    LowPower = 0x01,
    Regular = 0x02,
    Enhanced = 0x03,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bandwidth {
    Normal = 0x00,
    High = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AverageNum {
    Avg1 = 0x00,
    Avg2 = 0x01,
    Avg4 = 0x02,
    Avg8 = 0x03,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AxisEnableDisable {
    Disable = 0x00,
    Enable = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptEnableDisable {
    Disable = 0x00,
    Enable = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptLatch {
    Pulsed = 0x00,
    Latched = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptPolarity {
    ActiveLow = 0x00,
    ActiveHigh = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptDrive {
    OpenDrain = 0x00,
    PushPull = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InterruptMap {
    Unmap = 0x00,
    Map = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum I2cWdtEnable {
    Disable = 0x00,
    Enable = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum I2cWdtSelect {
    Short = 0x00,
    Long = 0x01,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SelfTestMode {
    Normal = 0x00,
    PositiveX = 0x01,
    NegativeX = 0x02,
    PositiveY = 0x03,
    NegativeY = 0x04,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CtrlUser {
    Disable = 0x00,
    Enable = 0x01,
}

#[derive(Debug, Clone)]
pub struct DutOffsetCoef {
    pub offset_x: f32, // raw signed 12-bit, no divisor
    pub offset_y: f32,
    pub offset_z: f32,
    pub t_offs: f32, // signed 8-bit / 5.0
}

#[derive(Debug, Clone)]
pub struct DutSensitCoef {
    pub sens_x: f32, // signed 8-bit / 256.0
    pub sens_y: f32,
    pub sens_z: f32,
    pub t_sens: f32, // signed 8-bit / 512.0  ← different divisor!
}

#[derive(Debug, Clone)]
pub struct DutTco {
    pub tco_x: f32, // signed 8-bit / 32.0
    pub tco_y: f32,
    pub tco_z: f32,
}

#[derive(Debug, Clone)]
pub struct DutTcs {
    pub tcs_x: f32, // signed 8-bit / 16384.0
    pub tcs_y: f32,
    pub tcs_z: f32,
}

#[derive(Debug, Clone)]
pub struct CrossAxis {
    pub cross_x_y: f32, // signed 8-bit / 800.0
    pub cross_y_x: f32,
    pub cross_z_x: f32,
    pub cross_z_y: f32,
}

#[derive(Debug, Clone)]
pub struct MagCompensation {
    pub dut_offset_coef: DutOffsetCoef,
    pub dut_sensit_coef: DutSensitCoef,
    pub dut_tco: DutTco,
    pub dut_tcs: DutTcs,
    pub dut_t0: f32, // reference temperature in °C
    pub cross_axis: CrossAxis,
}

impl Default for MagCompensation {
    fn default() -> Self {
        Self {
            dut_offset_coef: DutOffsetCoef {
                offset_x: 0.0,
                offset_y: 0.0,
                offset_z: 0.0,
                t_offs: 0.0,
            },
            dut_sensit_coef: DutSensitCoef {
                sens_x: 0.0,
                sens_y: 0.0,
                sens_z: 0.0,
                t_sens: 0.0,
            },
            dut_tco: DutTco {
                tco_x: 0.0,
                tco_y: 0.0,
                tco_z: 0.0,
            },
            dut_tcs: DutTcs {
                tcs_x: 0.0,
                tcs_y: 0.0,
                tcs_z: 0.0,
            },
            dut_t0: 0.0,
            cross_axis: CrossAxis {
                cross_x_y: 0.0,
                cross_y_x: 0.0,
                cross_z_x: 0.0,
                cross_z_y: 0.0,
            },
        }
    }
}

/// 3D sensor data (raw values)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DData {
    /// X-axis value
    pub x: i32,
    /// Y-axis value
    pub y: i32,
    /// Z-axis value
    pub z: i32,
    /// Temperature value
    pub t: i32,
}

/// Scaled 3D sensor data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sensor3DDataScaled {
    /// X-axis scaled value
    pub x: f32,
    /// Y-axis scaled value
    pub y: f32,
    /// Z-axis scaled value
    pub z: f32,
    /// Temperature scaled value
    pub t: f32,
}

/// Scaled 3D sensor data
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PmuCmdStatus0 {
    pub pmu_cmd_busy: u8,
    pub odr_overwrite: u8,
    pub avg_overwrite: u8,
    pub power_mode_is_normal: u8,
    pub cmd_is_illegal: u8,
    pub pmu_cmd_value: u8,
}
