use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn empty() {
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
    style.set_property("background", "orange").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(512);
    canvas.set_height(512);

    console_error_panic_hook::set_once();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::util::backend_bits_from_env().unwrap_or(wgpu::Backends::all()),
        dx12_shader_compiler: wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default(),
    });

    // fails: assertion failed: data.is::<T>()
    let _ = match instance.create_surface_from_canvas(&canvas) {
        Ok(surface) => surface,
        Err(err) => {
            panic!("{}", err.to_string())
        }
    };
}
