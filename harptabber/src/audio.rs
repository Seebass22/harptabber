use rodio::source::{SineWave, Source};
use rodio::{OutputStream, Sink};
use std::time::Duration;

pub fn play(index: i32) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Add a dummy source of the sake of the example.
    let freq = f32::powf(2.0, index as f32 / 12.0) * 440.0;
    let source = SineWave::new(freq as u32)
        .take_duration(Duration::from_secs_f32(0.3))
        .amplify(0.20);
    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}
