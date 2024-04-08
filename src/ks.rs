// Karplus Strong
use crate::AudioComponent;
use crate::one_zero::OneZero;
// use crate::dummy::Dummy;
// TODO: 
// * bug ?
pub struct KS {
    delay_buffer: Vec<f32>, 
    read_index: usize, 
    write_index: usize, 
    feedback: f32,
    delay_length: usize,
    is_triggered: bool,
    one_zero: OneZero, 
}

impl KS {
    pub fn new(sample_rate: usize, feedback: f32, freq: f32, b1: f32, ) -> Self {
        let delay_length = (sample_rate as f32 / freq).round() as usize;
        KS {
            delay_buffer: vec![0.0; delay_length],
            read_index: delay_length - 1,
            write_index: 0,
            feedback,
            delay_length,
            is_triggered: false,
            one_zero: OneZero::new(b1),
        }
    }
    pub fn trigger(&mut self) {
        self.is_triggered = true;
    }
}

impl AudioComponent for KS {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let excitation = if self.is_triggered { self.is_triggered = false; 1.0 } else { 0.0 };
        let input = in_frame + excitation + self.delay_buffer[self.read_index];
        let filtered_input = self.one_zero.tick(input);
        let output = filtered_input + self.feedback * self.delay_buffer[self.read_index]; 
        self.delay_buffer[self.write_index] = output;
        self.read_index = (self.read_index + 1) % self.delay_length;
        self.write_index = (self.write_index + 1) % self.delay_length;
        output
    }
}
