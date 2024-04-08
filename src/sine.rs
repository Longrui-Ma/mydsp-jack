//! The `SineWave` component for generating a sinewave using a static sinetable and a `Phasor` instance.
use crate::AudioComponent;
use crate::sine_table::SineTable;
use crate::phasor::Phasor;

#[derive(Debug)]
pub struct SineWave {
    sine_table: &'static SineTable,
    phasor: Phasor, // Move ownership
}

impl SineWave {
    /// Returns a `SineWave` instance by referencing a static `SineTable` and moving the `Phasor` instance.
    /// 
    /// # Examples:
    /// Creating a `SineWave` instance of 440Hz:
    /// ```rust
    /// # use mydsp_jack::phasor::Phasor;
    /// # use mydsp_jack::sine_table::SineTable;
    /// # use mydsp_jack::sine::SineWave;
    /// # use mydsp_jack::AudioComponent;
    /// use once_cell::sync::Lazy;
    /// static SINETABLE1: Lazy<SineTable> = Lazy::new(|| SineTable::new(4096));
    /// let phase_increment = (440/48000) as f32;
    /// let phasor1 = Phasor::new(None, phase_increment);
    /// # let phasor2 = Phasor::new(0.49, phase_increment);
    /// let sinewave1 = SineWave::new(&SINETABLE1, phasor1);
    /// # let mut sinewave2 = SineWave::new(&SINETABLE1, phasor2);
    /// # debug_assert_eq!(sinewave2.tick(0.0), 0.06285161);
    /// ```
    pub fn new(sine_table: &'static SineTable, phasor: Phasor, ) -> Self {
        SineWave {
            sine_table,
            phasor,
        }
    }
}

impl AudioComponent for SineWave {
    fn tick(&mut self, in_frame: f32) -> f32 {
        self.sine_table.get_value(self.phasor.tick(in_frame))
    }
}
