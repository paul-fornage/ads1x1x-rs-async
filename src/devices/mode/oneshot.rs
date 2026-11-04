//! One-shot measurement mode.

use core::marker::PhantomData;

#[cfg(feature = "async")]
use crate::{eh::delay::DelayNs, BitFlags, ChannelId, Register};

use crate::{conversion, devices::OperatingMode, mode, Ads1x1x, Error, ModeChangeError};

#[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
impl<I2C, IC, CONV, E> Ads1x1x<I2C, IC, CONV, mode::OneShot>
where
    I2C: crate::eh::i2c::I2c<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Changes to continuous operating mode.
    pub async fn into_continuous(
        mut self,
    ) -> Result<Ads1x1x<I2C, IC, CONV, mode::Continuous>, ModeChangeError<E, Self>> {
        if let Err(Error::I2C(e)) = self.set_operating_mode(OperatingMode::Continuous).await {
            return Err(ModeChangeError::I2C(e, self));
        }
        Ok(Ads1x1x {
            i2c: self.i2c,
            address: self.address,
            config: self.config,
            fsr: self.fsr,
            #[cfg(feature = "sync")]
            a_conversion_was_started: true,
            _conv: PhantomData,
            _ic: PhantomData,
            _mode: PhantomData,
        })
    }

    /// Performs a new conversion on the specified channel and returns its result.
    ///
    /// The output value will be within `[2047..-2048]` for 12-bit devices
    /// (`ADS101x`) and within `[32767..-32768]` for 16-bit devices (`ADS111x`).
    /// The voltage that these values correspond to must be calculated using
    /// the full-scale range ([`FullScaleRange`](crate::FullScaleRange)) selected.
    ///
    /// Waits for any previous conversion to finish before starting a new one.
    /// Polls conversion status with an async delay of at least 100 microseconds
    /// between checks while the ADC is busy.
    #[cfg(feature = "async")]
    pub async fn read<CH: ChannelId<Self>, D: DelayNs + ?Sized>(
        &mut self,
        _channel: CH,
        delay: &mut D,
    ) -> Result<i16, Error<E>> {
        self.wait_until_ready(delay).await?;
        let config = self.config.with_mux_bits(CH::channel_id());
        self.write_register(Register::Config, config.with_high(BitFlags::Os).bits)
            .await?;
        self.config = config;
        self.wait_until_ready(delay).await?;
        let value = self.read_register(Register::Conversion).await?;
        Ok(CONV::convert_measurement(value))
    }

    #[cfg(feature = "async")]
    async fn wait_until_ready<D: DelayNs + ?Sized>(&mut self, delay: &mut D) -> Result<(), Error<E>> {
        while self.is_measurement_in_progress().await? {
            delay.delay_us(100).await;
        }
        Ok(())
    }
}
