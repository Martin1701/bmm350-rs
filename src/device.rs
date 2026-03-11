use crate::{
    interface::{I2cInterface, ReadData, WriteData},
    registers::*,
    types::{
        AxisEnableDisable, CrossAxis, DataRate, DutOffsetCoef, DutSensitCoef, DutTco, DutTcs,
        Error, MagCompensation, PerformanceMode, PmuCmdStatus0, PowerMode, Sensor3DData,
    },
    AverageNum, InterruptDrive, InterruptEnableDisable, InterruptLatch, InterruptMap,
    InterruptPolarity, MagConfig, Register, Sensor3DDataScaled,
};
use embedded_hal::delay::DelayNs;

/// Main struct representing the BMM350 device
pub struct Bmm350<DI, D> {
    /// Communication interface (I2C or SPI)
    iface: DI,
    /// Delay provider
    delay: D,
    /// Variant ID
    var_id: u8,
    /// Magnetometer compensation data
    mag_comp: MagCompensation,
}

impl<I2C, D> Bmm350<I2cInterface<I2C>, D>
where
    D: DelayNs,
{
    /// Create a new BMM350 device instance
    ///
    /// # Arguments
    ///
    /// * `i2c` - The I2C interface
    /// * `address` - The I2C address of the device
    /// * `delay` - A delay provider
    pub fn new_with_i2c(i2c: I2C, address: u8, delay: D) -> Self {
        Bmm350 {
            iface: I2cInterface { i2c, address },
            delay,
            var_id: 0,
            mag_comp: MagCompensation::default(), // Default range in uT
        }
    }
}

