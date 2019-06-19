use super::waves::*;

use midir::MidiInput;
use midir::MidiInputConnection;

use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone, Copy)]
pub struct SamplingContext {
    pub clock: f32,
    pub sample_rate: f32,
}

pub trait Node: Send {
    fn sample(&self, ctx: &SamplingContext) -> f32;
}

#[derive(Clone, Copy)]
pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle,
}

pub struct WaveGenerator {
    pub freq: f32,
    pub offset: f32,
    pub wave_type: WaveType,
}

impl Node for WaveGenerator {
    fn sample(&self, ctx: &SamplingContext) -> f32 {
        match self.wave_type {
            WaveType::Sine => sine_wave(self.freq, ctx.clock, self.offset),
            WaveType::Square => square_wave(self.freq, ctx.clock, self.offset),
            WaveType::Sawtooth => sawtooth_wave(self.freq, ctx.clock, self.offset),
            WaveType::Triangle => triangle_wave(self.freq, ctx.clock, self.offset),
        }
    }
}

pub struct Sum {
    pub nodes: Vec<Box<dyn Node>>,
}

impl Node for Sum {
    fn sample(&self, ctx: &SamplingContext) -> f32 {
        self.nodes.iter().map(|n| n.sample(ctx)).sum()
    }
}

/*pub struct MidiInputOscillator {
    pub wavetype: WaveType,
    input_device: usize,
    midi_channel: u8,
    note_buffer: Arc<Mutex<[u8; 128]>>,
    connection: MidiInputConnection<()>,
}

impl Node for MidiInputOscillator {
    fn sample(&self, ctx: &SamplingContext) -> f32 {
        let mut output = 0.0;

        for (pitch, velocity) in self
            .note_buffer
            .lock()
            .expect("failed to lock note_buffer")
            .iter()
            .enumerate()
        {
            if *velocity == 0 {
                continue;
            }

            let a4: i8 = 60;
            let a4_freq = 440.0;
            let dpitch = pitch as i8 - a4;

            let freq = 2.0f32.powf((dpitch as f32 / 12.0).into()) * a4_freq;

            output = output
                + WaveGenerator {
                    wave_type: self.wavetype,
                    offset: 0.0,
                    freq,
                }
                .sample(ctx);
        }

        output / 5.0
    }
}

impl MidiInputOscillator {
    pub fn new(wavetype: WaveType, input_device: usize, midi_channel: u8) -> Self {
        let midi_in = MidiInput::new("midir reading input").expect("Failed to create midi input");

        let note_buffer = Arc::new(Mutex::new([0; 128]));

        //
        let _note_buffer = note_buffer.clone();
        let _conn_in = midi_in
            .connect(
                input_device,
                "midir-read-input",
                move |_stamp, message, _| {
                    let status = message[0];

                    let _channel = status & 0b00001111;

                    let p_type = status & 0b11110000;

                    match p_type {
                        // NOTE_ ON
                        0b10010000 => {
                            let pitch = message[1] & 0b01111111;
                            let velocity = message[2] & 0b01111111;

                            println!("Turning note: {} on to velocity {}", pitch, velocity);

                            _note_buffer.lock().expect("failed to lock note_buffer")
                                [pitch as usize] = velocity;
                        }
                        0b10000000 => {
                            let pitch = message[1] & 0b01111111;

                            println!("Turning note: {} off", pitch);
                            _note_buffer.lock().expect("failed to lock note_buffer")
                                [pitch as usize] = 0;
                        }
                        _ => {
                            return;
                        }
                    }
                },
                (),
            )
            .expect("failed to create midi listening connection");

        MidiInputOscillator {
            wavetype,
            input_device,
            midi_channel,
            note_buffer: note_buffer.clone(),
            connection: _conn_in,
        }
    }

    pub fn reset(&mut self) {
        for n in self
            .note_buffer
            .lock()
            .expect("failed to lock note_buffer")
            .iter_mut()
        {
            *n = 0;
        }
    }
}*/
