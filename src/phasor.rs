//! The `Phasor` component for generating a phase that increments between [0.0, 1.0), creating oscillators or modulators. 
use crate::AudioComponent;

#[derive(Debug)]
pub struct Phasor {
    phase: f32, // current phase
    phase_increment: f32, // per tick
}

impl Phasor {
    /// Returns a `Phasor` instance with a given `initial_phase` and `phase_increment`.
    /// 
    /// # Examples:
    /// Creating a `Phasor` instance with an initial phase of 0.0 and a phase increment of 0.1:
    /// ```rust
    /// # use mydsp_jack::phasor::Phasor;
    /// # use mydsp_jack::AudioComponent;
    /// // Pass `None` to `initial_phase` to indicate using default value (initial_phase = 0.0): (more info in echo.rs)
    /// let phasor1 = Phasor::new(None, 0.1); // aka: let phasor1 = Phasor::new(0.0, 0.1); 
    /// # let mut phasor3 = Phasor::new(0.95, 0.1); // cannot assert if cannot borrow phasor1 as mutable
    /// # let resp3 = phasor3.tick(0.0);
    /// # //println!("{}", resp3);
    /// # debug_assert_eq!(resp3, 0.049999952); // Phase wraps from 0.95 + 0.1 -> 0.049999952
    /// ```
    /// # Panics
    /// The function panics if `initial_phase` is not in the range [0.0, 1.0) or warns if `phase_increment` is 0.0.
    /// ```rust, should_panic
    /// # use mydsp_jack::phasor::Phasor;
    /// let phasor_warn = Phasor::new(0.0, 0.0); // warning
    /// let phasor_panic = Phasor::new(2.0, 0.1); // panic
    /// ```
    pub fn new(initial_phase: impl Into<Option<f32>>, phase_increment: f32) -> Self {
        let initial_phase = initial_phase.into().unwrap_or(0.0);
        if initial_phase < 0.0 || initial_phase >= 1.0 {
            panic!("!!!Panic: initial_phase must be in the range [0, 1)");
        }
        if phase_increment == 0.0 {
            println!("!!!Warning: phase_increment set to 0");
        }
        Phasor { 
            phase: initial_phase, 
            phase_increment 
        }
    }
    /// Modifies phase increment.
    /// 
    /// # Examples:
    /// Updating the `Phasor` with a given `phase_increment`:
    /// ```rust
    /// # use mydsp_jack::phasor::Phasor;
    /// # use mydsp_jack::AudioComponent;
    /// # let mut phasor2 = Phasor::new(0.0, 0.1);
    /// phasor2.set_phase_increment(0.05);
    /// # debug_assert_eq!(phasor2.tick(0.0), 0.05); 
    /// ```
    pub fn set_phase_increment(&mut self, phase_increment: f32) {
        if phase_increment == 0.0 {
            println!("!!!Warning: phase_increment set to 0");
        }
        self.phase_increment = phase_increment;
    }
}

impl AudioComponent for Phasor {
    fn tick(&mut self, _in_frame: f32) -> f32 {
        self.phase += self.phase_increment;
        self.phase -= self.phase.floor(); // phase wraps around 0 to 1
        self.phase
    }
}
