use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use std::{f64, io::Write};
use wasm_bindgen::JsCast;
use viteditor_rs::{Editor, Viteditor, Position};
use web_sys::{console, CanvasRenderingContext2d, ConsoleEvent, Event, Blob, FileReaderSync};
use js_sys::Reflect;

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
           self.0.fill_text(c.to_string().as_str(), (i * 10) as f64, 10.0).unwrap();
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
    let state = WebEditor(Viteditor::default());

    console::log_1(&JsValue::from_str(str));

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    // Set canvas size to window size
    canvas.set_width(document.document_element().unwrap().client_width() as u32 - 10);
    canvas.set_height(document.document_element().unwrap().client_height() as u32 - 10);

    context.fill_text("H", 10.0, 10.0).unwrap();
}

