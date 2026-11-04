#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub(crate) enum Register {
    Conversion = 0x00,
    Config = 0x01,
    LowThreshold = 0x02,
    HighThreshold = 0x03,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u16)]
pub(crate) enum BitFlags {
    Os = 0b1000_0000_0000_0000,
    Mux2 = 0b0100_0000_0000_0000,
    Mux1 = 0b0010_0000_0000_0000,
    Mux0 = 0b0001_0000_0000_0000,
    Pga2 = 0b0000_1000_0000_0000,
    Pga1 = 0b0000_0100_0000_0000,
    Pga0 = 0b0000_0010_0000_0000,
    OpMode = 0b0000_0001_0000_0000,
    Dr2 = 0b0000_0000_1000_0000,
    Dr1 = 0b0000_0000_0100_0000,
    Dr0 = 0b0000_0000_0010_0000,
    CompMode = 0b0000_0000_0001_0000,
    CompPol = 0b0000_0000_0000_1000,
    CompLat = 0b0000_0000_0000_0100,
    CompQue1 = 0b0000_0000_0000_0010,
    CompQue0 = 0b0000_0000_0000_0001,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct Config {
    pub(crate) bits: u16,
}

impl Config {
    pub(crate) fn is_high(&self, flag: BitFlags) -> bool {
        (self.bits & flag as u16) != 0
    }

    pub(crate) fn with_high(&self, flag: BitFlags) -> Self {
        Config {
            bits: self.bits | flag as u16,
        }
    }

    pub(crate) fn with_low(&self, flag: BitFlags) -> Self {
        Config {
            bits: self.bits & !(flag as u16),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config { bits: 0x8583 }
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Config {
    fn format(&self, formatter: defmt::Formatter) {
        defmt::write!(
            formatter,
            "Config {{ OS: {0=15..16}, MUX: {0=12..15}, PGA: {0=9..12}, \
             OP_MODE: {0=8..9}, DR: {0=5..8}, COMP_MODE: {0=4..5}, \
             COMP_POL: {0=3..4}, COMP_LAT: {0=2..3}, COMP_QUE: {0=0..2} }}",
            self.bits
        );
    }
}
