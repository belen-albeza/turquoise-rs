const WIDTH: usize = 272;
const HEIGHT: usize = 192;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CPU {
    cursor: (usize, usize),
    v_buffer: [bool; WIDTH * HEIGHT],
    pc: usize,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cursor: (WIDTH / 2, HEIGHT / 2),
            v_buffer: [false; WIDTH * HEIGHT],
            pc: 0,
        }
    }

    pub fn tick(&mut self) -> Result<(), String> {
        self.pc = (self.pc + 1) % (WIDTH * HEIGHT);
        self.v_buffer[self.pc] = true;
        Ok(())
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn v_buffer(&self) -> &[bool; (WIDTH * HEIGHT) as usize] {
        &self.v_buffer
    }
}
