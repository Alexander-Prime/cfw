use crate::primitive::UNormAxis;

pub struct Button<P, F>
where
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    pub pressed: P,
    pub force: F,
}
