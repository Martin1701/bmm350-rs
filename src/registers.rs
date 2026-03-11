/// BMM350 register addresses and constant values
pub struct Register;
impl Register {
    /// Chip ID register address
    pub const CHIPID: u8 = 0x00;
    /// Error register address
    pub const ERR_REG: u8 = 0x02;
    /// Status register address
    pub const STATUS: u8 = 0x03;
    /// Magnetometer X-axis data register address
    pub const MAG_X_LSB: u8 = 0x31;
    /// Magnetometer configuration register address
    pub const PMU_CMD_AGGR_SET: u8 = 0x04;
    /// Axis enable register address
    pub const PMU_CMD_AXIS_EN: u8 = 0x05;
    /// Power mode command register address
    pub const PMU_CMD: u8 = 0x06;
    /// Command register address
    pub const CMD: u8 = 0x7E;
    /// Expected chip ID for BMM350
    pub const BMM350_CHIP_ID: u8 = 0x33;
    /// Soft reset command value
    pub const CMD_SOFT_RESET: u8 = 0xB6;

    pub const TMR_SELFTEST_USER: u8 = 0x60;

    pub const INT_CTRL: u8 = 0x2E;

    pub const INT_STATUS: u8 = 0x30;

    pub const I2C_WDT_SET: u8 = 0x0A;

    pub const OTP_CMD_REG: u8 = 0x50;

    pub const OTP_CMD_PWR_OFF_OTP: u8 = 0x80;

    pub const OTP_STATUS_REG: u8 = 0x55;

    pub const OTP_DATA_MSB_REG: u8 = 0x52;

    pub const OTP_DATA_LSB_REG: u8 = 0x53;

    pub const PMU_CMD_STATUS_0: u8 = 0x07;

    pub const PMU_CMD_STATUS_0_BR: u8 = 0x07;

    pub const PMU_CMD_STATUS_0_FGR: u8 = 0x05;

    pub const PMU_CMD_UPD_OAE: u8 = 0x02;

    pub const AVG_MASK: u8 = 0x30;

    pub const AVG_POS: u8 = 0x4;

    pub const REG_PMU_CMD: u8 = 0x06;

    pub const PMU_CMD_NM_TC: u8 = 0x09;

    pub const PMU_CMD_NM: u8 = 0x01;

    pub const PMU_CMD_SUS: u8 = 0x00;

    pub const REG_PMU_CMD_AGGR_SET: u8 = 0x04;

    pub const SUSPEND_TO_NORMAL_DELAY: u32 = 38_000;

    pub const SUS_TO_FORCEDMODE_NO_AVG_DELAY: u32 = 15000;
    pub const SUS_TO_FORCEDMODE_AVG_2_DELAY: u32 = 17000;
    pub const SUS_TO_FORCEDMODE_AVG_4_DELAY: u32 = 20000;
    pub const SUS_TO_FORCEDMODE_AVG_8_DELAY: u32 = 28000;

    pub const SUS_TO_FORCEDMODE_FAST_NO_AVG_DELAY: u32 = 4000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_2_DELAY: u32 = 5000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_4_DELAY: u32 = 9000;
    pub const SUS_TO_FORCEDMODE_FAST_AVG_8_DELAY: u32 = 16000;
}

// OTP indices
pub const BMM350_TEMP_OFF_SENS: usize = 0x0D;
pub const BMM350_MAG_OFFSET_X: usize = 0x0E;
pub const BMM350_MAG_OFFSET_Y: usize = 0x0F;
pub const BMM350_MAG_OFFSET_Z: usize = 0x10;
pub const BMM350_MAG_SENS_X: usize = 0x10;
pub const BMM350_MAG_SENS_Y: usize = 0x11;
pub const BMM350_MAG_SENS_Z: usize = 0x11;
pub const BMM350_MAG_TCO_X: usize = 0x12;
pub const BMM350_MAG_TCO_Y: usize = 0x13;
pub const BMM350_MAG_TCO_Z: usize = 0x14;
pub const BMM350_MAG_TCS_X: usize = 0x12;
pub const BMM350_MAG_TCS_Y: usize = 0x13;
pub const BMM350_MAG_TCS_Z: usize = 0x14;
pub const BMM350_MAG_DUT_T_0: usize = 0x18;
pub const BMM350_CROSS_X_Y: usize = 0x15;
pub const BMM350_CROSS_Y_X: usize = 0x15;
pub const BMM350_CROSS_Z_X: usize = 0x16;
pub const BMM350_CROSS_Z_Y: usize = 0x16;

// Post-solder correction (BMM350_POST_SOLDER_CORR enabled by default)
pub const BMM350_SENS_CORR_Y: f32 = 0.01;
pub const BMM350_TCS_CORR_Z: f32 = 0.0001;

pub const BMM350_OTP_CMD_DIR_READ: u8 = 0x20;
pub const BMM350_OTP_WORD_ADDR_MSK: u8 = 0x1F;
pub const BMM350_OTP_STATUS_ERROR_MSK: u8 = 0xE0;
pub const BMM350_OTP_STATUS_CMD_DONE: u8 = 0x01;

pub const BMM350_UPD_OAE_DELAY: u32 = 1_000;

// Compensation scale factors
pub const BMM350_LSB_TO_UT_XY: f32 = {
    const POWER: f32 = 1_000_000.0 / 1_048_576.0;
    const ADC_GAIN: f32 = 1.0 / 1.5;
    const LUT_GAIN: f32 = 0.714607238769531;
    POWER / (14.55 * 19.46 * ADC_GAIN * LUT_GAIN)
};
pub const BMM350_LSB_TO_UT_Z: f32 = {
    const POWER: f32 = 1_000_000.0 / 1_048_576.0;
    const ADC_GAIN: f32 = 1.0 / 1.5;
    const LUT_GAIN: f32 = 0.714607238769531;
    POWER / (9.0 * 31.0 * ADC_GAIN * LUT_GAIN)
};
pub const BMM350_LSB_TO_DEGC: f32 = {
    const ADC_GAIN: f32 = 1.0 / 1.5;
    const LUT_GAIN: f32 = 0.714607238769531;
    1.0 / (0.00204 * ADC_GAIN * LUT_GAIN * 1_048_576.0)
};
pub const BMM350_TEMP_OFFSET: f32 = 25.49;
