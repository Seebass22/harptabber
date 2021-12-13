use rodio::source::{SineWave, Source};
use std::time::Duration;

pub fn play(index: i32, sink: &rodio::Sink) {
    let freq = f32::powf(2.0, index as f32 / 12.0) * 440.0;
    let source = SineWave::new(freq as u32)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    sink.append(source);
}
