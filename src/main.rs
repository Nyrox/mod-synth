extern crate cpal;

use std::sync::Arc;
use std::sync::Mutex;

mod synth;
use synth::*;

fn main() {
    //Setup cpal
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device
        .default_output_format()
        .expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    //Make output node
    let mut graph = Graph::new();
    let out = Sum { };
    let out_index = graph.insert(Box::new(out));

    //Make wave gen node, hook up
    let wavegen = WaveGenerator {
        freq: 440.0,
        offset: 0.0,
        wave_type: WaveType::Sawtooth
    };
    let wavegen_index = graph.insert(Box::new(wavegen));
    graph.get_mut(out_index).inputs.push(wavegen_index);    

    let graph = Arc::new(Mutex::new(graph));

    //Get 1 sample recursively
    let mut next_value = (move |graph: Arc<Mutex<Graph>>, out_index: usize| {
        move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            let sample_position = sample_clock / sample_rate;

            let context = SamplingContext {
                clock: sample_position,
                sample_rate,
            };

            let out = graph.lock().unwrap().eval_node(&context, out_index);

            //println!("{}", out.min(1.0).max(-1.0));

            out.min(1.0).max(-1.0)
        }
    })(graph, out_index);

    //Boilerplate shit
    event_loop.run(move |_, data| match data {
        cpal::StreamData::Output {
            buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
        } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
                let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                for out in sample.iter_mut() {
                    *out = value;
                }
            }
        }
        cpal::StreamData::Output {
            buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
        } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
                let value = (next_value() * std::i16::MAX as f32) as i16;
                for out in sample.iter_mut() {
                    *out = value;
                }
            }
        }
        cpal::StreamData::Output {
            buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
        } => {
            for sample in buffer.chunks_mut(format.channels as usize) {
                let value = next_value();
                for out in sample.iter_mut() {
                    *out = value;
                }
            }
        }
        _ => (),
    });
}
