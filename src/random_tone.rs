extern crate rand;

use std::f32::consts::PI;
use self::rand::Rng;
use self::rand::distributions::{Range, IndependentSample};
use rodio::Source;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct RandomTone {
    freq: f32,
    freqs: Vec<f32>,
    num_sample: usize,
    target_freq: f32,
    chirp_speed: i32,
}

impl RandomTone {
    #[inline]
    pub fn new(freqs: Vec<f32>) -> RandomTone {
        RandomTone {
            freq: freqs[0],
            target_freq: freqs[0],
            freqs,
            num_sample: 0,
            chirp_speed: 100,
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
            let step_size = self.target_freq / self.chirp_speed as f32;
            self.freq += diff.min(step_size).max(-step_size);
        }

        // once per second, pick a new target
        if self.num_sample % 48000 == 0 {
            println!("{}", self.freq);
            let mut rng = rand::thread_rng();
            // choose a random freq from freqs, down up to two octaves or up one octave
            self.target_freq = *rand::thread_rng().choose(&self.freqs).unwrap() * (2.0 as f32).powi(Range::new(-2, 2).ind_sample(&mut rng));
            // take between zero and one second to switch to new tone
            self.chirp_speed = Range::new(0, 48000).ind_sample(&mut rng);
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
