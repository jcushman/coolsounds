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
