extern crate rodio;
mod random_tone;

use rodio::Sink;
use rodio::Endpoint;
use rodio::Source;
use rodio::Sample;
use std::time::Duration;
use std::thread;
use random_tone::RandomTone;

fn sink_for_source<S>(endpoint: &Endpoint, source: S) -> Sink
        //borrowed this "where" from the defintion of sink.append -- don't understand what it does yet :)
        where S: Source + Send + 'static,
              S::Item: Sample,
              S::Item: Send{
    let sink = Sink::new(&endpoint);
    sink.append(source);
    sink
}

fn main() {
    let endpoint = rodio::get_default_endpoint().unwrap();

    let freqs = [523.25, 587.33, 880.00, 659.25, 783.99];  // C, D, E, G, A

    let sources = (0..5).map(|_| RandomTone::new(freqs.to_vec())).collect::<Vec<_>>();
    let _sinks = sources.into_iter().map(|x| sink_for_source(&endpoint, x)).collect::<Vec<Sink>>();

    thread::sleep(Duration::from_secs(600));
}
