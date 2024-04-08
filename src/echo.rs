//! The `Echo` component includes `feedback` control and `dry/wet` control, using a fixed delay from the `Delay` instance.
//! 
//! Returns output = dry * input + wet * (input + delayed input * feedback)
//! 
//! ##  Using `Option<f32>` for default values
//! Rust does not have a null type, so `Option<f32>` is used here, which contains `Some()` and `None`.  
//! Without `phase: impl Into<Option<f32>>` and `phase.into().unwrap_or(0.0)`, this usage would not be possible:
//! ```rust
//! # use mydsp_jack::delay::Delay;
//! # use mydsp_jack::echo::Echo;
//! // Pass `None` to `dry/wet` to indicate using default values (feedback=0.0, dry=0.5, wet=1.0-dry):
//! let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
//! let echo0 = Echo::new(nframes_delay, None, None, None);
//! ```
//! However, values of `feedback`, `dry`, `wet` should only be `f32` or `None`. 
//! Integers are not supported. (Monomorphization)
//! 
//! **memo**:
//! To achieve using default parameters, I have thought about other methods like using Default trait, 
//! builder pattern or simply exposing fields which contain default values like `pub phase`, 
//! but `phase: impl Into<Option<f32>>` seems to be the most elegant.
use crate::AudioComponent;
use crate::delay::Delay;

#[derive(Debug)]
pub struct Echo {
    delay: Delay,
    feedback: f32, // feedback control
    dry: f32, // coefficient of original signal
    wet: f32, // coefficient of processed signal
}

impl Echo {
    /// Returns a `Echo` instance with a fixed size specified by nframes_delay.
    /// 
    /// # Examples:
    /// Creating a `Delay` instance with a circular buffer size = 3:
    /// ```rust
    /// # use mydsp_jack::delay::Delay;
    /// # use mydsp_jack::echo::Echo;
    /// let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
    /// // output = dry(0.7) * input + wet(0.3) * (input + delayed input * feedback(0.6))
    /// let echo1 = Echo::new(nframes_delay, 0.6, 0.7, None); // aka. let echo1 = Echo::new(nframes_delay, 0.6, 0.7, 0.3);
    /// ```
    /// # Panics
    /// The function panics if not 0.0<=`feedback`<=1.0, and (dry<0.0 or wet<0.0 or dry+wet!=1).
    /// ```rust, should_panic
    /// # use mydsp_jack::delay::Delay;
    /// # use mydsp_jack::echo::Echo;
    /// # let nframes_delay:usize = 48000/2; // delay of 0.5s if sample rate = 48000Hz
    /// // output = dry(0.7) * input + wet(0.5) * (input + delayed input * feedback(2.0))
    /// let echo1 = Echo::new(nframes_delay, 2.0, None, 0.7);
    /// ```
    pub fn new(nframes_delay: usize, feedback: impl Into<Option<f32>>, dry: impl Into<Option<f32>>, wet: impl Into<Option<f32>>) -> Self {
        // default: no feedback (=0), dry/wet ratio = 1:1 (=0.5 each)
        let feedback = feedback.into().unwrap_or(0.0);
        if !(0.0..=1.0).contains(&feedback) {
            panic!("!!!Echo panic: Feedback must be between 0.0 and 1.0");
        }
        let dry = dry.into().unwrap_or(0.5);
        let wet = wet.into().unwrap_or(1.0 - dry);
        if dry < 0.0 || wet < 0.0 || (dry + wet - 1.0).abs() > f32::EPSILON {
            panic!("!!!Echo panic: Dry and Wet coefficients must be non-negative and their sum must equal 1.0");
        }
        Echo {
            delay: Delay::new(nframes_delay),
            feedback,
            dry,
            wet,
        }
    }
}

impl AudioComponent for Echo {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let delayed_frame = self.delay.tick(in_frame + self.feedback * self.delay.read());
        self.dry * in_frame + self.wet * delayed_frame
    }
}
