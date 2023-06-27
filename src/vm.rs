use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::wasm::{self, RenderContext};

const WIDTH: u32 = 272;
const HEIGHT: u32 = 192;

const THEME: [&'static str; 4] = ["#22ccbb", "#000000", "#ffeebb", "#ffffff"];

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    context: RenderContext,
    cpu: CPU,
}

impl VM {
    pub fn new(container_id: &str, cpu: CPU) -> Self {
        let container = wasm::element_by_id(container_id).unwrap_or(wasm::body());
        let canvas = wasm::create_canvas(WIDTH, HEIGHT);
        container
            .append_child(&canvas)
            .expect("could not append canvas to body");

        let context = wasm::canvas_context(&canvas);

        Self { context, cpu }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let cpu = Rc::new(RefCell::new(self.cpu));

        let context = self.context.clone();
        *g.borrow_mut() = Some(wasm::Closure::new(move || {
            cpu.borrow_mut().tick().unwrap();
            render(&context, &cpu);
            wasm::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        wasm::request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }
}

fn render(context: &web_sys::CanvasRenderingContext2d, cpu: &Rc<RefCell<CPU>>) {
    // draw background
    context.set_fill_style(&wasm::JsValue::from(THEME[0]));
    context.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

    // draw cursor
    let cursor = cpu.borrow().cursor();
    context.set_fill_style(&wasm::JsValue::from(THEME[2]));
    context.fill_rect(cursor.0 as f64, cursor.1 as f64, 1.0, 1.0);
}
