use super::graph::GraphNode;
use super::nodes::*;
use sfml::graphics::{
    CircleShape, Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, RenderWindow,
    Shape, Text, Transformable,
};
use sfml::system::{Clock, Time, Vector2f};
use std::collections::HashMap;

pub struct UI<'s> {
    pub nodes: Vec<UINode<'s>>,

    dragging: bool,
    dragging_index: usize,
    dragging_anchor: Vector2f,
}

impl<'s> Drawable for UI<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        rt: &mut RenderTarget,
        rs: RenderStates,
    ) {
        for i in self.nodes.iter() {
            rt.draw(i);
        }
    }
}

impl<'s> UI<'s> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),

            dragging: false,
            dragging_index: 0,
            dragging_anchor: Vector2f::new(0.0, 0.0),
        }
    }
}

pub struct UINode<'s> {
    pub node_index: usize,

    pub x: f32,
    pub y: f32,

    title_bar: RectangleShape<'s>,
    bg: RectangleShape<'s>,
}

impl<'s> Drawable for UINode<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        rt: &mut RenderTarget,
        rs: RenderStates,
    ) {
        rt.draw(&self.bg);
        rt.draw(&self.title_bar);
    }
}

impl<'s> UINode<'s> {
    pub fn new(node_index: usize, x: f32, y: f32) -> UINode<'s> {
        let mut bg = RectangleShape::new();
        bg.set_size(Vector2f::new(100.0, 100.0));
        bg.set_origin(Vector2f::new(0.0, 0.0));
        bg.set_position(Vector2f::new(x, y));
        bg.set_fill_color(&Color::GREEN);

        let mut title_bar = RectangleShape::new();
        title_bar.set_size(Vector2f::new(100.0, 20.0));
        title_bar.set_origin(Vector2f::new(0.0, 0.0));
        title_bar.set_position(Vector2f::new(x, y));
        title_bar.set_fill_color(&Color::BLUE);

        UINode {
            node_index: node_index,

            x: x,
            y: y,

            title_bar: title_bar,
            bg: bg,
        }
    }
}

pub trait NodeMetaData {
    fn get_name(&self) -> &'static str;
    fn get_inputs(&self) -> Vec<&'static str>;
    fn get_outputs(&self) -> Vec<&'static str>;
}

impl NodeMetaData for WaveGenerator {
    fn get_name(&self) -> &'static str {
        match self.wave_type {
            WaveType::Sine => "Sine wave",
            WaveType::Square => "Square wave",
            WaveType::Sawtooth => "Sawtooth wave",
            WaveType::Triangle => "Triangle wave",
        }
    }
    fn get_inputs(&self) -> Vec<&'static str> {
        vec!["freq"]
    }
    fn get_outputs(&self) -> Vec<&'static str> {
        vec!["out"]
    }
}
