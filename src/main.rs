
extern crate cpal;

use std::sync::Arc;
use std::sync::Mutex;

mod synth;
use synth::nodes::*;

use midir::{MidiInput, Ignore};

use std::io::{stdin, stdout, Write};
use std::error::Error;

fn run() -> Result<(), Box<Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_port = match midi_in.port_count() {
        0 => return Err("no input port found".into()),
        1 => {
            println!("Choosing the only available input port: {}", midi_in.port_name(0).unwrap());
            0
        },
        _ => {
            println!("\nAvailable input ports:");
            for i in 0..midi_in.port_count() {
                println!("{}: {}", i, midi_in.port_name(i).unwrap());
            }
            print!("Please select input port: ");
            stdout().flush()?;
            let mut input = String::new();
            stdin().read_line(&mut input)?;
            input.trim().parse()?
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
    let _conn_in = midi_in.connect(in_port, "midir-read-input", move |stamp, message, _| {
        let status = message[0];
        
        let ptype = status & 0b11110000;
        let channel = status & 0b00001111;

        

        println!("{}: {:?} (len = {}) | channel: {}, type: {}", stamp, message, message.len(), channel, ptype);
    }, ())?;

    println!("Connection open, reading input from '{}' (press enter to exit) ...", in_port_name);

    input.clear();
    stdin().read_line(&mut input)?; // wait for next enter key press

    println!("Closing connection");
    Ok(())
}

use midir::MidiInputConnection;

struct MidiInputOscillator {
    pub wavetype: WaveType,
    input_device: usize,
    midi_channel: u8,
    note_buffer: Arc<Mutex<[u8; 128]>>,
    connection: MidiInputConnection<()>,
}

impl Node for MidiInputOscillator {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        let mut output = 0.0;

        for (pitch, velocity) in self.note_buffer.lock().expect("failed to lock note_buffer").iter().enumerate() {
           if *velocity == 0 { continue;  }
            
           let a4: i8 = 60;
           let a4_freq = 440.0;
           let dpitch = pitch as i8 - a4;

           let freq = 2.0f32.powf ((dpitch as f32/12.0).into()) * a4_freq;

           output = output + WaveGenerator {
               wave_type: self.wavetype,
               offset: 0.0,
               freq
            }.sample (ctx);
        }

        output / 5.0
    }
}

impl MidiInputOscillator {
    pub fn new (wavetype: WaveType, input_device: usize, midi_channel: u8) -> Self {
        let mut midi_in = MidiInput::new("midir reading input").expect("Failed to create midi input");
    
        let note_buffer = Arc::new(Mutex::new([0; 128]));
        
        // 
        let _note_buffer = note_buffer.clone();
        let _conn_in = midi_in.connect (input_device, "midir-read-input", move |stamp, message, _| {
            let status = message[0];

            let channel = status & 0b00001111;
            

            let p_type = status & 0b11110000;


            match p_type {
                // NOTE_ ON
                0b10010000 => {
                    let pitch = message [1] & 0b01111111;
                    let velocity = message [2] & 0b01111111;
   
                    println! ("Turning note: {} on to velocity {}", pitch, velocity);

                    _note_buffer.lock().expect("failed to lock note_buffer")[pitch as usize] = velocity;
                },
                0b10000000 => {
                    let pitch = message [1] & 0b01111111;

                    println! ("Turning note: {} off", pitch);
                    _note_buffer.lock().expect("failed to lock note_buffer")[pitch as usize] = 0;
                },
                _ => { return; } 
            }

        }, ()).expect("failed to create midi listening connection");
       
        MidiInputOscillator {
            wavetype,
            input_device,
            midi_channel,
            note_buffer: note_buffer.clone(),
            connection: _conn_in,
        }
    }

    pub fn reset (&mut self) {
        for n in self.note_buffer.lock().expect("failed to lock note_buffer").iter_mut() {
            *n = 0;
        }
    }
}

fn main() {
    

    //Setup cpal
    let device = cpal::default_output_device().expect("Failed to get default output device");
    let format = device.default_output_format().expect("Failed to get default output format");
    let event_loop = cpal::EventLoop::new();
    let stream_id = event_loop.build_output_stream(&device, &format).unwrap();
    event_loop.play_stream(stream_id.clone());

    let sample_rate = format.sample_rate.0 as f32;
    let mut sample_clock = 0f32;
   
    /*let tree = Sum {
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
    };*/

    
    let tree = Sum {
        nodes: vec! [
            Box::new(WaveGenerator {
                wave_type: WaveType::Square,
                freq: 440.0,
                offset: 0.0,
            }),
            Box::new(WaveGenerator {
                wave_type: WaveType::Square,
                freq: 230.0,
                offset: 0.0,
            })
        ]
    };

    let tree = MidiInputOscillator::new (WaveType::Sawtooth, 1, 1);
    
    let tree = Mutex::new (tree);
    let tree = Arc::new (tree);
    
    //Get 1 sample recursively
    let mut next_value = (move |out_node: Arc<Mutex<dyn Node>>| {
        move || {
            sample_clock = (sample_clock + 1.0) % sample_rate;
            let sample_position = sample_clock / sample_rate;

            let context = SamplingContext {
                clock: sample_position,
                sample_rate
            };
                     
            let out = out_node.lock().unwrap().sample (&context);
            
            //println!("{}", out.min(1.0).max(-1.0));
            
            out.min(1.0).max(-1.0)   
        }
    }) (tree);

    //Boilerplate shit
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
