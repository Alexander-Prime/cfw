use crate::primitive::{NormAxis, UNormAxis};

pub struct Touchpad<X, Y, T, P, F>
where
    X: Iterator<Item = NormAxis>,
    Y: Iterator<Item = NormAxis>,
    T: Iterator<Item = bool>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    pub x: X,
    pub y: Y,
    pub touched: T,
    pub pressed: P,
    pub force: F,
}
