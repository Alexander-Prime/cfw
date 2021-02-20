use crate::primitive::{NormAxis, UNormAxis};

pub trait InputGroup {
    fn name(&self) -> &str;
}

pub struct Button<P, F>
where
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    pressed: P,
    force: F,
}

pub struct Trigger<T, P, F>
where
    T: Iterator<Item = UNormAxis>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    travel: T,
    pressed: P,
    force: F,
}

pub struct Stick<X, Y, P, F>
where
    X: Iterator<Item = NormAxis>,
    Y: Iterator<Item = NormAxis>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    x: X,
    y: Y,
    pressed: P,
    force: F,
}

pub struct Touchpad<X, Y, T, P, F>
where
    X: Iterator<Item = NormAxis>,
    Y: Iterator<Item = NormAxis>,
    T: Iterator<Item = bool>,
    P: Iterator<Item = bool>,
    F: Iterator<Item = UNormAxis>,
{
    x: X,
    y: Y,
    touched: T,
    pressed: P,
    force: F,
}
