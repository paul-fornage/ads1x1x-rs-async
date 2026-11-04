use ads1x1x::{channel, DataRate12Bit, DataRate16Bit};
#[cfg(feature = "async")]
use embedded_hal_mock::eh1::delay::NoopDelay;
use embedded_hal_mock::eh1::i2c::Transaction as I2cTrans;

mod common;
use crate::common::{
    destroy_ads1013, destroy_ads1113, new_ads1013, new_ads1113, BitFlags as BF, Config, Register,
    DEVICE_ADDRESS as DEV_ADDR,
};

macro_rules! measure_tests {
    ($IC:ident, $create:ident, $destroy:ident, $expected:expr) => {
        mod $IC {
            use super::*;

            #[cfg(feature = "sync")]
            #[test]
            fn read_if_measurement_in_progress() {
                let config = Config::default().with_low(BF::OS);
                let transactions = [I2cTrans::write_read(
                    DEV_ADDR,
                    vec![Register::CONFIG],
                    vec![config.msb(), config.lsb()],
                )];
                let mut dev = $create(&transactions);
                assert!(matches!(
                    dev.read(channel::DifferentialA0A1),
                    Err(nb::Error::WouldBlock)
                ));
                $destroy(dev);
            }

            #[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
            #[cfg_attr(feature = "async", pollster::test)]
            #[cfg_attr(feature = "sync", test)]
            async fn can_measure() {
                let default_config = Config::default();
                let config_with_os = Config::default().with_high(BF::OS);
                let transactions = [
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![default_config.msb(), default_config.lsb()],
                    ),
                    I2cTrans::write(
                        DEV_ADDR,
                        vec![Register::CONFIG, config_with_os.msb(), config_with_os.lsb()],
                    ),
                    I2cTrans::write_read(
                        DEV_ADDR,
                        vec![Register::CONFIG],
                        vec![config_with_os.msb(), config_with_os.lsb()],
                    ),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let mut dev = $create(&transactions);
                #[cfg(feature = "async")]
                let measurement = dev
                    .read(channel::DifferentialA0A1, &mut NoopDelay::new())
                    .await
                    .unwrap();
                #[cfg(feature = "sync")]
                let measurement = nb::block!(dev.read(channel::DifferentialA0A1)).unwrap();
                assert_eq!($expected, measurement);
                $destroy(dev);
            }

            #[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
            #[cfg_attr(feature = "async", pollster::test)]
            #[cfg_attr(feature = "sync", test)]
            async fn can_measure_continuous() {
                let config = Config::default().with_low(BF::OP_MODE);
                let transactions = [
                    I2cTrans::write(DEV_ADDR, vec![Register::CONFIG, config.msb(), config.lsb()]),
                    I2cTrans::write_read(DEV_ADDR, vec![Register::CONVERSION], vec![0x80, 0x00]),
                ];
                let dev = $create(&transactions);
                let mut dev = dev.into_continuous().await.ok().unwrap();
                let measurement = dev.read().await.unwrap();
                assert_eq!($expected, measurement);
                $destroy(dev);
            }
        }
    };
}

measure_tests!(ads1013, new_ads1013, destroy_ads1013, -2048);
measure_tests!(ads1113, new_ads1113, destroy_ads1113, -32768);

