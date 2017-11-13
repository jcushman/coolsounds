use std::f32::consts::PI;
use rand::{Rng, thread_rng};
use rand::distributions::{Range, IndependentSample};
use rodio::Source;
use std::time::Duration;


// Move from sine wave to sine wave in a chirpy way, by sliding the sine frequency once per second
#[derive(Clone, Debug)]
pub struct RandomTone {
    freq: f32,
    freqs: Vec<f32>,
    num_sample: usize,
    target_freq: f32,
    chirp_step_size: f32,
    chirp_max_length: i32,
}

impl RandomTone {
    #[inline]
    pub fn new(freqs: Vec<f32>, chirp_max_length: i32) -> RandomTone {
        RandomTone {
            freq: freqs[0],
            target_freq: freqs[0],
            freqs,
            num_sample: 0,
            chirp_step_size: 1.0,
            chirp_max_length,
        }
    }
}

impl Iterator for RandomTone {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        self.num_sample = self.num_sample.wrapping_add(1);

        let value = 2.0 * PI * self.freq * self.num_sample as f32 / 48000.0;

        // move freq towards target_freq
        if self.freq != self.target_freq {
            let diff = self.target_freq - self.freq;
            self.freq += diff.min(self.chirp_step_size).max(-self.chirp_step_size);
        }

        // once per second, pick a new target
        if self.num_sample % 48000 == 0 {
            let mut rng = thread_rng();
            // choose a random freq from freqs, down up to two octaves or up one octave
            self.target_freq = *thread_rng().choose(&self.freqs).unwrap() * (2.0 as f32).powi(Range::new(-2, 2).ind_sample(&mut rng));
            // take between zero and one second to switch to new tone
            let gap = (self.target_freq - self.freq).abs();
            self.chirp_step_size = gap / Range::new(1, self.chirp_max_length).ind_sample(&mut rng) as f32;
        }

        Some(value.sin())
    }
}

impl Source for RandomTone {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    #[inline]
    fn channels(&self) -> u16 {
        1
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        48000
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        None
    }
}
