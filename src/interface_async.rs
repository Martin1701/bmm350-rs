use crate::Error;
use embedded_hal_async::i2c;

/// I2C communication interface for BMM350
#[derive(Debug)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
    pub(crate) address: u8,
}

/// Trait for writing data to the BMM350
#[allow(async_fn_in_trait)]
pub trait WriteData {
    type Error;
    /// Write a single byte to a register
    ///
    /// # Arguments
    ///
    /// * `register` - The register address
    /// * `data` - The byte to write
    async fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error>;
    /// Write multiple bytes of data
    ///
    /// # Arguments
    ///
    /// * `payload` - The data to write
    async fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error>;
}

impl<I2C, E> WriteData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    async fn write_register(&mut self, register: u8, data: u8) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [register, data];
        self.i2c
            .write(self.address, &payload)
            .await
            .map_err(Error::Comm)
    }

    async fn write_data(&mut self, payload: &[u8]) -> Result<(), Self::Error> {
        self.i2c
            .write(self.address, payload)
            .await
            .map_err(Error::Comm)
    }
}

#[allow(async_fn_in_trait)]
pub trait ReadData {
    type Error;
    /// Read a single byte from a register
    ///
    /// # Arguments
    ///
    /// * `register` - The register address to read from
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error>;
    /// Read multiple bytes of data
    ///
    /// # Arguments
    ///
    /// * `payload` - Buffer to store the read data
    async fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Self::Error>;
}

impl<I2C, E> ReadData for I2cInterface<I2C>
where
    I2C: i2c::I2c<Error = E>,
{
    type Error = Error<E>;
    async fn read_register(&mut self, register: u8) -> Result<u8, Self::Error> {
        let mut buf = [0u8; 3];
        self.i2c
            .write_read(self.address, &[register], &mut buf)
            .await
            .map_err(Error::Comm)?;
        Ok(buf[2]) // skip 2 dummy bytes
    }

    async fn read_data<'a>(&mut self, payload: &'a mut [u8]) -> Result<&'a [u8], Error<E>> {
        let address = payload[0];
        let len = payload.len();
        let data = &mut payload[1..len];

        let total_len = data.len() + 2;
        let mut temp_buf = [0u8; 34]; // most read is 32 bytes

        self.i2c
            .write_read(self.address, &[address], &mut temp_buf[..total_len])
            .await
            .map_err(Error::Comm)?;

        // Copy data from temp_buf to data, skipping dummy bytes
        data.copy_from_slice(&temp_buf[2..total_len]);

        Ok(data)
    }
}
