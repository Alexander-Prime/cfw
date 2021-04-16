use super::InputGroup;

pub struct Rotation<P, R, Y>
where
    P: Iterator<Item = f32>,
    R: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
{
    pub pitch: P,
    pub roll: R,
    pub yaw: Y,
}

impl<P, R, Y> Iterator for Rotation<P, R, Y>
where
    P: Iterator<Item = f32>,
    R: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
{
    type Item = (f32, f32, f32);

    fn next(&mut self) -> Option<Self::Item> {
        let (pitch, roll, yaw) = (self.pitch.next(), self.roll.next(), self.yaw.next());
        Some((pitch?, roll?, yaw?))
    }
}

impl<P, R, Y> InputGroup for Rotation<P, R, Y>
where
    P: Iterator<Item = f32>,
    R: Iterator<Item = f32>,
    Y: Iterator<Item = f32>,
{
    fn name(&self) -> &str {
        "Rotation"
    }
}

pub trait Rot: IntoIterator<Item = (f32, f32, f32)> {}
