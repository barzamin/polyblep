extern crate cpal;
extern crate polyblep;

use polyblep::{Oscillator, Wave};

fn main() {
    use cpal::{EventLoop, StreamData, UnknownTypeOutputBuffer};

    let event_loop = EventLoop::new();
    let device = cpal::default_output_device().expect("no output device available");
    let mut supported_formats_range = device
        .supported_output_formats()
        .expect("error while querying formats");
    let format = supported_formats_range
        .next()
        .expect("no supported format?!")
        .with_max_sample_rate();

    let mut osc = Oscillator::new(format.sample_rate.0 as f64, 440., Wave::Sawtooth);

    println!("format: {:?}", format);

    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id);
    event_loop.run(move |_stream_id, stream_data| match stream_data {
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = ((osc.next_sample() + 1.) / 2. * std::u16::MAX as f64) as u16;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = (osc.next_sample() * std::i16::MAX as f64) as i16;
            }
        }
        StreamData::Output {
            buffer: UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            for elem in buffer.iter_mut() {
                *elem = osc.next_sample() as f32;
            }
        }
        _ => (),
    });
}
