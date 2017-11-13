use rodio::Source;
use rodio::source::{SineWave, TakeDuration};
use std::time::Duration;


// Play a sequence of notes with durations:
pub struct TonePattern {
    tones: Vec<Vec<u32>>,
    index: usize,
}
impl TonePattern {
    pub fn new(tones: Vec<Vec<u32>>) -> TonePattern {
        TonePattern {tones, index: 0}
    }
}

impl Iterator for TonePattern {
    type Item = TakeDuration<SineWave>;

    #[inline]
    fn next(&mut self) -> Option<TakeDuration<SineWave>> {
        let tone = &self.tones[self.index];
        let source = SineWave::new(tone[0]).take_duration(Duration::from_millis(tone[1] as u64));
        self.index = (self.index + 1) % self.tones.len();
        Some(source)
    }
}
