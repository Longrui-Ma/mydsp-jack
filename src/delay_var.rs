//! The `DelayVar` component reads(with offset)/writes and returns frames of changeble delay in a fixed-size circular buffer.
//! 
//! offset_delay = 0 -> no delay (at t);  
//! offset_delay = 1 -> read previous frame (at t-1);  
//! offset_delay = 2 -> read previous frame (at t-2);  
//! 
//! time of delay = offset_delay / sample_rate  
//! max time delay = buffer_size - 1 / sample_rate  
//! offset_delay < buffer_size  
use crate::AudioComponent;

#[derive(Debug)]
pub struct DelayVar {
    buffer_delay: Vec<f32>, // circular buffer to store delayed frames.
    index_write: usize, // position in buffer_delay to write.
    index_read: usize, // position in buffer_delay to read.
    buffer_size: usize, // size of the circular buffer (fixed).
    offset_delay: usize, // offset of the read index from the write index.
}

impl DelayVar {
    /// Returns a `DelayVar` instance with a fixed size specified by buffer_size and an initial offset_delay.
    /// 
    /// # Examples:
    /// Creating a `DelayVar` instance with a circular buffer size = 3:
    /// ```rust
    /// # use mydsp_jack::delay_var::DelayVar;
    /// # use mydsp_jack::AudioComponent;
    /// let mut delay1 = DelayVar::new(3, 0); // buffer_size (size) = 3, no delay
    /// debug_assert_eq!(delay1.tick(1.0), 1.0); // first in_frame = 1.0 at postition 1
    /// debug_assert_eq!(delay1.tick(2.0), 2.0); // second in_frame = 2.0 at postition 2
    /// debug_assert_eq!(delay1.tick(3.0), 3.0); // third in_frame = 3.0 at postition 3
    /// debug_assert_eq!(delay1.tick(4.0), 4.0); // 4.0 at position 1
    /// let mut delay2 = DelayVar::new(3, 1); // buffer_size (size) = 3, delayed 1 frame
    /// debug_assert_eq!(delay2.tick(1.0), 0.0); // first in_frame = 1.0 at postition 1, read delay at position 3
    /// debug_assert_eq!(delay2.tick(2.0), 1.0); // second in_frame = 2.0 at postition 2, read delay at position 1
    /// debug_assert_eq!(delay2.tick(3.0), 2.0); // third in_frame = 3.0 at postition 3, read delay at position 2
    /// debug_assert_eq!(delay2.tick(4.0), 3.0); // 4.0 at position 1, read delay at position 3
    /// ```
    /// # Panics
    /// The function panics if `buffer_size` (circular buffer size) is not a positive integer.
    /// ```rust, should_panic
    /// # use mydsp_jack::delay_var::DelayVar;
    /// let delay_panic = DelayVar::new(0, 1);
    /// ```
    pub fn new(buffer_size: usize, offset_delay: usize) -> Self {
        if buffer_size <= 0 {
            panic!("!!!Panic: buffer_size must be a positive integer");
        }
        if offset_delay >= buffer_size {
            panic!("!!!Panic: offset_delay must be less than buffer_size");
        }
        DelayVar {
            buffer_delay: vec![0.0; buffer_size],
            index_write: 0,
            index_read: 0,
            buffer_size,
            offset_delay,
        }
    }
    pub fn read(&self) -> f32 {
        self.buffer_delay[self.index_read]
    }
    /// Sets `offset_delay` to modify time of delay.
    /// 
    /// # Examples:
    /// Seting a `offset_delay` from 0(no delay) to 2:
    /// ```rust
    /// # use mydsp_jack::delay_var::DelayVar;
    /// # use mydsp_jack::AudioComponent;
    /// let mut delay3 = DelayVar::new(3, 0); // buffer_size (size) = 3, no delay
    /// debug_assert_eq!(delay3.tick(1.0), 1.0); // first in_frame = 1.0 at postition 1, read delay at position 1
    /// debug_assert_eq!(delay3.tick(2.0), 2.0); // second in_frame = 2.0 at postition 2, read delay at position 2
    /// delay3.set_offset_delay(2);
    /// debug_assert_eq!(delay3.tick(3.0), 1.0); // third in_frame = 3.0 at postition 3, read delay at position 1
    /// debug_assert_eq!(delay3.tick(4.0), 2.0); // 4.0 at position 1, read delay at position 2
    /// ```
    /// # Panics
    /// The function panics if `buffer_size` (circular buffer size) is not a positive integer.
    /// ```rust, should_panic
    /// # use mydsp_jack::delay_var::DelayVar;
    /// let mut delay_panic = DelayVar::new(3, 1);
    /// delay_panic.set_offset_delay(3);
    /// ```
    pub fn set_offset_delay(&mut self, offset_delay: usize) {
        if offset_delay >= self.buffer_size {
            panic!("!!!Panic: offset_delay must be less than buffer_size");
        }
        self.offset_delay = offset_delay;
    }
}

impl AudioComponent for DelayVar {
    fn tick(&mut self, in_frame: f32) -> f32 {
        self.buffer_delay[self.index_write] = in_frame; // write
        self.index_read = (self.index_write + self.buffer_size - self.offset_delay) % self.buffer_size; // update read position
        let delayed_frame = self.buffer_delay[self.index_read]; // read
        self.index_write = (self.index_write + 1) % self.buffer_size; // update write position
        delayed_frame
    }
}
