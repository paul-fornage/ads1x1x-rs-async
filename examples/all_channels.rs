use linux_embedded_hal::I2cdev;

use ads1x1x::{channel, Ads1x1x, TargetAddr};

mod common;
use common::{BlockingDelay, BlockingI2c};

#[pollster::main]
async fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let mut adc = Ads1x1x::new_ads1015(BlockingI2c(dev), TargetAddr::default());
    let mut delay = BlockingDelay;
    let values = [
        adc.read(channel::SingleA0, &mut delay).await.unwrap(),
        adc.read(channel::SingleA1, &mut delay).await.unwrap(),
        adc.read(channel::SingleA2, &mut delay).await.unwrap(),
        adc.read(channel::SingleA3, &mut delay).await.unwrap(),
    ];
    for (channel, value) in values.iter().enumerate() {
        println!("Channel {channel}: {value}");
    }
    // get I2C device back
    let _dev = adc.destroy_ads1015().0;
}
