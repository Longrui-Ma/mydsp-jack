use crate::sine_table::SineTable;
use crate::phasor::Phasor;
use crate::echo::Echo;
use crate::sine::SineWave;
// use crate::dummy::Dummy;
use crate::AudioComponent;
// TODO: 
// * bug ?
pub struct Flanger {
    sine_wave: SineWave, // TODO: WN etc.
    echo: Echo,
    nframes_delay: usize,
    lfo_index: f32,
    depth: f32,
}

impl Flanger {
    pub fn new(sine_table: &SineTable, phasor: Phasor, gain:f32, lfo_index: f32, depth: f32, nframes_delay: usize, feedback: f32, dry: f32, wet: f32) -> Self {
        let sine_wave = SineWave::new(sine_table, phasor, gain);
        // let echo = Echo::new(nframes_delay, feedback, port_pairs, Box::new(Dummy::new())); //in_app move to echo
        let echo = Echo::new(nframes_delay, feedback, dry, wet);
        Flanger {
            sine_wave,
            echo,
            nframes_delay,
            lfo_index,
            depth,
        }
    }
}

impl AudioComponent for Flanger {
    fn tick(&mut self, in_frame: f32) -> f32 {
        let lfo = self.sine_wave.tick(in_frame) * 0.5 + 0.5;
        self.echo.set_delay((self.nframes_delay as f32 * (1.0 - lfo * self.lfo_index)) as usize);
        self.echo.tick(in_frame) * self.depth
    }
}
