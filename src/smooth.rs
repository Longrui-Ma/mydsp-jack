use crate::AudioComponent;

pub struct Smooth {
    previous_output: f32,
    smoothing_factor: f32,
}

impl Smooth {
    pub fn new(smoothing_factor: f32, ) -> Self {
        Smooth {
            previous_output: 0.0,
            smoothing_factor,
        }
    }
}

impl AudioComponent for Smooth {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let output = (1.0 - self.smoothing_factor) * in_frame + self.smoothing_factor * self.previous_output;
        self.previous_output = output;
        output
    }
}