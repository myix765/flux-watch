mod outline;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ttf_parser::Face;
use web_sys::console;
use outline::{PathCommand, BezierCollector, PathCommandJS, GlyphLayout};

macro_rules! log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into())
    }
}

const FONT_DATA: &[u8] = include_bytes!("../assets/Anta/Anta-Regular.ttf");

fn extract_commands(face: &Face, ch: char, scale_x: f32, scale_y: f32) -> Vec<PathCommandJS> {
    let glyph_id = face.glyph_index(ch).expect(&format!("Missing glyph {}", ch));
    let mut collector = BezierCollector { scale_x, scale_y, commands: vec![] };
    face.outline_glyph(glyph_id, &mut collector);
    collector.commands.iter().map(|cmd| {
        match cmd {
            PathCommand::MoveTo(x, y) => PathCommandJS {
                kind: "MoveTo".into(),
                x: *x, y: *y,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
            PathCommand::LineTo(x, y) => PathCommandJS { kind: "LineTo".into(), x: *x, y: *y, cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0 },
            PathCommand::QuadTo(cx1, cy1, x, y) => PathCommandJS { kind: "QuadTo".into(), x: *x, y: *y, cx1: *cx1, cy1: *cy1, cx2: 0.0, cy2: 0.0 },
            PathCommand::CubicTo(cx1, cy1, cx2, cy2, x, y) => PathCommandJS { kind: "CubicTo".into(), x: *x, y: *y, cx1: *cx1, cy1: *cy1, cx2: *cx2, cy2: *cy2 },
            PathCommand::Close => PathCommandJS { kind: "Close".into(), x: 0.0, y: 0.0, cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0 },
        }
    }).collect()
}

fn get_glyph_bbox(face: &Face, ch: char) -> (f32, f32, f32, f32) {
    if let Some(glyph_id) = face.glyph_index(ch) {
        if let Some(bbox) = face.glyph_bounding_box(glyph_id) {
            return (
                bbox.x_min as f32,
                bbox.y_min as f32,
                bbox.x_max as f32,
                bbox.y_max as f32,
            );
        }
    }
    (0.0, 0.0, 0.0, 0.0)
}
#[wasm_bindgen]
pub fn get_time_layout(hour: u32, minutes: u32, width: f32, height: f32, seed: u32) -> JsValue {
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");

    let hour_tens   = char::from_digit(hour / 10, 10).unwrap();
    let hour_ones   = char::from_digit(hour % 10, 10).unwrap();
    let minute_tens = char::from_digit(minutes / 10, 10).unwrap();
    let minute_ones = char::from_digit(minutes % 10, 10).unwrap();

    let stroke_width = 2.0;
    let cell_w = (width - stroke_width) / 2.0;
    let cell_h = (height - stroke_width) / 2.0;

    let chars = [hour_tens, hour_ones, minute_tens, minute_ones];

    let layout: Vec<GlyphLayout> = chars.iter().enumerate().map(|(i, ch)| {
        let (x_min, y_min, x_max, y_max) = get_glyph_bbox(&face, *ch);

        let font_w = x_max - x_min;
        let font_h = y_max - y_min;

        let scale_x = cell_w / font_w;
        let scale_y = cell_h / font_h;

        let col = (i % 2) as f32;
        let row = (i / 2) as f32;

        let x_offset = col * cell_w - x_min * scale_x;
        let y_offset = row * cell_h + cell_h - y_min * scale_y;

        GlyphLayout {
            digit: ch.to_string(),
            commands: extract_commands(&face, *ch, scale_x, scale_y),
            x_offset,
            y_offset,
        }
    }).collect();

    serde_wasm_bindgen::to_value(&layout).unwrap()
}