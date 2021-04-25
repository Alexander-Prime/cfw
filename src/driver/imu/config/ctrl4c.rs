use super::{Register, RegisterSetting};

pub struct Ctrl4C(
    pub XlBwScalOdr,
    pub SleepG,
    pub Int2OnInt1,
    pub FifoTempEn,
    pub DrdyMask,
    pub I2cDisable,
    pub StopOnFth,
);

impl Register for Ctrl4C {
    fn address(&self) -> u8 {
        0x13
    }

    fn value(&self) -> u8 {
        let Self(xlbwscalodr, sleepg, int2onint1, fifotempen, drdymask, i2cdisable, stoponfth) =
            self;

        xlbwscalodr.value()
            | sleepg.value()
            | int2onint1.value()
            | fifotempen.value()
            | drdymask.value()
            | i2cdisable.value()
            | stoponfth.value()
    }
}

pub enum XlBwScalOdr {
    /// Bandwidth determined by ODR selection
    ByOdr,

    /// Bandwidth determined by setting [`BwXl`] in [`Ctrl1Xl`](super::ctrl1xl::Ctrl1Xl)
    ByBwXl,
}

impl RegisterSetting for XlBwScalOdr {
    fn value(&self) -> u8 {
        match self {
            Self::ByOdr => 0b_0000_0000,
            Self::ByBwXl => 0b_1000_0000,
        }
    }
    fn mask(&self) -> u8 {
        0b_1000_0000
    }
}

pub enum SleepG {
    /// Gyroscope sleep mode enabled
    GyroEnable,

    /// Gyroscope sleep mode disabled
    GyroDisable,
}

impl RegisterSetting for SleepG {
    fn value(&self) -> u8 {
        match self {
            Self::GyroEnable => 0b_0000_0000,
            Self::GyroDisable => 0b_0100_0000,
        }
    }
    fn mask(&self) -> u8 {
        0b_0100_0000
    }
}

pub enum Int2OnInt1 {
    /// all interrupt signals in logic or on INT1 pad
    Int1Only,

    /// interrupt signals divided between INT1 and INT2 pads;
    Int1AndInt2,
}

impl RegisterSetting for Int2OnInt1 {
    fn value(&self) -> u8 {
        match self {
            Self::Int1AndInt2 => 0b_0000_0000,
            Self::Int1Only => 0b_0010_0000,
        }
    }
    fn mask(&self) -> u8 {
        0b_0010_0000
    }
}

/// Enable temperature data as 3rd FIFO data set
pub enum FifoTempEn {
    /// enable temperature data as 3rd FIFO data set
    TempDataEnable,

    /// disable temperature data as 3rd FIFO data set
    TempDataDisable,
}

impl RegisterSetting for FifoTempEn {
    fn value(&self) -> u8 {
        match self {
            Self::TempDataDisable => 0b_0000_0000,
            Self::TempDataEnable => 0b_0001_0000,
        }
    }

    fn mask(&self) -> u8 {
        0b_0001_0000
    }
}

/// Data-ready mask enable. If enabled, when switching from Power-Down to an active mode,
/// the accelerometer and gyroscope data-ready signals are masked until the settling of the sensor
/// filters is completed
pub enum DrdyMask {
    /// Data-ready mask enable
    DrdyMaskEnable,

    /// Data-ready mask disable
    DrdyMaskDisable,
}

impl RegisterSetting for DrdyMask {
    fn value(&self) -> u8 {
        match self {
            Self::DrdyMaskDisable => 0b_0000_0000,
            Self::DrdyMaskEnable => 0b_0000_1000,
        }
    }
    fn mask(&self) -> u8 {
        0b_0000_1000
    }
}

/// Disable I2C interface
pub enum I2cDisable {
    /// both I2C and SPI enabled
    I2cEnable,

    /// I2C disabled, SPI only
    I2cDisable,
}

impl RegisterSetting for I2cDisable {
    fn value(&self) -> u8 {
        match self {
            Self::I2cEnable => 0b_0000_0000,
            Self::I2cDisable => 0b_0000_0100,
        }
    }
    fn mask(&self) -> u8 {
        0b_0000_0100
    }
}

/// Enable FIFO threshold level use.
pub enum StopOnFth {
    /// FIFO depth is limited to threshold level
    FifoDepthLimitedByThreshold,

    /// FIFO depth is not limited
    FifoDepthUnlimited,
}

impl RegisterSetting for StopOnFth {
    fn value(&self) -> u8 {
        match self {
            Self::FifoDepthUnlimited => 0b_0000_0000,
            Self::FifoDepthLimitedByThreshold => 0b_0000_0001,
        }
    }
    fn mask(&self) -> u8 {
        0b_0000_0001
    }
}
