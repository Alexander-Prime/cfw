use super::InputGroup;

pub struct Acceleration<X, Y, Z>
where
    X: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
    Z: Iterator<Item = f32>,
{
    pub x: X,
    pub y: Y,
    pub z: Z,
}

impl<X, Y, Z> Iterator for Acceleration<X, Y, Z>
where
    X: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
    Z: Iterator<Item = f32>,
{
    type Item = (f32, f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y, z) = (self.x.next(), self.y.next(), self.z.next());
        Some((x?, y?, z?))
    }
}

impl<X, Y, Z> InputGroup for Acceleration<X, Y, Z>
where
    X: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
    Z: Iterator<Item = f32>,
{
    fn name(&self) -> &str {
        "Acceleration"
    }
}
