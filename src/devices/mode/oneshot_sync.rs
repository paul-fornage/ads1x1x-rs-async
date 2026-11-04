use crate::{conversion, mode, Ads1x1x, BitFlags, ChannelId, Error, Register};

impl<I2C, IC, CONV, E> Ads1x1x<I2C, IC, CONV, mode::OneShot>
where
    I2C: crate::eh::i2c::I2c<Error = E>,
    CONV: conversion::ConvertMeasurement,
{
    /// Requests a conversion on the specified channel and returns its result when ready.
    ///
    /// Returns `nb::Error::WouldBlock` while a measurement is in progress or after
    /// starting a conversion. Requesting a different channel starts a new conversion
    /// once the previous conversion has finished.
    ///
    /// The output range is `[-2048..=2047]` for 12-bit devices and
    /// `[-32768..=32767]` for 16-bit devices. The corresponding voltage depends
    /// on the selected [`FullScaleRange`](crate::FullScaleRange).
    pub fn read<CH: ChannelId<Self>>(&mut self, _channel: CH) -> nb::Result<i16, Error<E>> {
        if self.is_measurement_in_progress().map_err(nb::Error::Other)? {
            return Err(nb::Error::WouldBlock);
        }
        let config = self.config.with_mux_bits(CH::channel_id());
        if self.a_conversion_was_started && self.config == config {
            let value = self
                .read_register(Register::CONVERSION)
                .map_err(nb::Error::Other)?;
            self.a_conversion_was_started = false;
            return Ok(CONV::convert_measurement(value));
        }
        self.write_register(Register::CONFIG, config.with_high(BitFlags::OS).bits)
            .map_err(nb::Error::Other)?;
        self.config = config;
        self.a_conversion_was_started = true;
        Err(nb::Error::WouldBlock)
    }
}
