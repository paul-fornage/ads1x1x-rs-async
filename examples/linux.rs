use linux_embedded_hal::I2cdev;

use ads1x1x::{channel, Ads1x1x, TargetAddr};

mod common;
use common::{BlockingDelay, BlockingI2c};

#[pollster::main]
async fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1013(BlockingI2c(dev), TargetAddr::default());
    let value = adc
        .read(channel::DifferentialA0A1, &mut BlockingDelay)
        .await
        .unwrap();
    println!("Measurement: {value}");
    // get I2C device back
    let _dev = adc.destroy_ads1013().0;
}
