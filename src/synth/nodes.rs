use super::constants::*;
use super::waves::*;

#[derive(Clone, Copy)]
pub struct SamplingContext {
    pub clock: f32,
    pub sample_rate: f32,
}

pub trait Node: Send {
    fn sample (&self, ctx: &SamplingContext) -> f32;
}

pub enum WaveType {
    Sine,
    Square,
    Sawtooth,
    Triangle
}

pub struct WaveGenerator {
    pub freq: f32,
    pub offset: f32,
    pub wave_type: WaveType
}

impl Node for WaveGenerator {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        match self.wave_type {
            WaveType::Sine => sine_wave(self.freq, ctx.clock, self.offset),
            WaveType::Square => square_wave(self.freq, ctx.clock, self.offset),
            WaveType::Sawtooth => sawtooth_wave(self.freq, ctx.clock, self.offset),
            WaveType::Triangle => triangle_wave(self.freq, ctx.clock, self.offset)
        }   
    }
}

pub struct Sum {
    pub nodes: Vec<Box<dyn Node>>,
}

impl Node for Sum {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        self.nodes.iter().map (|n| n.sample(ctx)).sum()
    }
}