use super::nodes::*;
use sfml::graphics::{
    CircleShape, Color, Drawable, Font, RectangleShape, RenderStates, RenderTarget, RenderWindow,
    Shape, Text, Transformable,
};
use sfml::system::{Clock, Time, Vector2f};

pub struct UINode<'s> {
    pub x: f32,
    pub y: f32,
    pub node: Box<Node>,

    bg: RectangleShape<'s>,
}

impl<'s> Drawable for UINode<'s> {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        rt: &mut RenderTarget,
        rs: RenderStates,
    ) {
        rt.draw(&self.bg);
    }
}

impl<'s> UINode<'s> {
    pub fn new(x: f32, y: f32, node: Box<Node>) -> UINode<'s> {
        let mut bg = RectangleShape::new();
        bg.set_size(Vector2f::new(100.0, 100.0));
        bg.set_origin(Vector2f::new(0.0, 0.0));
        bg.set_position(Vector2f::new(x, y));
        bg.set_fill_color(&Color::GREEN);

        UINode {
            x: x,
            y: y,
            node: node,

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
