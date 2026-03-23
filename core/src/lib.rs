mod outline;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ttf_parser::Face;
use web_sys::console;
use outline::{PathCommand, BezierCollector};
use serde::Serialize;

macro_rules! log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into())
    }
}

const FONT_DATA: &[u8] = include_bytes!("../assets/Anta/Anta-Regular.ttf");

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

#[wasm_bindgen]
pub fn get_glyph_outline(d: char, target_height: f32) -> JsValue {
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");

    let glyph_id = face.glyph_index(d).expect(format!("Font is missing digit {}", d).as_str());
    log!("Glyph ID: {:?}", glyph_id);
    let b_box = face.glyph_bounding_box(glyph_id).expect("Failed to get bounding box");
    log!("Bounding box: {:?}", b_box);
    let units_per_em = face.units_per_em() as f32;
    let scale = target_height / units_per_em;
    let mut collector = BezierCollector { scale, commands: vec![] };
    face.outline_glyph(glyph_id, &mut collector).unwrap();

    let js_commands: Vec<PathCommandJS> = collector.commands.iter().map(|cmd| {
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
    }).collect();

    serde_wasm_bindgen::to_value(&js_commands).unwrap()
}