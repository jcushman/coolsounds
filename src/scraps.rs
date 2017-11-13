// Stuff that was hard to write and then turned out to be wrong ...


//*** play audio file
// voice source
let file = std::fs::File::open("assets/OWH_01.ogg").unwrap();
let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
let mut sink1 = Sink::new(&endpoint);
sink1.append(source);
sinks.push( sink1);


//*** split and play


//    let window_length = 1000;
    let mut window = source.by_ref().take(window_length).map(sample_to_db).collect::<Vec<f32>>();
    let mut running_total = window.iter().fold(0.0,(|sum,value| sum + *value));
//    let threshold = 10.0;
    let mut found_noise = false;
    let mut snippets = Vec::new();
    let mut snippet = Vec::new();
//    window = source.take(window_length);
    for i in source.by_ref() {
        let db = sample_to_db(i);
//        println!("{} {} {} {}", i, sample_to_db(i), running_total/(window_length as f32), threshold);
        snippet.push(i);
        window.push(db);

        running_total += db;
        running_total -=  window.remove(0);
        if running_total / (window_length as f32) < threshold {
            if found_noise {
                println!("{}: {}", snippets.len(), snippet.len());
                snippets.push(snippet);
                snippet = Vec::new();
                found_noise = false;
                if snippets.len() > snippet_limit {
                    break;
                }
            }
        } else if !found_noise{
            found_noise = true;
        }
    }
    snippets.push(snippet);

//    println!("Playing sound 0 at sample rate {} - {}", source.samples_rate(), source.channels());
    let snip_sink = Sink::new(&endpoint);
    for s in snippets{
        if s.len() < (source.samples_rate() as usize)/2 {
            continue;
        }
        println!("Playing sound");
        let snip_source = SamplesBuffer::new(source.channels(), source.samples_rate(), s.to_vec());
        snip_sink.append(snip_source);
        let sine_source = SineWave::new(220).take_duration(Duration::from_millis(100));
        snip_sink.append(sine_source);
    }
    sinks.push(snip_sink);



fn sample_to_db(sample: i16) -> f32{
    if sample == 0 {
        return 0.0;
    }
    (sample.abs() as f32 / i16::max_value() as f32).log10() * 20.0
}






// atttempt to implement what turned out to already exist as DynamicMixer ...

use std::cmp;
use std::time::Duration;

use rodio::source::UniformSourceIterator;

use rodio::Sample;
use rodio::Source;

/// Internal function that builds a `TanhMix` object.
pub fn tanh_mix(inputs: Vec<Box<Source<Item=f32>>>) -> TanhMix
{
    let channels = inputs[0].channels();
    let rate = inputs[0].samples_rate();

    TanhMix {
        inputs: inputs.iter().map(|i| UniformSourceIterator::new(*i, channels, rate)).collect()
    }
}

//#[derive(Clone)]
pub struct TanhMix {
    inputs: Vec
        <UniformSourceIterator
            <Box
                <Source<Item=f32>>,
            f32>>
}

impl Iterator for TanhMix {
    type Item = f32;

    #[inline]
    fn next(&mut self) -> Option<f32> {
        let mut sum = None;
        for mut input in self.inputs {
            match input.next() {
                Some(x) =>
                    if sum.is_some(){
                        sum.map(|v:f32| v.saturating_add(x))
                    }else{
                        Some(x)
                    },
            };
        }
        sum.map(|v| v.tanh())
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let mut min = 0;
        let mut max = Some(0);
        for input in self.inputs {
            let hint = input.size_hint();
            min = min.max(hint.0);
            if max.is_some(){
                match hint.1 {
                    Some(x) =>
                        max.as_mut().map(|v| v.max(x)),
                    None => { max = None },
                };
            }
        }
        (min, max)
    }
}

impl ExactSizeIterator for TanhMix
{
}

impl Source for TanhMix {
    #[inline]
    fn current_frame_len(&self) -> Option<usize> {
        let mut result = Some(usize::max_value());
        for input in self.inputs {
            match input.current_frame_len() {
                Some(mut x) => result.as_mut().map(|v| v.min(&mut x)),
                _ => { result = None; break; },
            };
        }
        result
    }

    #[inline]
    fn channels(&self) -> u16 {
        self.inputs[0].channels()
    }

    #[inline]
    fn samples_rate(&self) -> u32 {
        self.inputs[0].samples_rate()
    }

    #[inline]
    fn total_duration(&self) -> Option<Duration> {
        let mut result = Some(Duration::from_millis(0));
        for input in self.inputs {
            match input.total_duration() {
                Some(mut x) => result.as_mut().map(|v| v.max(&mut x)),
                _ => { result = None; break; },
            };
        }
        result
    }
}
