mod outline;

use sdfer::{Image2d, Unorm8};
use sdfer::esdt;
use tiny_skia::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use ttf_parser::Face;
use web_sys::console;
use outline::{PathCommand, BezierCollector, PathCommandJS, GlyphLayout, GlyphData};

macro_rules! log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into())
    }
}

const FONT_DATA: &[u8] = include_bytes!("../assets/Anta/Anta-Regular.ttf");
const SDF_RESOLUTION: usize = 256;

fn extract_commands(face: &Face, ch: char) -> Vec<PathCommand> {
    let glyph_id = face.glyph_index(ch).expect(&format!("Missing glyph {}", ch));
    let mut collector = BezierCollector { commands: vec![] };
    face.outline_glyph(glyph_id, &mut collector);
    collector.commands
}

fn build_path(commands: &[PathCommand]) -> Option<Path> {
    let mut pb = PathBuilder::new();
    for cmd in commands {
        match cmd {
            PathCommand::MoveTo(x, y) => pb.move_to(*x, *y),
            PathCommand::LineTo(x, y) => pb.line_to(*x, *y),
            PathCommand::QuadTo(cx1, cy1, x, y) => pb.quad_to(*cx1, *cy1, *x, *y),
            PathCommand::CubicTo(cx1, cy1, cx2, cy2, x, y) => pb.cubic_to(*cx1, *cy1, *cx2, *cy2, *x, *y),
            PathCommand::Close => pb.close(),
        }
    }
    pb.finish()
}

fn to_js_commands(commands: &[PathCommand], scale_x: f32, scale_y: f32) -> Vec<PathCommandJS> {
    commands.iter().map(|cmd| {
        match cmd {
            PathCommand::MoveTo(x, y) => PathCommandJS {
                kind: "MoveTo".into(),
                x: x * scale_x, y: -y * scale_y,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
            PathCommand::LineTo(x, y) => PathCommandJS {
                kind: "LineTo".into(),
                x: x * scale_x, y: -y * scale_y,
                cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
            PathCommand::QuadTo(cx1, cy1, x, y) => PathCommandJS {
                kind: "QuadTo".into(),
                x: x * scale_x, y: -y * scale_y,
                cx1: cx1 * scale_x, cy1: -cy1 * scale_y,
                cx2: 0.0, cy2: 0.0
            },
            PathCommand::CubicTo(cx1, cy1, cx2, cy2, x, y) => PathCommandJS {
                kind: "CubicTo".into(),
                x: x * scale_x, y: -y * scale_y,
                cx1: cx1 * scale_x, cy1: -cy1 * scale_y,
                cx2: cx2 * scale_x, cy2: -cy2 * scale_y
            },
            PathCommand::Close => PathCommandJS {
                kind: "Close".into(),
                x: 0.0, y: 0.0, cx1: 0.0, cy1: 0.0, cx2: 0.0, cy2: 0.0
            },
        }
    }).collect()
}

fn get_glyph_bbox(face: &Face, ch: char) -> (f32, f32, f32, f32) {
    if let Some(glyph_id) = face.glyph_index(ch) {
        if let Some(bbox) = face.glyph_bounding_box(glyph_id) {
            return (bbox.x_min as f32, bbox.y_min as f32, bbox.x_max as f32, bbox.y_max as f32);
        }
    }
    (0.0, 0.0, 0.0, 0.0)
}

fn rasterize_glyph(commands: &[PathCommand], bbox: (f32, f32, f32, f32), pixmap: &mut Pixmap, paint: &Paint) -> Image2d<Unorm8> {
    let (x_min, y_min, x_max, y_max) = bbox;
    let font_w = x_max - x_min;
    let font_h = y_max - y_min;

    let sdf_scale_x = SDF_RESOLUTION as f32 / font_w;
    let sdf_scale_y = SDF_RESOLUTION as f32 / font_h;
    let transform = Transform::from_translate(-x_min, -y_min)
        .post_scale(sdf_scale_x, sdf_scale_y);

    pixmap.fill(Color::TRANSPARENT);
    if let Some(path) = build_path(commands) {
        pixmap.fill_path(&path, paint, FillRule::EvenOdd, transform, None);
    }

    let mut image: Image2d<Unorm8, Vec<Unorm8>> = Image2d::new(SDF_RESOLUTION, SDF_RESOLUTION);
    let pixels = pixmap.pixels();
    for y in 0..SDF_RESOLUTION {
        for x in 0..SDF_RESOLUTION {
            image[(x, y)] = Unorm8::from_bits(pixels[y * SDF_RESOLUTION + x].alpha());
        }
    }
    image
}

#[wasm_bindgen]
pub fn get_time_layout(hour: u32, minutes: u32, width: f32, height: f32, seed: u32) -> JsValue {
    console_error_panic_hook::set_once();
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");

    let chars = [
        char::from_digit(hour / 10, 10).unwrap(),
        char::from_digit(hour % 10, 10).unwrap(),
        char::from_digit(minutes / 10, 10).unwrap(),
        char::from_digit(minutes % 10, 10).unwrap(),
    ];

    // pass 1 — rasterize each glyph and compute SDF
    let mut pixmap = Pixmap::new(SDF_RESOLUTION as u32, SDF_RESOLUTION as u32).unwrap();
    let paint = Paint::default();
    let mut reuse_bufs = None;

    let glyph_data: Vec<GlyphData> = chars.iter().map(|ch| {
        let commands = extract_commands(&face, *ch);
        let bbox = get_glyph_bbox(&face, *ch);
        let mut image = rasterize_glyph(&commands, bbox, &mut pixmap, &paint);
        let (sdf, bufs) = esdt::glyph_to_sdf(&mut image, esdt::Params::default(), reuse_bufs.take());
        reuse_bufs = Some(bufs);
        GlyphData { ch: *ch, commands, sdf, bbox }
    }).collect();

    // pass 2 — layout solver (placeholder: uniform 2x2 grid for now)
    let stroke_width = 2.0;
    let spacing = 6.0;
    let cell_w = (width - stroke_width - spacing) / 2.0;
    let cell_h = (height - stroke_width - spacing) / 2.0;

    let layout: Vec<GlyphLayout> = glyph_data.iter().enumerate().map(|(i, glyph)| {
        let (x_min, y_min, x_max, y_max) = glyph.bbox;
        let font_w = x_max - x_min;
        let font_h = y_max - y_min;

        let scale_x = cell_w / font_w;
        let scale_y = cell_h / font_h;

        let col = (i % 2) as f32;
        let row = (i / 2) as f32;

        let x_offset = col * (cell_w + spacing) - x_min * scale_x;
        let y_offset = row * (cell_h + spacing) + cell_h - y_min * scale_y;

        GlyphLayout {
            digit: glyph.ch.to_string(),
            commands: to_js_commands(&glyph.commands, scale_x, scale_y),
            x_offset,
            y_offset,
        }
    }).collect();

    serde_wasm_bindgen::to_value(&layout).unwrap()
}