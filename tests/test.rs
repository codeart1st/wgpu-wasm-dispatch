#![feature(thread_local)]
#![allow(clippy::await_holding_refcell_ref)]
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::*;

use std::cell::RefCell;
use std::sync::Once;
use wasm_bindgen::JsCast;

pub const CANVAS_SIZE: (u32, u32) = (512, 512);

static INIT: Once = Once::new();

#[thread_local]
pub static CANVAS: RefCell<Option<web_sys::HtmlCanvasElement>> = RefCell::new(None);

wasm_bindgen_test_configure!(run_in_browser);

pub fn initialize() {
  INIT.call_once(|| {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.create_element("canvas").unwrap();
    let canvas = canvas
      .dyn_into::<web_sys::HtmlElement>()
      .map_err(|_| ())
      .unwrap();

    let body = document.body().unwrap();
    let style = canvas.style();

    body.append_child(&canvas).unwrap();
    style.set_property("position", "absolute").unwrap();
    style.set_property("top", "1em").unwrap();
    style.set_property("right", "1em").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
      .dyn_into::<web_sys::HtmlCanvasElement>()
      .map_err(|_| ())
      .unwrap();

    let (width, height) = CANVAS_SIZE;
    canvas.set_width(width);
    canvas.set_height(height);

    *CANVAS.borrow_mut() = Some(canvas);
  });
}

#[wasm_bindgen_test]
async fn empty() {
  initialize();

  let canvas_ref = CANVAS.borrow();
  let canvas = canvas_ref.as_ref().unwrap();
  wgpu_wasm_dispatch::wasm::start_with_canvas(canvas).await;
}
