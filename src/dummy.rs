//! The `Dummy` component returns input without any modification.
use crate::AudioComponent;

pub struct Dummy {}

impl Dummy {
    /// Creating a `Dummy` instance to return input without any modification.
    /// 
    /// # Examples:
    /// Creating a `Dummy` instance without any input:
    /// ```rust
    /// # use mydsp_jack::dummy::Dummy;
    /// let dummy = Dummy::new();
    /// ```
    pub fn new() -> Self {
        Dummy {}
    }
}

impl AudioComponent for Dummy {
    fn tick(&mut self, in_frame: f32) -> f32 {
        in_frame
    }
}
