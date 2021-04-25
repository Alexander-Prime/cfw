pub mod ctrl1xl;

trait Register {
    fn address(&self) -> u8;
    fn value(&self) -> u8;
}

trait RegisterSetting {
    fn mask(&self) -> u8;
    fn value(&self) -> u8;
}
