use js_sys::Reflect;
use std::{f64, io::Write};
use viteditor_rs::{Editor, Position, Viteditor};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::{console, Blob, CanvasRenderingContext2d, ConsoleEvent, Event, FileReaderSync};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct WebEditor(Viteditor);

impl Editor for WebEditor {
    fn terminal_size() -> (usize, usize) {
        let document = web_sys::window().unwrap().document().unwrap();
        let rows = ((document.document_element().unwrap().client_height() - 10) / 10) as usize;
        let cols = ((document.document_element().unwrap().client_width() - 10) / 10) as usize;
        (rows, cols)
    }

    fn goto<T: std::io::Write>(out: &mut T, pos: Position) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn clear_all<T: Write>(out: &mut T) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn write_str<T: Write>(out: &mut T, str: &str) -> Result<(), std::io::Error> {
        out.write(str.as_bytes()).unwrap();
        out.flush()
    }
}

struct Ctx(CanvasRenderingContext2d);

impl Write for Ctx {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let str = String::from_utf8(buf.to_vec()).unwrap();
        for (i, c) in str.chars().enumerate() {
            self.0
                .fill_text(c.to_string().as_str(), (i * 10) as f64, 25.0)
                .unwrap();
        }
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    Ok(())
}

#[wasm_bindgen]
pub fn open(str: &str) {
    start(str);
}

pub fn start(str: &str) {
    let mut state = Viteditor::default();

    // state.buf = str.lines().map(|line| line.chars().collect()).collect();
    state.buf = "Helloworld"
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let mut context = Ctx(canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap());

    // Set canvas size to window size
    canvas.set_width(document.document_element().unwrap().client_width() as u32 - 10);
    canvas.set_height(document.document_element().unwrap().client_height() as u32 - 10);
    context.0.set_font("24px");
    context.0.set_text_baseline("top");
    context.0.set_text_align("left");

    WebEditor::draw(&mut context, &mut state);
}
