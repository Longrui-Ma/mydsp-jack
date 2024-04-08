use crate::AudioComponent;

pub struct Distortion {
    drive: f32,
    offset: f32,
    gain: f32,
}

impl Distortion {
    pub fn new(drive: f32, offset: f32, gain: f32, ) -> Self {
        Distortion {
            drive,
            offset,
            gain,
        }
    }
    fn cubic(&self, x: f32) -> f32 {
        x - x.powi(3) / 3.0
    }
}

impl AudioComponent for Distortion {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let mut output = in_frame * 10.0_f32.powf(2.0 * self.drive) + self.offset;
        output = output.clamp(-1.0, 1.0); //limit output in [-1.0, 1.0]
        output = self.cubic(output);
        output * self.gain
    }
}
