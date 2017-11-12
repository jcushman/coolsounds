use rodio::{Decoder, Source};
use rodio::buffer::SamplesBuffer;
use std::io::BufReader;
use std::fs::File;
use rand::{Rng, thread_rng};
use rand::distributions::{Range, IndependentSample};

pub struct FileSampler {
    data: Vec<i16>,
    channels: u16,
    samples_rate: u32,
    samples_per_second: f32,
    regions: Vec<Vec<f32>>,
}

impl FileSampler {
    pub fn new(fname: &str, regions: Vec<Vec<f32>>) -> FileSampler {
        // parse audio file
        let file = File::open(fname).unwrap();
        let decoder = Decoder::new(BufReader::new(file)).unwrap();

        // build struct
        let samples_rate = decoder.samples_rate();
        let channels = decoder.channels();
        let samples_per_second = samples_rate as f32 * channels as f32;
        let data = decoder.collect::<Vec<i16>>();

        FileSampler {data, channels, samples_rate, samples_per_second, regions}
    }
}

impl Iterator for FileSampler {
    type Item = SamplesBuffer<i16>;

    #[inline]
    fn next(&mut self) -> Option<SamplesBuffer<i16>> {
        Some(
            if Range::new(0, 5).ind_sample(&mut thread_rng()) == 0 {
                let region = thread_rng().choose(&self.regions).unwrap();
                let start_offset = (self.samples_per_second * region[0] as f32) as usize;
                let end_offset = (self.samples_per_second * region[1] as f32) as usize;
                let samples = self.data[start_offset..end_offset].to_vec();
                SamplesBuffer::new(self.channels, self.samples_rate, samples)
            }else{
                SamplesBuffer::new(1, 48000, vec![0; 48000])
                // this would be more efficient as something like this, but I can't work out the generic return
                // type for this function to return different kinds of Sources:
                // Zero::new(1, 48000).take_duration(Duration::from_millis(1000))
            }
        )
    }
}
