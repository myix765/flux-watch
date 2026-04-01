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

fn extract_commands(face: &Face, ch: char, scale: f32) -> Vec<PathCommandJS> {
    let glyph_id = face.glyph_index(ch).expect(&format!("Missing glyph {}", ch));
    let mut collector = BezierCollector { scale, commands: vec![] };
    face.outline_glyph(glyph_id, &mut collector);
    collector.commands.iter().map(|cmd| {
        match cmd {
            PathCommand::MoveTo(x, y) => PathCommandJS { 
                kind: "MoveTo".into(), x: *x, y: *y,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
            PathCommand::LineTo(x, y) => PathCommandJS { 
                kind: "LineTo".into(), x: *x, y: *y,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
            PathCommand::QuadTo(cx1, cy1, x, y) => PathCommandJS { 
                kind: "QuadTo".into(), x: *x, y: *y,
                cx1: *cx1, cy1: *cy1, cx2: 0.0, cy2: 0.0
            },
            PathCommand::CubicTo(cx1, cy1, cx2, cy2, x, y) => PathCommandJS { 
                kind: "CubicTo".into(), x: *x, y: *y,
                cx1: *cx1, cy1: *cy1, cx2: *cx2, cy2: *cy2
            },
            PathCommand::Close => PathCommandJS { 
                kind: "Close".into(), x: 0.0, y: 0.0,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
        }
    }).collect()
}

#[wasm_bindgen]
pub fn get_glyph_outline(d: char, target_height: f32) -> JsValue {
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");
    let scale = target_height / face.units_per_em() as f32;
    serde_wasm_bindgen::to_value(&extract_commands(&face, d, scale)).unwrap()
}

#[wasm_bindgen]
pub fn get_time_layout(hour: u32, minute: u32, target_height: f32, seed: u32) -> JsValue {
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");
    let scale = target_height / face.units_per_em() as f32;

    let digits = [
        (char::from_digit(hour / 10, 10).unwrap(), 0.0_f32, 0.0_f32),
        (char::from_digit(hour % 10, 10).unwrap(), 50.0_f32, 0.0_f32),
        (char::from_digit(minute / 10, 10).unwrap(), 0.0_f32, 100.0_f32),
        (char::from_digit(minute % 10, 10).unwrap(), 50.0_f32, 100.0_f32),
    ];

    let layout: Vec<GlyphLayout> = digits.iter().map(|(ch, x, y)| {
        GlyphLayout {
            digit: ch.to_string(),
            commands: extract_commands(&face, *ch, scale),
            x_offset: *x,
            y_offset: *y,
        }
    }).collect();

    serde_wasm_bindgen::to_value(&layout).unwrap()
}