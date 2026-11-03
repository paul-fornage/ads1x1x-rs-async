// This example demonstrates the use of a type alias for the `Ads1x1x` struct
// to ease usage in signatures.

use linux_embedded_hal::I2cdev;

use ads1x1x::{
    channel,
    ic::{Ads1115, Resolution16Bit},
    Ads1x1x, TargetAddr,
};

mod common;
use common::{BlockingDelay, BlockingI2c};

/// Type alias
type Adc = Ads1x1x<BlockingI2c, Ads1115, Resolution16Bit, ads1x1x::mode::OneShot>;

/// Read a single value from channel A.
pub async fn read(adc: &mut Adc) -> i16 {
    adc.read(channel::SingleA0, &mut BlockingDelay).await.unwrap()
}

#[pollster::main]
async fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1115(BlockingI2c(dev), TargetAddr::default());

    let value = read(&mut adc).await;
    println!("Measurement: {value}");
    // get I2C device back
    let _dev = adc.destroy_ads1115().0;
}
