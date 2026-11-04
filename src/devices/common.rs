//! Common functions.

use crate::{devices::OperatingMode, Ads1x1x, BitFlags, Config, Error, Register};

#[cfg_attr(not(feature = "async"), maybe_async::must_be_sync)]
impl<I2C, IC, CONV, MODE, E> Ads1x1x<I2C, IC, CONV, MODE>
where
    I2C: crate::eh::i2c::I2c<Error = E>,
{
    pub(super) async fn write_register(&mut self, register: u8, data: u16) -> Result<(), Error<E>> {
        let data = data.to_be_bytes();
        let payload: [u8; 3] = [register, data[0], data[1]];
        self.i2c.write(self.address, &payload).await.map_err(Error::I2C)
    }

    pub(super) async fn read_register(&mut self, register: u8) -> Result<u16, Error<E>> {
        let mut data = [0, 0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .await
            .map_err(Error::I2C)
            .and(Ok(u16::from_be_bytes(data)))
    }

    pub(super) async fn set_operating_mode(&mut self, mode: OperatingMode) -> Result<(), Error<E>> {
        let config = match mode {
            OperatingMode::OneShot => self.config.with_high(BitFlags::OP_MODE),
            OperatingMode::Continuous => self.config.with_low(BitFlags::OP_MODE),
        };
        self.write_register(Register::CONFIG, config.bits).await?;
        self.config = config;
        Ok(())
    }

    /// Checks whether a measurement is currently in progress.
    pub async fn is_measurement_in_progress(&mut self) -> Result<bool, Error<E>> {
        let config = Config {
            bits: self.read_register(Register::CONFIG).await?,
        };
        Ok(!config.is_high(BitFlags::OS))
    }
}
