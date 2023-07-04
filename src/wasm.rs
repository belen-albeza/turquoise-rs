use wasm_bindgen::prelude::*;
use web_sys::ImageData;

pub type Closure<T> = wasm_bindgen::closure::Closure<T>;
pub type JsValue = wasm_bindgen::JsValue;
pub type RenderContext = web_sys::CanvasRenderingContext2d;
pub type CanvasElement = web_sys::HtmlCanvasElement;

pub fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

pub fn document() -> web_sys::Document {
    window().document().expect("no document in window")
}

pub fn element_by_id<T>(id: &str) -> Option<T>
where
    T: wasm_bindgen::JsCast,
{
    document()
        .get_element_by_id(id)
        .map(|x| x.dyn_into::<T>().unwrap())
}

pub fn canvas_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Could not get 2D context from canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

pub fn image_data(buffer: &[u8], width: u32, height: u32) -> web_sys::ImageData {
    ImageData::new_with_u8_clamped_array_and_sh(wasm_bindgen::Clamped(buffer), width, height)
        .expect("Could not create image data")
}

pub fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[allow(dead_code)]
pub fn log(msg: &str) {
    web_sys::console::log_1(&msg.into());
}
