use super::{Register, RegisterSetting};

/// Linear acceleration sensor control register 1 (r/w).
pub struct Ctrl1Xl(pub OdrXl, pub FsXl, pub BwXl);

impl Register for Ctrl1Xl {
    fn address(&self) -> u8 {
        0x10
    }

    fn value(&self) -> u8 {
        let Self(odrxl, fsxl, bwxl) = self;

        odrxl.value() | fsxl.value() | bwxl.value()
    }
}

/// Output data rate and power mode selection.
pub enum OdrXl {
    /// Power-down
    PowerDown,

    /// 12.5 Hz
    DataRate12_5Hz,

    /// 26 Hz
    DataRate26Hz,

    /// 52 Hz
    DataRate52Hz,

    /// 104 Hz
    DataRate104Hz,

    /// 208 Hz
    DataRate208Hz,

    /// 416 Hz
    DataRate416Hz,

    /// 833 Hz
    DataRate833Hz,

    /// 1.66 kHz
    DataRate1_66Khz,

    /// 3.33 kHz
    DataRate3_33Khz,

    /// 6.66 kHz
    DataRate6_66Khz,
}

impl RegisterSetting for OdrXl {
    fn value(&self) -> u8 {
        match self {
            Self::PowerDown => 0b_0000_0000,
            Self::DataRate12_5Hz => 0b_0001_0000,
            Self::DataRate26Hz => 0b_0010_0000,
            Self::DataRate52Hz => 0b_0011_0000,
            Self::DataRate104Hz => 0b_0100_0000,
            Self::DataRate208Hz => 0b_0101_0000,
            Self::DataRate416Hz => 0b_0110_0000,
            Self::DataRate833Hz => 0b_0111_0000,
            Self::DataRate1_66Khz => 0b_1000_0000,
            Self::DataRate3_33Khz => 0b_1001_0000,
            Self::DataRate6_66Khz => 0b_1010_0000,
        }
    }

    fn mask(&self) -> u8 {
        0b_1111_0000
    }
}

/// Accelerometer full-scale selection.
pub enum FsXl {
    /// ±2g
    TwoG,

    /// ±4g
    FourG,

    /// ±8g
    EightG,

    /// ±16g
    SixteenG,
}

impl RegisterSetting for FsXl {
    fn value(&self) -> u8 {
        match self {
            Self::TwoG => 0b_0000_0000,
            Self::FourG => 0b_0000_1000,
            Self::EightG => 0b_0000_1100,
            Self::SixteenG => 0b_0000_0100,
        }
    }

    fn mask(&self) -> u8 {
        0b_0000_1100
    }
}
/// Anti-aliasing filter bandwidth selection.
pub enum BwXl {
    /// 50 Hz
    Bw50Hz,

    /// 100 Hz
    Bw100Hz,

    /// 200 Hz
    Bw200Hz,

    /// 400 Hz
    Bw400Hz,
}

impl RegisterSetting for BwXl {
    fn value(&self) -> u8 {
        match self {
            Self::Bw50Hz => 0b_0000_0011,
            Self::Bw100Hz => 0b_0000_0010,
            Self::Bw200Hz => 0b_0000_0001,
            Self::Bw400Hz => 0b_0000_0000,
        }
    }

    fn mask(&self) -> u8 {
        0b_0000_0011
    }
}
