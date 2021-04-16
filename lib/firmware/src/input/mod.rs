mod acceleration;
mod button;
mod rotation;
mod stick;
mod touchpad;
mod trigger;

pub use acceleration::Acceleration;
pub use button::Button;
pub use rotation::Rotation;
pub use stick::Stick;
pub use touchpad::Touchpad;
pub use trigger::Trigger;

pub trait InputGroup: Iterator {
    fn name(&self) -> &str;
}
