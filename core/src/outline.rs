use ttf_parser::OutlineBuilder;
use serde::Serialize;

pub enum PathCommand {
    MoveTo(f32, f32),
    LineTo(f32, f32),
    QuadTo(f32, f32, f32, f32),
    CubicTo(f32, f32, f32, f32, f32, f32),
    Close,
}

pub struct BezierCollector {
    pub scale_x: f32,
    pub scale_y: f32,
    pub commands: Vec<PathCommand>,
}

// flip y values for compatibility with pixijs axis system
impl OutlineBuilder for BezierCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::MoveTo(
            x * self.scale_x,
            -y * self.scale_y
        ));
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.commands.push(PathCommand::LineTo(
            x * self.scale_x,
            -y * self.scale_y
        ));
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::QuadTo(
            x1 * self.scale_x, -y1 * self.scale_y,
            x * self.scale_x,  -y * self.scale_y
        ));
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.commands.push(PathCommand::CubicTo(
            x1 * self.scale_x, -y1 * self.scale_y,
            x2 * self.scale_x, -y2 * self.scale_y,
            x * self.scale_x,  -y * self.scale_y
        ));
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