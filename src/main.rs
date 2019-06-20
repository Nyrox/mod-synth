#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cpal;
#[macro_use]
extern crate lazy_static;

use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, RenderTarget, RenderWindow, Shape, Text,
    Transformable,
};
use sfml::system::{Clock, Time, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style};

use std::sync::Arc;
use std::sync::Mutex;

mod synth;
use synth::*;

lazy_static! {
    static ref graph: Arc<Mutex<Graph>> = Arc::new(Mutex::new(Graph::new()));
}

fn main() {
    //Make output node
    let out = Sum {};
    let out_index = graph.lock().unwrap().insert(Box::new(out));

    //Make wave gen node, hook up
    let wavegen = WaveGenerator {
        freq: 440.0,
        offset: 0.0,
        wave_type: WaveType::Sawtooth,
    };
    let wavegen_index = graph.lock().unwrap().insert(Box::new(wavegen));
    graph.lock().unwrap().get_mut(out_index).inputs.push(wavegen_index);

    setup_sound(out_index);
    sfml_loop();
}

fn sfml_loop() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Modular Synth",
        Style::CLOSE,
        &Default::default(),
    );
    window.set_vertical_sync_enabled(true);
    let mut running = true;
    let mut clock = Clock::start();

    let mut ui_root = UI::new();
    let mut x = 0.0;
    let mut graph_len = graph.lock().unwrap().len();
    
    for i in 0..graph_len {
        ui_root.node_indexes.insert(i, UINode::new(x, 20.0));
        x += 120.0;
    } 

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::MouseButtonPressed { button, x, y } => {}
                _ => {}
            }
        }
        if running {
            let dt = clock.restart().as_seconds();
        }
        window.clear(&Color::BLACK);
        if running {
            window.draw(&ui_root);
        }
        window.display()
    }
}

fn setup_sound(out_index: usize) {
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

    //Get 1 sample recursively
    let mut next_value = (move || {
        move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            let sample_position = sample_clock / sample_rate;

            let context = SamplingContext {
                clock: sample_position,
                sample_rate,
            };

            let out = graph.lock().unwrap().eval_node(&context, out_index);

            out.min(1.0).max(-1.0)
        }
    })();

    //Boilerplate shit
    std::thread::spawn(move || {
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
    });
}
