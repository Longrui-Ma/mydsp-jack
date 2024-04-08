//! The `Delay` component reads/writes and returns delayed frame in a fixed-size circular buffer.
//! 
//! nframes_delay / sample_rate = time of delay.  
//! e.g. nframes_delay = sample_rate / 2;  // delay of 0.5s.
use crate::AudioComponent;

#[derive(Debug)]
pub struct Delay {
    buffer_delay: Vec<f32>, // circular buffer to store delayed frames.
    index: usize, // position in buffer_delay to read and write.
    nframes_delay: usize, // size of the circular buffer (fixed). 
}

impl Delay {
    /// Returns a `Delay` instance with a fixed size specified by nframes_delay.
    /// 
    /// # Examples:
    /// Creating a `Delay` instance with a circular buffer size = 3:
    /// ```rust
    /// # use mydsp_jack::delay::Delay;
    /// # use mydsp_jack::AudioComponent;
    /// let mut delay1 = Delay::new(3); // nframes_delay (size) = 3
    /// delay1.tick(1.0); // first in_frame = 1.0
    /// delay1.tick(2.0); // second in_frame = 2.0
    /// delay1.tick(3.0); // third in_frame = 3.0
    /// debug_assert_eq!(delay1.tick(4.0), 1.0); 
    /// // the oldest frame = 1.0, which is replaced by 4.0 after `tick()` and becomes the newest frame.
    /// ```
    /// # Panics
    /// The function panics if `nframes_delay` (circular buffer size) is not a positive integer.
    /// ```rust, should_panic
    /// # use mydsp_jack::delay::Delay;
    /// let delay2 = Delay::new(0);
    /// ```
    pub fn new(nframes_delay: usize, ) -> Self {
        if nframes_delay <= 0 {
            panic!("!!!Panic: nframes_delay (circular buffer size) must be a positive integer");
        }
        Delay {
            buffer_delay: vec![0.0; nframes_delay],
            index: 0,
            nframes_delay,
        }
    }
    /// Return the oldest delayed_frame.  
    /// memo: read() is called (index=t-1) before tick() (index=t), so read() return the oldest frame
    /// 
    /// # Examples:
    /// Creating a `Delay` instance with a circular buffer size = 3:
    /// ```rust
    /// # use mydsp_jack::delay::Delay;
    /// # use mydsp_jack::AudioComponent;
    /// let mut delay3 = Delay::new(3); // nframes_delay (size) = 3
    /// delay3.tick(1.0); // first in_frame = 1.0
    /// delay3.tick(2.0); // second in_frame = 2.0
    /// delay3.tick(3.0); // third in_frame = 3.0
    /// debug_assert_eq!(delay3.read(), 1.0); // before `tick()`, the oldest frame = 1.0.
    /// debug_assert_eq!(delay3.tick(4.0), 1.0); // `tick()` increments index.
    /// debug_assert_eq!(delay3.read(), 2.0); // after `tick()`, the oldest frame = 2.0.
    /// ```
    pub fn read(&self) -> f32 {
        self.buffer_delay[self.index]
    }
}

impl AudioComponent for Delay {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let delayed_frame = self.buffer_delay[self.index]; // read.
        self.buffer_delay[self.index] = in_frame; // write.
        self.index = (self.index + 1) % self.nframes_delay; // update write position.
        delayed_frame
    } 
}
