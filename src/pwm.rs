//! The `Pwm` component controls duty cycle.
use crate::AudioComponent;

pub struct Pwm {
    // duty_cycle: f32, // range: [0,1]
    current_frame: usize, 
    on_frame: usize,
    period: usize, // e.g. sample_rate => 1s
}

impl Pwm {
    /// Creating a `Pwm` instance to control duty cycle [0.0, 1.0] (from 0% to 100%).
    /// 
    /// # Examples:
    /// Creating a `Pwm` instance with a 50% duty cycle in a period of 1s:
    /// ```rust
    /// # use mydsp_jack::pwm::Pwm;
    /// # use mydsp_jack::AudioComponent;
    /// let pwm1 = Pwm::new(0.5, 48000); // period = 1s if sample rate = 48000Hz
    /// ```
    /// # Panics
    /// The function panics if `duty_cycle` is not in the range [0.0, 1.0] or `period` <= 0.
    /// ```rust, should_panic
    /// # use mydsp_jack::pwm::Pwm;
    /// let pwm_panic = Pwm::new(2.0, 0);
    /// ```
    pub fn new(duty_cycle: f32, period: usize) -> Self {
        if duty_cycle < 0.0 || duty_cycle > 1.0 {
            panic!("!!!Panic: duty_cycle must be in the range [0, 1]");
        }
        if period <= 0 {
            panic!("!!!Panic: duty_cycle must be a positive integer");
        }
        Pwm {
            current_frame: 0,
            on_frame: (period as f32 * duty_cycle) as usize,
            period,
        }
    }
}

impl AudioComponent for Pwm {
    fn tick(&mut self, _in_frame: f32) -> f32 {
        let output = if self.current_frame < self.on_frame {
            1.0
        } else {
            0.0
        };
        self.current_frame = (self.current_frame + 1) % self.period;  
        output
    }
}
