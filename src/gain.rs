//! The `Gain` component modifies the volume between (-inf, inf) by multiplying it with other components.
//! 
//! (0.0->0%, 1.0->100%) 
use crate::AudioComponent;

pub struct Gain {
    gain: f32,
}

impl Gain {
    /// Creating a `Gain` instance to control volume.
    /// 
    /// # Examples:
    /// Creating a `Gain` instance using volume between (-inf, inf) (0.0->0%, 1.0->100%):
    /// ```rust
    /// # use mydsp_jack::gain::Gain;
    /// # use mydsp_jack::AudioComponent;
    /// let mut gain5 = Gain::new(0.5); // 50% volume
    /// # debug_assert_eq!(gain5.tick(0.0), 0.5); 
    /// ```
    pub fn new(gain:f32) -> Self {
        if gain == 0.0 {
            println!("!!!Warning: gain set to 0.0");
        }
        Gain {gain}
    }
    /// Modifies gain.
    /// 
    /// # Examples:
    /// Modifying volume to a value between (-inf, inf) (0.0->0%, 1.0->100%):
    /// ```rust
    /// # use mydsp_jack::gain::Gain;
    /// # use mydsp_jack::AudioComponent;
    /// # let mut gain5 = Gain::new(0.5); // 50% volume
    /// # debug_assert_eq!(gain5.tick(0.0), 0.5); 
    /// gain5.set_gain(2.0); // set to 200% volume
    /// # debug_assert_eq!(gain5.tick(0.0), 2.0);
    /// ```
    pub fn set_gain(&mut self, gain: f32) {
        if gain == 0.0 {
            println!("!!!Warning: gain set to 0.0");
        }
        self.gain = gain;
    }
}

impl AudioComponent for Gain {
    fn tick(&mut self, _in_frame: f32) -> f32 {
        self.gain
    }
}
