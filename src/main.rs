extern crate rodio;
extern crate rand;

mod random_tone;
mod file_sampler;
mod tone_pattern;

use rodio::{Sink, Endpoint, Source, Sample};
use rodio::source::{Buffered, SineWave, from_iter};
use std::env;
use random_tone::RandomTone;
use file_sampler::FileSampler;
use tone_pattern::TonePattern;

fn sink_for_source<S>(endpoint: &Endpoint, source: S, volume: f32) -> Sink
        //borrowed this "where" from the defintion of sink.append -- don't understand what it does yet :)
        where S: Source + Send + 'static,
              S::Item: Sample,
              S::Item: Send{
    let mut sink = Sink::new(&endpoint);
    sink.append(source);
    sink.set_volume(volume);
    sink
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let inputs = args[1..].iter().map(|arg| arg.parse::<f32>().unwrap()).collect::<Vec<f32>>();
//    let snippet_limit = args[1].parse::<usize>().unwrap();
//    let threshold = args[2].parse::<f32>().unwrap();
//    let window_length = args[3].parse::<usize>().unwrap();

    let endpoint = rodio::get_default_endpoint().unwrap();

    let mut sinks = Vec::new();
    let freqs = [523.25, 587.33, 880.00, 659.25, 783.99];  // C, D, E, G, A

    // tone sources
    for _ in 0..3 as u32{
        sinks.push(
            sink_for_source(&endpoint,
                            RandomTone::new(freqs.to_vec(), inputs[0] as i32),
                            0.01));
    }

    for i in 0..5 as u32{
        let freq_offset = 1 * i;
        let time_offset = 101 * i;
        sinks.push(
                sink_for_source(&endpoint,
                                rodio::source::from_iter(
                                    TonePattern::new(vec![
                                                            vec![55+freq_offset, 1000+time_offset],
                                                            vec![110+freq_offset, 1000+time_offset],
                                                            vec![220+freq_offset, 500+time_offset],
                                                        ]),
                                ),
                                0.1));
    }

    // split file
    let sampler = FileSampler::new("assets/OWH_01.ogg",
                                       vec![
                                                    vec![60.0, 62.0],  // sit in silence
                                                    vec![65.0, 67.0],  // as the end draws near
                                                    vec![86.0, 91.0],  // there is time to hear the kind voice of friends
                                                    vec![137.5, 141.0] // live, I am coming
                                                    ]);
    sinks.push(
        sink_for_source(&endpoint,
                        rodio::source::from_iter(sampler),
                        2.0));

    sinks[0].sleep_until_end();
    sinks[sinks.len()-1].sleep_until_end();
}
