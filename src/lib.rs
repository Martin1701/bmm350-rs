#![no_std]

/// BMM350 driver for Rust
///
/// This module provides a high-level interface for interacting with the Bosch BMM350 magnetometer.
/// It supports both I2C interfaces and allows for configuration of magnetometer settings.

#[cfg(not(feature = "async"))]
pub mod device;
#[cfg(not(feature = "async"))]
pub mod interface;

#[cfg(feature = "async")]
pub mod device_async;
#[cfg(feature = "async")]
pub mod interface_async;
mod registers;
pub use registers::Register;
mod types;
pub use types::{
    AverageNum, AxisEnableDisable, Bandwidth, CtrlUser, DataRate, Error, I2cWdtEnable,
    I2cWdtSelect, InterruptDrive, InterruptEnableDisable, InterruptLatch, InterruptMap,
    InterruptPolarity, MagCompensation, PerformanceMode, PowerMode, SelfTestMode, Sensor3DData,
    Sensor3DDataScaled,
};
mod sensor_data;
pub use sensor_data::*;
#[derive(Debug, Clone, Copy)]
pub struct MagConfig {
    pub odr: DataRate,
    pub performance: PerformanceMode,
}

impl MagConfig {
    pub fn builder() -> MagConfigBuilder {
        MagConfigBuilder::default()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MagConfigBuilder {
    odr: Option<DataRate>,
    performance: Option<PerformanceMode>,
}

impl MagConfigBuilder {
    pub fn odr(mut self, odr: DataRate) -> Self {
        self.odr = Some(odr);
        self
    }
    pub fn performance(mut self, performance: PerformanceMode) -> Self {
        self.performance = Some(performance);
        self
    }
    pub fn build(self) -> MagConfig {
        MagConfig {
            odr: self.odr.unwrap_or(DataRate::ODR100Hz),
            performance: self.performance.unwrap_or(PerformanceMode::Regular),
        }
    }
}

impl From<MagConfig> for u8 {
    fn from(config: MagConfig) -> Self {
        (config.odr as u8 & 0x0F) | ((config.performance as u8 & 0x03) << 4)
    }
}
