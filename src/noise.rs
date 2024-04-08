//! The `Noise` component generates white noise.
//! 
//! **memo**:
//! `Uniform::new` and `Uniform::new_inclusive` construct a uniform distribution sampling from the given range; 
//!     these functions may do extra work up front to make sampling of multiple values faster.  
//! <https://docs.rs/rand/0.6.5/rand/distributions/uniform/struct.Uniform.html>
//! ## TODO:
//! * Add other types of noise.
use crate::AudioComponent;
// use rand::Rng;
use rand::distributions::{Distribution, Uniform};

#[derive(Debug)]
pub struct WhiteNoise {
    between: Uniform<f32>,
}

impl WhiteNoise {
    /// Creating a `WhiteNoise` instance to generate white noise.
    /// 
    /// # Examples:
    /// Creating a `Noise` instance without any input:
    /// ```rust
    /// # use mydsp_jack::noise::WhiteNoise;
    /// let wn1 = WhiteNoise::new();
    /// ```
    pub fn new() -> WhiteNoise {
        WhiteNoise {
            between: Uniform::from(-1.0..1.0),
        }
    }
}

impl AudioComponent for WhiteNoise {
    fn tick(&mut self, _in_frame: f32) -> f32 {
        // let mut rng = rand::thread_rng(); 
        // self.gain * rng.gen::<f32>() * 2.0 - 1.0 // [-1.0, 1.0)
        let mut rng = rand::thread_rng();
        self.between.sample(&mut rng) // [-1.0, 1.0)
    }
}