impl<DI, D, E> Bmm350<DI, D>
where
    DI: ReadData<Error = Error<E>> + WriteData<Error = Error<E>>,
    D: DelayNs,
{
    /// Initialize the device
    pub fn init(&mut self) -> Result<(), Error<E>> {
        self.delay.delay_us(3_000);
        self.write_register(Register::CMD, Register::CMD_SOFT_RESET)
            ?;
        self.delay.delay_us(24_000);

        let err = self.read_register(Register::ERR_REG)?;
        if err != 0 {
            return Err(Error::InvalidConfig);
        }

        let chip_id = self.read_register(Register::CHIPID)?;
        if chip_id != Register::BMM350_CHIP_ID {
            return Err(Error::InvalidDevice);
        }

        // Perform OTP dump after boot
        self.otp_dump_after_boot()?;

        // Power off OTP
        self.write_register(Register::OTP_CMD_REG, Register::OTP_CMD_PWR_OFF_OTP)
            ?;

        self.magnetic_reset()?;

        Ok(())
    }

    fn otp_dump_after_boot(&mut self) -> Result<(), Error<E>> {
        let mut otp_data = [0u16; 32];

        for i in 0..32 {
            otp_data[i] = self.read_otp_word(i as u8)?;
        }

        self.var_id = ((otp_data[30] & 0x7f00) >> 9) as u8;

        // Update magnetometer offset and sensitivity data
        self.update_mag_compensation(&otp_data);

        Ok(())
    }

    fn read_otp_word(&mut self, addr: u8) -> Result<u16, Error<E>> {
        let otp_cmd = BMM350_OTP_CMD_DIR_READ | (addr & BMM350_OTP_WORD_ADDR_MSK);
        self.write_register(Register::OTP_CMD_REG, otp_cmd)?;

        let mut done = false;
        for _ in 0..10 {
            self.delay.delay_us(300);
            let status = self.read_register(Register::OTP_STATUS_REG)?;
            if status & BMM350_OTP_STATUS_ERROR_MSK != 0 {
                return Err(Error::OtpError);
            }
            if status & BMM350_OTP_STATUS_CMD_DONE != 0 {
                done = true;
                break;
            }
        }
        if !done {
            return Err(Error::OtpTimeout);
        }

        let msb = self.read_register(Register::OTP_DATA_MSB_REG)?;
        let lsb = self.read_register(Register::OTP_DATA_LSB_REG)?;
        Ok(((msb as u16) << 8) | (lsb as u16))
    }

    fn update_mag_compensation(&mut self, otp: &[u16; 32]) {
        let off_x = fix_sign_12(otp[BMM350_MAG_OFFSET_X] & 0x0FFF);
        let off_y = fix_sign_12(
            ((otp[BMM350_MAG_OFFSET_X] & 0xF000) >> 4) | (otp[BMM350_MAG_OFFSET_Y] & 0x00FF),
        );
        let off_z =
            fix_sign_12((otp[BMM350_MAG_OFFSET_Y] & 0x0F00) | (otp[BMM350_MAG_OFFSET_Z] & 0x00FF));
        let t_off = (otp[BMM350_TEMP_OFF_SENS] & 0x00FF) as u8 as i8;
        let sens_x = ((otp[BMM350_MAG_SENS_X] & 0xFF00) >> 8) as u8 as i8;
        let sens_y = (otp[BMM350_MAG_SENS_Y] & 0x00FF) as u8 as i8;
        let sens_z = ((otp[BMM350_MAG_SENS_Z] & 0xFF00) >> 8) as u8 as i8;
        let t_sens = ((otp[BMM350_TEMP_OFF_SENS] & 0xFF00) >> 8) as u8 as i8;
        let tco_x = (otp[BMM350_MAG_TCO_X] & 0x00FF) as u8 as i8;
        let tco_y = (otp[BMM350_MAG_TCO_Y] & 0x00FF) as u8 as i8;
        let tco_z = (otp[BMM350_MAG_TCO_Z] & 0x00FF) as u8 as i8;
        let tcs_x = ((otp[BMM350_MAG_TCS_X] & 0xFF00) >> 8) as u8 as i8;
        let tcs_y = ((otp[BMM350_MAG_TCS_Y] & 0xFF00) >> 8) as u8 as i8;
        let tcs_z = ((otp[BMM350_MAG_TCS_Z] & 0xFF00) >> 8) as u8 as i8;
        let dut_t0 = otp[BMM350_MAG_DUT_T_0] as i16 as f32 / 512.0 + 23.0;
        let cr_x_y = (otp[BMM350_CROSS_X_Y] & 0x00FF) as u8 as i8;
        let cr_y_x = ((otp[BMM350_CROSS_Y_X] & 0xFF00) >> 8) as u8 as i8;
        let cr_z_x = (otp[BMM350_CROSS_Z_X] & 0x00FF) as u8 as i8;
        let cr_z_y = ((otp[BMM350_CROSS_Z_Y] & 0xFF00) >> 8) as u8 as i8;

        self.mag_comp = MagCompensation {
            dut_offset_coef: DutOffsetCoef {
                offset_x: off_x as f32,
                offset_y: off_y as f32,
                offset_z: off_z as f32,
                t_offs: t_off as f32 / 5.0,
            },
            dut_sensit_coef: DutSensitCoef {
                sens_x: sens_x as f32 / 256.0,
                sens_y: sens_y as f32 / 256.0,
                sens_z: sens_z as f32 / 256.0,
                t_sens: t_sens as f32 / 512.0,
            },
            dut_tco: DutTco {
                tco_x: tco_x as f32 / 32.0,
                tco_y: tco_y as f32 / 32.0,
                tco_z: tco_z as f32 / 32.0,
            },
            dut_tcs: DutTcs {
                tcs_x: tcs_x as f32 / 16384.0,
                tcs_y: tcs_y as f32 / 16384.0,
                tcs_z: tcs_z as f32 / 16384.0,
            },
            dut_t0,
            cross_axis: CrossAxis {
                cross_x_y: cr_x_y as f32 / 800.0,
                cross_y_x: cr_y_x as f32 / 800.0,
                cross_z_x: cr_z_x as f32 / 800.0,
                cross_z_y: cr_z_y as f32 / 800.0,
            },
        };

        self.mag_comp.dut_sensit_coef.sens_y += BMM350_SENS_CORR_Y;
        self.mag_comp.dut_tcs.tcs_z += BMM350_TCS_CORR_Z;
    }

    pub fn get_comp(&self) -> &MagCompensation {
        return &self.mag_comp;
    }

    /// Perform magnetic reset of the sensor.
    /// This is necessary after a field shock (400mT field applied to sensor).
    /// It performs both a bit reset and flux guide reset in suspend mode.
    pub fn magnetic_reset(&mut self) -> Result<(), Error<E>> {
        // Check if we're in normal mode
        let mut restore_normal = false;
        let mut pmu_status = self.read_pmu_cmd_status_0()?;

        // If we're in normal mode, we need to go to suspend first
        if pmu_status.power_mode_is_normal == 0x1 {
            restore_normal = true;
            self.set_power_mode(PowerMode::Suspend)?;
        }

        // Set Bit Reset (BR) command
        // TODO set BitReset as register instead of PowerMode enum
        self.write_register(Register::PMU_CMD, PowerMode::BitReset as u8)
            ?;
        self.delay.delay_us(14_000); // BR_DELAY

        // Verify BR status
        pmu_status = self.read_pmu_cmd_status_0()?;
        if pmu_status.pmu_cmd_value != Register::PMU_CMD_STATUS_0_BR {
            return Err(Error::ResetUnfinished);
        }

        // Set Flux Guide Reset (FGR) command
        // TODO set FluxGuideReset as register instead of PowerMode enum
        self.write_register(Register::PMU_CMD, PowerMode::FluxGuideReset as u8)
            ?;
        self.delay.delay_us(18_000); // FGR_DELAY

        // Verify FGR status
        let pmu_status = self.read_pmu_cmd_status_0()?;
        if pmu_status.pmu_cmd_value != Register::PMU_CMD_STATUS_0_FGR {
            return Err(Error::ResetUnfinished);
        }

        // Restore normal mode if we were in it before
        if restore_normal {
            self.set_power_mode(PowerMode::Normal)?;
        }

        Ok(())
    }

    /// Read the PMU command status register 0
    fn read_pmu_cmd_status_0(&mut self) -> Result<PmuCmdStatus0, Error<E>> {
        let status = self.read_register(Register::PMU_CMD_STATUS_0)?;

        Ok(PmuCmdStatus0 {
            pmu_cmd_busy: (status & 0x01),
            odr_overwrite: (status & 0x2) >> 0x1,
            avg_overwrite: (status & 0x4) >> 0x2,
            power_mode_is_normal: (status & 0x8) >> 0x3,
            cmd_is_illegal: (status & 0x10) >> 0x4,
            pmu_cmd_value: (status & 0xE0) >> 5,
        })
    }

    /// Set the magnetometer configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The magnetometer configuration
    pub fn set_mag_config(&mut self, config: MagConfig) -> Result<(), Error<E>> {
        self.write_register(Register::PMU_CMD_AGGR_SET, u8::from(config))
            ?;
        self.write_register(Register::PMU_CMD, Register::PMU_CMD_UPD_OAE)
            ?;
        self.delay.delay_us(BMM350_UPD_OAE_DELAY);
        Ok(())
    }

    /// Set the power mode of the sensor
    ///
    /// # Arguments
    ///
    /// * `mode` - The power mode to set
    pub fn set_power_mode(&mut self, mode: PowerMode) -> Result<(), Error<E>> {
        // TODO fix

        let last_pwr = self.read_register(Register::REG_PMU_CMD)?;
        if last_pwr > PowerMode::BrFast as u8 {
            return Err(Error::InvalidConfig);
        }

        if last_pwr == Register::PMU_CMD_NM || last_pwr == Register::PMU_CMD_UPD_OAE {
            self.write_register(Register::REG_PMU_CMD, Register::PMU_CMD_SUS)
                ?;
            self.delay.delay_us(6_000);
        }

        self.power_mode(mode)?;

        Ok(())
    }

    fn power_mode(&mut self, mode: PowerMode) -> Result<(), Error<E>> {
        let sus_to_forced_mode: [u32; 4] = [
            Register::SUS_TO_FORCEDMODE_NO_AVG_DELAY,
            Register::SUS_TO_FORCEDMODE_AVG_2_DELAY,
            Register::SUS_TO_FORCEDMODE_AVG_4_DELAY,
            Register::SUS_TO_FORCEDMODE_AVG_8_DELAY,
        ];

        /* Array to store suspend to forced mode fast delay */
        let sus_to_forced_mode_fast: [u32; 4] = [
            Register::SUS_TO_FORCEDMODE_FAST_NO_AVG_DELAY,
            Register::SUS_TO_FORCEDMODE_FAST_AVG_2_DELAY,
            Register::SUS_TO_FORCEDMODE_FAST_AVG_4_DELAY,
            Register::SUS_TO_FORCEDMODE_FAST_AVG_8_DELAY,
        ];

        self.write_register(Register::REG_PMU_CMD, mode as u8)
            ?;
        let get_avg: u8 = self.read_register(Register::REG_PMU_CMD_AGGR_SET)?;
        let avg = (get_avg & Register::AVG_MASK) >> Register::AVG_POS;
        let mut delay_us = 0;
        match mode {
            PowerMode::Normal => {
                delay_us = 38_000;
            }
            PowerMode::Forced => {
                delay_us = sus_to_forced_mode[avg as usize];
            }
            PowerMode::ForcedFast => {
                delay_us = sus_to_forced_mode_fast[avg as usize];
            }
            _ => {}
        }

        self.delay.delay_us(delay_us);

        Ok(())
    }

    /// Enable or disable axes
    ///
    /// # Arguments
    ///
    /// * `x` - Enable or disable X axis
    /// * `y` - Enable or disable Y axis
    /// * `z` - Enable or disable Z axis
    pub fn enable_axes(
        &mut self,
        x: AxisEnableDisable,
        y: AxisEnableDisable,
        z: AxisEnableDisable,
    ) -> Result<(), Error<E>> {
        let reg_data: u8 = (x as u8 & 0x01) | ((y as u8 & 0x01) << 1) | ((z as u8 & 0x01) << 2);
        self.write_register(Register::PMU_CMD_AXIS_EN, reg_data)
            
    }

    pub fn read_mag_data_and_compensate(&mut self) -> Result<Sensor3DDataScaled, Error<E>> {
        Ok(self.read_mag_data()?.to_ut(&self.get_comp()))
    }

    /// Read the raw magnetometer data
    pub fn read_mag_data(&mut self) -> Result<Sensor3DData, Error<E>> {
        // Prepare a buffer: 1 byte for start address + 9 bytes for data (X, Y, Z) + 3 bytes for temperature
        const DATA_LEN: usize = 12;
        const BUFFER_LEN: usize = 1 + DATA_LEN;
        let mut buffer = [0u8; BUFFER_LEN]; // Size 13
        buffer[0] = Register::MAG_X_LSB; // Start address 0x31

        // read_data will return a slice referencing buffer[1..10] containing the 12 data bytes
        let sensor_data_slice = self.read_data(&mut buffer[0..BUFFER_LEN])?;

        // Helper function for 24-bit signed reconstruction (still needed!)
        fn reconstruct_signed_24bit(xlsb: u8, lsb: u8, msb: u8) -> i32 {
            let unsigned_val = (xlsb as u32) | ((lsb as u32) << 8) | ((msb as u32) << 16);
            if (msb & 0x80) != 0 {
                (unsigned_val | 0xFF000000) as i32 // Manual sign extension
            } else {
                unsigned_val as i32
            }
        }

        // Use indices relative to the returned slice
        Ok(Sensor3DData {
            x: reconstruct_signed_24bit(
                sensor_data_slice[0],
                sensor_data_slice[1],
                sensor_data_slice[2],
            ),
            y: reconstruct_signed_24bit(
                sensor_data_slice[3],
                sensor_data_slice[4],
                sensor_data_slice[5],
            ),
            z: reconstruct_signed_24bit(
                sensor_data_slice[6],
                sensor_data_slice[7],
                sensor_data_slice[8],
            ),
            t: reconstruct_signed_24bit(
                sensor_data_slice[9],
                sensor_data_slice[10],
                sensor_data_slice[11],
            ),
        })
    }

    /// Perform a self-test
    // TODO fix this
    pub fn perform_self_test(&mut self) -> Result<bool, Error<E>> {
        // Save current configuration
        let current_power_mode = self.read_register(Register::PMU_CMD)?;
        let current_odr = self.read_register(Register::PMU_CMD_AGGR_SET)?;

        // Set device to normal mode and 100Hz ODR
        self.set_power_mode(PowerMode::Normal)?;
        self.set_mag_config(
            MagConfig::builder()
                .odr(DataRate::ODR100Hz)
                .performance(PerformanceMode::Regular)
                .build(),
        )
        ?;

        // Perform self-test
        let self_test_passed = true;

        // Restore original configuration
        self.write_register(Register::PMU_CMD, current_power_mode)
            ?;
        self.write_register(Register::PMU_CMD_AGGR_SET, current_odr)
            ?;

        Ok(self_test_passed)
    }

    /// Set the output data rate and performance mode
    pub fn set_odr_performance(
        &mut self,
        odr: DataRate,
        performance: AverageNum,
    ) -> Result<(), Error<E>> {
        // Validate ODR/averaging combination
        match odr {
            DataRate::ODR400Hz if performance as u8 >= AverageNum::Avg2 as u8 => {
                return Err(Error::InvalidConfig)
            }
            DataRate::ODR200Hz if performance as u8 >= AverageNum::Avg4 as u8 => {
                return Err(Error::InvalidConfig)
            }
            DataRate::ODR100Hz if performance as u8 >= AverageNum::Avg8 as u8 => {
                return Err(Error::InvalidConfig)
            }
            _ => {}
        }

        let new_reg_data = (odr as u8 & 0x0F) | ((performance as u8 & 0x03) << 4);

        self.write_register(Register::PMU_CMD_AGGR_SET, new_reg_data)
            ?;
        self.write_register(Register::PMU_CMD, Register::PMU_CMD_UPD_OAE)
            ?;

        self.delay.delay_us(BMM350_UPD_OAE_DELAY);
        Ok(())
    }

    /// Enable or disable the data ready interrupt
    pub fn enable_interrupt(
        &mut self,
        enable: InterruptEnableDisable,
    ) -> Result<(), Error<E>> {
        let reg_data = self.read_register(Register::INT_CTRL)?;
        let new_reg_data = (reg_data & !0x80) | ((enable as u8) << 7);
        self.write_register(Register::INT_CTRL, new_reg_data)
    }

    pub fn configure_interrupt(
        &mut self,
        latch: InterruptLatch,
        polarity: InterruptPolarity,
        drive: InterruptDrive,
        map: InterruptMap,
    ) -> Result<(), Error<E>> {
        let reg_data = self.read_register(Register::INT_CTRL)?;
        let new_reg_data = (reg_data & !0x0F)
            | (latch as u8 & 0x01)
            | ((polarity as u8 & 0x01) << 1)
            | ((drive as u8 & 0x01) << 2)
            | ((map as u8 & 0x01) << 3);
        self.write_register(Register::INT_CTRL, new_reg_data)
    }

    /// Read the interrupt status
    pub fn get_interrupt_status(&mut self) -> Result<bool, Error<E>> {
        let status = self.read_register(Register::INT_STATUS)?;
        Ok((status & 0x04) != 0)
    }

    /// Set the I2C watchdog timer
    pub fn set_i2c_watchdog(
        &mut self,
        enable: bool,
        long_timeout: bool,
    ) -> Result<(), Error<E>> {
        let reg_data = (enable as u8) | ((long_timeout as u8) << 1);
        self.write_register(Register::I2C_WDT_SET, reg_data)
    }

    fn write_register(&mut self, reg: u8, value: u8) -> Result<(), Error<E>> {
        self.iface.write_data(&[reg, value])
    }

    fn read_register(&mut self, reg: u8) -> Result<u8, Error<E>> {
        self.iface.read_register(reg)
    }

    fn read_data<'a>(&mut self, data: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        self.iface.read_data(data)
    }
}

#[inline]
fn fix_sign_12(raw: u16) -> i16 {
    let v = raw & 0x0FFF;
    if v & 0x0800 != 0 {
        (v | 0xF000) as i16
    } else {
        v as i16
    }
}
