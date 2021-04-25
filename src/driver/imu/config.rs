pub mod ctrl1xl;
pub mod ctrl4c;

pub trait Register {
    fn address(&self) -> u8;
    fn value(&self) -> u8;
}

trait RegisterSetting {
    fn mask(&self) -> u8;
    fn value(&self) -> u8;
}