mod data_rate_12bit {
    use super::*;

    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
            #[cfg_attr(feature = "async", pollster::test)]
            #[cfg_attr(feature = "sync", test)]
            async fn $name() {
                let transactions = [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::CONFIG, $config.msb(), $config.lsb()],
                )];
                let mut dev = new_ads1013(&transactions);
                dev.set_data_rate(DataRate12Bit::$variant).await.unwrap();
                destroy_ads1013(dev);
            }
        };
    }

    test!(
        sps128,
        Sps128,
        Config::default()
            .with_low(BF::DR2)
            .with_low(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps250,
        Sps250,
        Config::default()
            .with_low(BF::DR2)
            .with_low(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps490,
        Sps490,
        Config::default()
            .with_low(BF::DR2)
            .with_high(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps920,
        Sps920,
        Config::default()
            .with_low(BF::DR2)
            .with_high(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps1600,
        Sps1600,
        Config::default()
            .with_high(BF::DR2)
            .with_low(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps2400,
        Sps2400,
        Config::default()
            .with_high(BF::DR2)
            .with_low(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps3300,
        Sps3300,
        Config::default()
            .with_high(BF::DR2)
            .with_high(BF::DR1)
            .with_low(BF::DR0)
    );
}

mod data_rate_16bit {
    use super::*;

    macro_rules! test {
        ($name:ident, $variant:ident, $config:expr) => {
            #[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
            #[cfg_attr(feature = "async", pollster::test)]
            #[cfg_attr(feature = "sync", test)]
            async fn $name() {
                let transactions = [I2cTrans::write(
                    DEV_ADDR,
                    vec![Register::CONFIG, $config.msb(), $config.lsb()],
                )];
                let mut dev = new_ads1113(&transactions);
                dev.set_data_rate(DataRate16Bit::$variant).await.unwrap();
                destroy_ads1113(dev);
            }
        };
    }

    test!(
        sps8,
        Sps8,
        Config::default()
            .with_low(BF::DR2)
            .with_low(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps16,
        Sps16,
        Config::default()
            .with_low(BF::DR2)
            .with_low(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps32,
        Sps32,
        Config::default()
            .with_low(BF::DR2)
            .with_high(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps64,
        Sps64,
        Config::default()
            .with_low(BF::DR2)
            .with_high(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps128,
        Sps128,
        Config::default()
            .with_high(BF::DR2)
            .with_low(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps250,
        Sps250,
        Config::default()
            .with_high(BF::DR2)
            .with_low(BF::DR1)
            .with_high(BF::DR0)
    );
    test!(
        sps475,
        Sps475,
        Config::default()
            .with_high(BF::DR2)
            .with_high(BF::DR1)
            .with_low(BF::DR0)
    );
    test!(
        sps860,
        Sps860,
        Config::default()
            .with_high(BF::DR2)
            .with_high(BF::DR1)
            .with_high(BF::DR0)
    );
}

#[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
#[cfg_attr(feature = "async", pollster::test)]
#[cfg_attr(feature = "sync", test)]
async fn can_read_measurement_in_progress() {
    let config_os = Config::default().with_low(BF::OS);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![config_os.msb(), config_os.lsb()],
    )];
    let mut dev = new_ads1013(&transactions);
    let in_progress = dev.is_measurement_in_progress().await.unwrap();
    assert!(in_progress);
    destroy_ads1013(dev);
}

#[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
#[cfg_attr(feature = "async", pollster::test)]
#[cfg_attr(feature = "sync", test)]
async fn can_read_measurement_not_in_progress() {
    let config_os = Config::default().with_high(BF::OS);
    let transactions = [I2cTrans::write_read(
        DEV_ADDR,
        vec![Register::CONFIG],
        vec![config_os.msb(), config_os.lsb()],
    )];
    let mut dev = new_ads1013(&transactions);
    let in_progress = dev.is_measurement_in_progress().await.unwrap();
    assert!(!in_progress);
    destroy_ads1013(dev);
}

#[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
#[cfg_attr(feature = "async", pollster::test)]
#[cfg_attr(feature = "sync", test)]
async fn can_convert_to_continuous() {
    let config = Config::default().with_low(BF::OP_MODE);
    let transactions = [I2cTrans::write(
        DEV_ADDR,
        vec![Register::CONFIG, config.msb(), config.lsb()],
    )];
    let dev = new_ads1013(&transactions);
    let dev = dev.into_continuous().await.ok().unwrap();
    destroy_ads1013(dev);
}

#[cfg_attr(feature = "sync", maybe_async::must_be_sync)]
#[cfg_attr(feature = "async", pollster::test)]
#[cfg_attr(feature = "sync", test)]
async fn can_convert_to_one_shot() {
    let config_cont = Config::default().with_low(BF::OP_MODE);
    let config_os = Config::default();
    let transactions = [
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CONFIG, config_cont.msb(), config_cont.lsb()],
        ),
        I2cTrans::write(
            DEV_ADDR,
            vec![Register::CONFIG, config_os.msb(), config_os.lsb()],
        ),
    ];
    let dev = new_ads1013(&transactions);
    let dev = dev.into_continuous().await.ok().unwrap();
    let dev = dev.into_one_shot().await.ok().unwrap();
    destroy_ads1013(dev);
}
