
extern crate cpal;

const PI: f32 = std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct SamplingContext {
    pub clock: f32,
    pub sample_rate: f32,
}

pub trait Node: Send {
    fn sample (&self, ctx: &SamplingContext) -> f32;
}


struct Sinoid {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Sinoid {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        ((ctx.clock * self.freq + self.offset) * PI * 2.0).sin()
    }
}

struct Square {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Square {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        if ((ctx.clock * self.freq + self.offset) * PI * 2.0).sin() > 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}

struct Sum {
    pub nodes: Vec<Box<dyn Node>>,
}

impl Node for Sum {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        self.nodes.iter().map (|n| n.sample(ctx)).sum()
    }
}

use std::sync::Arc;
use std::sync::Mutex;

fn main() {
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;

    
    let tree = Sum {
        nodes: vec![
            Box::new(Sinoid {
                freq: 440.0,
                offset: 0.0
            }),
            Box::new(Square {
                freq: 440.0,
                offset: 0.5
            }),
        ]
    };

    let tree = Mutex::new (tree);
    let tree = Arc::new (tree);
    
    let mut next_value = (move |out_node: Arc<Mutex<dyn Node>>| {
        move || {
            sample_clock = ((sample_clock + 1.0) % sample_rate);
            let sample_position = sample_clock / sample_rate;

            let context = SamplingContext {
                clock: sample_position,
                sample_rate
            };
                     
            let out = out_node.lock().unwrap().sample (&context);
            println!("{}", out.min(1.0).max(-1.0)   );
            out.min(1.0).max(-1.0)   
        }
    }) (tree);














    event_loop.run(move |_, data| {
        match data {
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            },
            _ => (),
        }
    });
}
