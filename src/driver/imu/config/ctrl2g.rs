use super::{Register, RegisterSetting};

/// Angular rate sensor control register 2 (r/w).
pub struct Ctrl2G(pub OdrG, pub FsG, pub Fs125);

impl Register for Ctrl2G {
    fn address(&self) -> u8 {
        0x11
    }

    fn value(&self) -> u8 {
        let Self(odrg, fsg, fs125) = self;
        odrg.value() | fsg.value() | fs125.value()
    }
}

/// Gyroscope output data rate selection.
pub enum OdrG {
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
}

impl RegisterSetting for OdrG {
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
        }
    }

    fn mask(&self) -> u8 {
        0b_1111_0000
    }
}

/// Gyroscope full-scale selection.
pub enum FsG {
    /// 250 dps
    Dps250,

    /// 500 dps
    Dps500,

    /// 1000 dps
    Dps1000,

    /// 2000 dps
    Dps2000,
}

impl RegisterSetting for FsG {
    fn value(&self) -> u8 {
        match self {
            Self::Dps250 => 0b_0000_0000,
            Self::Dps500 => 0b_0000_0100,
            Self::Dps1000 => 0b_0000_1000,
            Self::Dps2000 => 0b_0000_1100,
        }
    }

    fn mask(&self) -> u8 {
        0b_0000_1100
    }
}
/// Gyroscope full-scale at 125 dps.
pub enum Fs125 {
    DpsByFsG,
    Dps125,
}

impl RegisterSetting for Fs125 {
    fn value(&self) -> u8 {
        match self {
            Self::DpsByFsG => 0b_0000_0000,
            Self::Dps125 => 0b_0000_0010,
        }
    }

    fn mask(&self) -> u8 {
        0b_0000_0010
    }
}
