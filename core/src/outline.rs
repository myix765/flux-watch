use ttf_parser::OutlineBuilder;
use serde::Serialize;
use sdfer::{Image2d, Unorm8};

pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadTo(f32, f32, f32, f32),
    CubicTo(f32, f32, f32, f32, f32, f32),
    Close,
}

pub struct BezierCollector {
    pub commands: Vec<PathCommand>,
}

pub struct GlyphData {
    pub ch: char,
    pub commands: Vec<PathCommand>,
    pub sdf: Image2d<Unorm8>,
    pub bbox: (f32, f32, f32, f32),
}

impl OutlineBuilder for BezierCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo(x, y));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo(x, y));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::QuadTo(x1, y1, x, y));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::CubicTo(x1, y1, x2, y2, x, y));
    }
    fn close(&mut self) {
        self.commands.push(PathCommand::Close);
    }
}

#[derive(Serialize)]
pub struct PathCommandJS {
    pub kind: String,      // "MoveTo", "LineTo", "QuadTo", "CubicTo", "Close"
    pub x: f32,
    pub y: f32,
    pub cx1: f32,          // control point 1 (only used for quad/cubic)
    pub cy1: f32,
    pub cx2: f32,          // control point 2 (only used for cubic)
    pub cy2: f32,
}

#[derive(Serialize)]
pub struct GlyphLayout {
    pub digit: String,
    pub commands: Vec<PathCommandJS>,
    pub x_offset: f32,
    pub y_offset: f32,
}