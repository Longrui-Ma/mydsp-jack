use crate::AudioComponent;

pub struct OneZero {
    previous_input: f32,
    b1: f32,             // filter coef
}

impl OneZero {
    pub fn new(b1: f32, ) -> Self {
        OneZero {
            previous_input: 0.0,
            b1,
        }
    }
}

impl AudioComponent for OneZero {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let output = in_frame + self.previous_input * self.b1;
        self.previous_input = in_frame;
        output * 0.5
    }
}
