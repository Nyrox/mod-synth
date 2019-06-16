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

struct Sinoid {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Sinoid {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        sine_wave(self.freq, ctx.clock, self.offset)
    }
}

pub struct Square {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Square {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        square_wave(self.freq, ctx.clock, self.offset)
    }
}

pub struct Sawtooth {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Sawtooth {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        sawtooth_wave(self.freq, ctx.clock, self.offset)
    }
}

pub struct Triangle {
    pub freq: f32,
    pub offset: f32,
}

impl Node for Triangle {
    fn sample (&self, ctx: &SamplingContext) -> f32 {
        triangle_wave(self.freq, ctx.clock, self.offset)
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