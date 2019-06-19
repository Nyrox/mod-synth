#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

extern crate cpal;

use sfml::graphics::{
    CircleShape, Color, Font, RectangleShape, 
    RenderTarget, RenderWindow, Shape, Text, Transformable,
};
use sfml::system::{Clock, Time, Vector2f};
use sfml::window::{ContextSettings, Event, Key, Style};

use std::sync::Arc;
use std::sync::Mutex;

mod synth;
use synth::nodes::*;
use synth::ui::*;

fn main() {
    setup_sound();
    sfml_loop();
}

fn sfml_loop() {
    //FIXME: tree defined globally 1 place
    let tree = WaveGenerator {
        wave_type: WaveType::Sawtooth,
        freq: 440.0,
        offset: 0.0
    };

    let ui_root = UINode::new(50.0, 50.0, Box::new(tree));

    let mut window = RenderWindow::new(
        (800, 600),
        "Modular Synth",
        Style::CLOSE,
        &Default::default()
    );
    window.set_vertical_sync_enabled(true);
    let mut running = true;
    let mut clock = Clock::start();

    loop {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => return,
                Event::MouseButtonPressed { button, x, y } => {
                }
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

fn setup_sound() {
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

    let tree = WaveGenerator {
        wave_type: WaveType::Sawtooth,
        freq: 440.0,
        offset: 0.0
    };

    let tree = Mutex::new(tree);
    let tree = Arc::new(tree);

    //Get 1 sample recursively
    let mut next_value = (move |out_node: Arc<Mutex<dyn Node>>| {
        move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            let sample_position = sample_clock / sample_rate;

            let context = SamplingContext {
                clock: sample_position,
                sample_rate,
            };

            let out = out_node.lock().unwrap().sample(&context);

            //println!("{}", out.min(1.0).max(-1.0));

            out.min(1.0).max(-1.0)
        }
    })(tree);

    //Boilerplate shit
    std::thread::spawn(move || { 
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
    });
}
