use embedded_hal::i2c::{ErrorType, I2c, Operation};
use linux_embedded_hal::I2cdev;

pub struct BlockingI2c(pub I2cdev);

impl ErrorType for BlockingI2c {
    type Error = <I2cdev as ErrorType>::Error;
}

impl embedded_hal_async::i2c::I2c for BlockingI2c {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.0.transaction(address, operations)
    }
}

pub struct BlockingDelay;

impl embedded_hal_async::delay::DelayNs for BlockingDelay {
    async fn delay_ns(&mut self, ns: u32) {
        std::thread::sleep(std::time::Duration::from_nanos(u64::from(ns)));
    }
}
