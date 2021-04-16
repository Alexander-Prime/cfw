use crate::primitive::UNormAxis;

pub struct Trigger<T, P, F>
where
    T: Iterator<Item = UNormAxis>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    pub travel: T,
    pub pressed: P,
    pub force: F,
}
