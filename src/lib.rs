#![allow(incomplete_features)]
#![feature(adt_const_params)]

pub mod renderer;

#[cfg(target_arch = "wasm32")]
pub mod wasm {
  use wasm_bindgen::prelude::*;

  impl super::renderer::ToSurface for web_sys::HtmlCanvasElement {
    unsafe fn create_surface(
      &self,
      instance: &wgpu::Instance,
    ) -> Result<wgpu::Surface, wgpu::CreateSurfaceError> {
      instance.create_surface_from_canvas(self)
    }
  }

  #[wasm_bindgen(js_name = startWithCanvas)]
  pub async fn start_with_canvas(canvas: &web_sys::HtmlCanvasElement) {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    #[cfg(feature = "console_log")]
    match console_log::init_with_level(log::Level::Info) {
      Ok(()) => (),
      Err(err) => log::error!("{}", err),
    }

    super::init(canvas, (canvas.width(), canvas.height())).await;
  }
}

pub async fn init<W: renderer::ToSurface>(window: &W, size: (u32, u32)) {
  renderer::Renderer::new(window, size).await;
}
