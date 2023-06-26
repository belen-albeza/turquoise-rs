use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window().document().expect("no document in window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("no body in document")
}

fn element_by_id(id: &str) -> Option<web_sys::HtmlElement> {
    document()
        .get_element_by_id(id)
        .map(|x| x.dyn_into::<web_sys::HtmlElement>().unwrap())
}

fn create_canvas(width: u32, height: u32) -> web_sys::HtmlCanvasElement {
    let canvas = document()
        .create_element("canvas")
        .expect("could not create <canvas>")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.set_width(width);
    canvas.set_height(height);

    canvas
}

fn canvas_context(canvas: &web_sys::HtmlCanvasElement) -> web_sys::CanvasRenderingContext2d {
    canvas
        .get_context("2d")
        .expect("Could not get 2D context from canvas")
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap()
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn render(context: &web_sys::CanvasRenderingContext2d) {
    context.clear_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}

#[wasm_bindgen]
pub fn run(container_id: &str) -> Result<(), JsValue> {
    let container = element_by_id(container_id).unwrap_or(body());
    let canvas = create_canvas(WIDTH, HEIGHT);
    container
        .append_child(&canvas)
        .expect("could not append canvas to body");

    let context = canvas_context(&canvas);

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::new(move || {
        render(&context);
        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    Ok(())
}
