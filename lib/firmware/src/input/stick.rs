use crate::primitive::{NormAxis, UNormAxis};

pub struct Stick<X, Y, P, F>
where
    X: Iterator<Item = NormAxis>,
    Y: Iterator<Item = NormAxis>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    pub x: X,
    pub y: Y,
    pub pressed: P,
    pub force: F,
}
