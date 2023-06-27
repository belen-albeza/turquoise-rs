use std::cell::RefCell;
use std::rc::Rc;

use crate::wasm::{self, RenderContext};

const WIDTH: u32 = 272;
const HEIGHT: u32 = 192;

const THEME: [&'static str; 4] = ["#22ccbb", "#000000", "#ffeebb", "#ffffff"];

pub struct VM {
    context: RenderContext,
}

impl VM {
    pub fn new(container_id: &str) -> Self {
        let container = wasm::element_by_id(container_id).unwrap_or(wasm::body());
        let canvas = wasm::create_canvas(WIDTH, HEIGHT);
        container
            .append_child(&canvas)
            .expect("could not append canvas to body");

        let context = wasm::canvas_context(&canvas);

        Self { context }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        let context = self.context.clone();
        *g.borrow_mut() = Some(wasm::Closure::new(move || {
            render(&context);
            wasm::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        wasm::request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }
}

fn render(context: &web_sys::CanvasRenderingContext2d) {
    context.set_fill_style(&wasm::JsValue::from(THEME[0]));
    context.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);
}
