use wasm_bindgen::prelude::*;
use std::{f64, io::Write};
use wasm_bindgen::JsCast;
use viteditor_rs::{Editor, Viteditor, Position};
use web_sys::{CanvasRenderingContext2d, ConsoleEvent, Event, Blob, FileReaderSync};

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

   fn open(path: &std::path::Path, editor: &mut Viteditor) {
       let obj = web_sys::window().unwrap().window().document().unwrap().get_element_by_id("file").unwrap();

       let closure = Closure::wrap(Box::new(move |event: Event| {
           let file =event.target().unwrap().dyn_ref::<Blob>().unwrap();
           let reader = FileReaderSync::new();
           reader.read_as_text(file);
       }) as Box<dyn FnMut(Event)>);

       obj.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref());
   }
}

struct Ctx(CanvasRenderingContext2d);

impl Write for Ctx {
   fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
       for (i, c) in buf.into_iter().enumerate() {
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
    start();
    Ok(())
}

pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();

    let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

    // Set canvas size to window size
    canvas.set_width(document.document_element().unwrap().client_width() as u32 - 10);
    canvas.set_height(document.document_element().unwrap().client_height() as u32 - 10);

    context.fill_text("H", 10.0, 10.0).unwrap();
}

