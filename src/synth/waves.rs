use super::constants::*;

pub fn sine_wave(freq: f32, t: f32, offset: f32) -> f32 {
    ((t * freq + offset) * PI * 2.0).sin()
}

pub fn square_wave(freq: f32, t: f32, offset: f32) -> f32 {
    sine_wave(freq, t, offset).signum()
}

pub fn sawtooth_wave(freq: f32, t: f32, offset: f32) -> f32 {
    (2.0 * ((t * freq + offset) - (t * freq + offset + 0.5).floor()))
}

pub fn triangle_wave(freq: f32, t: f32, offset: f32) -> f32 {
    sawtooth_wave(freq, t, offset).abs() * 2.0 - 1.0
}