extern crate rodio;
extern crate rand;
extern crate hound;
#[macro_use]
extern crate clap;

mod random_tone;
mod file_sampler;
mod tone_pattern;
mod map_filter;

use rodio::{Sink, Endpoint, Source, Sample, dynamic_mixer};
use std::env;
use std::time::Duration;

use random_tone::RandomTone;
use file_sampler::FileSampler;
use tone_pattern::TonePattern;
use map_filter::Map;

// HELPERS

fn sink_for_source<S>(endpoint: &Endpoint, source: S) -> Sink
        where S: Source + Send + 'static,
              S::Item: Sample,
              S::Item: Send{
    let mut sink = Sink::new(&endpoint);
    sink.append(source);
    sink
}

// TODO: figure out the type signature for this ...
//fn soft_clip<S>(source: S) -> Map<S>
//    where S: Source,
//              S::Item: Sample,
//{
//    Map::new(source, |s: Sample, _| s.to_f32().tanh())
//}

// MAIN

fn main() {
    let args = clap_app!(coolsounds =>
        (@arg output_path: -o --output +takes_value "Output path. If ommitted, play to speakers.")
        (@arg output_seconds: -s --output-seconds +takes_value "Output length in seconds." )
        (@arg INPUTS: +multiple "Inputs to tweak sound")
    ).get_matches();

    // gather sources
    let mut sources:Vec<Box<Source<Item=f32> + Send>> = Vec::new();

    //*** treble -- chirpy pentatonic scale
    let freqs = [523.25, 587.33, 880.00, 659.25, 783.99];  // C, D, E, G, A
    for _ in 0..3 as u32{
        sources.push(Box::new(
            RandomTone::new(freqs.to_vec(), 48000 as i32).amplify(0.03),
        ));
    }

    //*** bass -- cycle through bass notes with varying overlaps
    let (mix_controller, mix) = dynamic_mixer::mixer(1, 48000);
    for i in 0..5 as u32{
        let freq_offset = 1 * i;
        let time_offset = 101 * i;
//        AmplifyByFunc::new(source, |n| (n as f32 % 48000.0) / 48000.0 + 0.5);
        mix_controller.add(
            rodio::source::from_iter(
                TonePattern::new(vec![
                    vec![55+freq_offset, 1000+time_offset],
                    vec![110+freq_offset, 1000+time_offset],
                    vec![220+freq_offset, 500+time_offset],
                ]),
            )
        );
    }

    // bass mix -- increase volume over 60 seconds. Get a nice audio spike every 60 seconds with output_vol = 1/1_000_000_000 * (secs % 60) ^ 4
    let vol_change = Map::new(mix, |s:f32, n| s * ((n as f32 % (48000.0*60.0)) / 48000.0).powi(6) / 1_000_000_000.0);
    // prevent clipping with tanh
    let soft_clip = Map::new(vol_change, |s: f32, _| s.tanh());
    sources.push(Box::new(soft_clip.amplify(0.1)));

    //*** lyrics -- from audio file
    let sampler = FileSampler::new("assets/OWH_01.ogg",
                                       vec![
                                                    vec![60.0, 62.0],  // sit in silence
                                                    vec![65.0, 67.0],  // as the end draws near
                                                    vec![86.0, 91.0],  // there is time to hear the kind voice of friends
                                                    vec![137.5, 141.0] // live, I am coming
                                                    ]);
    let sampler_source = rodio::source::from_iter(sampler);
    sources.push(Box::new(sampler_source.convert_samples().amplify(2.0)));


    //*** output -- WAV
    if let Some(output_path) = args.value_of("output_path"){
        println!("Writing sound output to {}", output_path);
        let (out_controller, out) = dynamic_mixer::mixer(2, 48000);
        for s in sources{
            out_controller.add(s);
        }
        let spec = hound::WavSpec {
            channels: 2,
            sample_rate: 48000,
            bits_per_sample: 32,
            sample_format: hound::SampleFormat::Float,
        };
        let mut writer = hound::WavWriter::create(output_path, spec).unwrap();
        for s in out.take_duration(Duration::from_secs(value_t!(args, "output_seconds", u64).unwrap_or(135))){
            writer.write_sample(s).unwrap();
        }
    //*** output -- play forever
    }else {
        println!("Playing forever ...");
        let endpoint = rodio::get_default_endpoint().unwrap();
        let mut sinks = Vec::new();
        for s in sources {
            let mut sink = Sink::new(&endpoint);
            sink.append(s);
            sinks.push(sink);
        }
        sinks[0].sleep_until_end();
    }
}

