use wasm_bindgen::prelude::*;
use ttf_parser::Face;
use web_sys::console;

macro_rules! log {
    ($($t:tt)*) => {
        console::log_1(&format!($($t)*).into())
    }
}

const FONT_DATA: &[u8] = include_bytes!("../assets/Anta/Anta-Regular.ttf");

#[wasm_bindgen]
pub fn get_glyph(d: char) {
    let face = Face::parse(FONT_DATA, 0).expect("Failed to parse font");
    for name in face.names() {
        if name.name_id == ttf_parser::name_id::FAMILY {
            if let Some(s) = name.to_string() {
                log!("Family: {}", s);
            }
        }
    }
}