use std::cell::RefCell;
use std::rc::Rc;

use crate::cpu::CPU;
use crate::wasm::{self, RenderContext};

const WIDTH: u32 = 272;
const HEIGHT: u32 = 192;

const THEME: [&'static str; 4] = ["#22ccbb", "#000000", "#ffeebb", "#ffffff"];
trait RGBA {
    fn rgb(&self) -> (u8, u8, u8);
}

impl RGBA for &'static str {
    fn rgb(&self) -> (u8, u8, u8) {
        let r: u8 = u8::from_str_radix(&self[1..=2], 16).unwrap();
        let g: u8 = u8::from_str_radix(&self[3..=4], 16).unwrap();
        let b: u8 = u8::from_str_radix(&self[5..=6], 16).unwrap();

        (r, g, b)
    }
}

type DisplayBuffer = [u8; (WIDTH * HEIGHT * 4) as usize];

#[derive(Debug, Clone, PartialEq)]
pub struct VM {
    context: RenderContext,
    buffer: DisplayBuffer,
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
        let buffer = [0; (WIDTH * HEIGHT * 4) as usize];

        Self {
            context,
            cpu,
            buffer,
        }
    }

    pub fn run(&mut self) -> Result<(), String> {
        let cpu = Rc::new(RefCell::new(self.cpu));
        let context = self.context.clone();
        let buffer = Rc::new(RefCell::new(self.buffer));

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        *g.borrow_mut() = Some(wasm::Closure::new(move || {
            cpu.borrow_mut().tick().unwrap();
            render(&context, &cpu, &buffer);
            wasm::request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        wasm::request_animation_frame(g.borrow().as_ref().unwrap());

        Ok(())
    }
}

fn render(
    context: &web_sys::CanvasRenderingContext2d,
    cpu: &Rc<RefCell<CPU>>,
    display_buffer: &Rc<RefCell<DisplayBuffer>>,
) {
    // draw background
    context.set_fill_style(&wasm::JsValue::from(THEME[0]));
    context.fill_rect(0.0, 0.0, WIDTH as f64, HEIGHT as f64);

    // render buffer data
    update_display_buffer(cpu.borrow().v_buffer(), &mut display_buffer.borrow_mut());
    let image_data = wasm::image_data(display_buffer.borrow().as_slice(), WIDTH, HEIGHT);
    context
        .put_image_data(&image_data, 0.0, 0.0)
        .expect("Could not update canvas buffer");

    // draw cursor
    let cursor = cpu.borrow().cursor();
    context.set_fill_style(&wasm::JsValue::from(THEME[2]));
    context.fill_rect(cursor.0 as f64, cursor.1 as f64, 1.0, 1.0);
}

fn update_display_buffer(
    v_buffer: &[bool; (WIDTH * HEIGHT) as usize],
    display_buffer: &mut DisplayBuffer,
) {
    let color_a = THEME[1].rgb();
    let color_b = THEME[0].rgb();

    for i in 0..(WIDTH * HEIGHT) as usize {
        let color = if v_buffer[i] { color_a } else { color_b };
        let j = i * 4;
        display_buffer[j] = color.0;
        display_buffer[j + 1] = color.1;
        display_buffer[j + 2] = color.2;
        display_buffer[j + 3] = 0xFF;
    }
}
