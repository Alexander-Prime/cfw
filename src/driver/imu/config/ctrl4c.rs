use super::{Register, RegisterSetting};

pub struct Ctrl4C(
    XlBwScalOdr,
    SleepG,
    Int2OnInt1,
    FifoTempEn,
    DrdyMask,
    I2cDisable,
    StopOnFth,
);

pub enum XlBwScalOdr {}

pub enum SleepG {}

pub enum Int2OnInt1 {}

pub enum FifoTempEn {}

pub enum DrdyMask {}

pub enum I2cDisable {}

pub enum StopOnFth {}
