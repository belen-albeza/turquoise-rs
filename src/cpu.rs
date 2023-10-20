use crate::program::{Command, Program};
use crate::wasm;
use std::error;
use std::fmt;

const WIDTH: usize = 272;
const HEIGHT: usize = 192;

#[derive(Debug, PartialEq)]
pub enum CPUError {
    #[allow(dead_code)]
    InvalidProgram,
}

impl fmt::Display for CPUError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidProgram => write!(f, "Invalid program"),
        }
    }
}

impl error::Error for CPUError {}

pub type Result<T> = std::result::Result<T, CPUError>;

#[derive(Debug, PartialEq, Clone)]
pub struct CPU {
    cursor: (usize, usize),
    v_buffer: [bool; WIDTH * HEIGHT],
    src: Program,
    flip: (i8, i8),
    is_mirror: bool,
    is_draw_disabled: bool,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cursor: (WIDTH / 2, HEIGHT / 2),
            v_buffer: [false; WIDTH * HEIGHT],
            src: Program::default(),
            flip: (1, 1),
            is_mirror: false,
            is_draw_disabled: false,
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.src = Program::from(rom);
    }

    pub fn tick(&mut self) -> Result<bool> {
        match self.src.next() {
            Some(cmd) => {
                self.exec_cmd(&cmd)?;
                Ok(false)
            }
            None => Ok(true),
        }
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn v_buffer(&self) -> &[bool; (WIDTH * HEIGHT) as usize] {
        &self.v_buffer
    }

    fn draw_cursor(&mut self) {
        let x = self.cursor.0 % WIDTH;
        let y = self.cursor.1 % HEIGHT;

        let i = y * WIDTH + x;
        self.v_buffer[i] = true;
    }

    fn exec_cmd(&mut self, cmd: &Command) -> Result<bool> {
        // wasm::log(format!("Executing {:?}", cmd).as_str());
        let shall_halt = match *cmd {
            Command::Move(x, y) => self.exec_move_cursor(x as i64, y as i64)?,
            Command::Flip(x, y) => self.exec_flip(x, y)?,
            Command::Mirror => self.exec_mirror()?,
            Command::Draw => self.exec_draw()?,
            Command::Scale(_) => Ok(false)?, // noop
            _ => {
                wasm::log(format!("unimplemented! {:?}", cmd).as_str());
                todo!("to be implemented")
            }
        };

        Ok(shall_halt)
    }

    fn exec_move_cursor(&mut self, x: i64, y: i64) -> Result<bool> {
        if !self.is_draw_disabled {
            self.draw_cursor();
        }

        let delta = if self.is_mirror { (y, x) } else { (x, y) };
        self.cursor.0 = ((self.cursor.0 as i64) + delta.0 * self.flip.0 as i64) as usize;
        self.cursor.1 = ((self.cursor.1 as i64) + delta.1 * self.flip.1 as i64) as usize;

        Ok(false)
    }

    fn exec_flip(&mut self, flip_x: u8, flip_y: u8) -> Result<bool> {
        if flip_x > 0 {
            self.flip.0 *= -1;
        }
        if flip_y > 0 {
            self.flip.1 *= -1;
        }

        Ok(false)
    }

    fn exec_mirror(&mut self) -> Result<bool> {
        self.is_mirror = !self.is_mirror;

        Ok(false)
    }

    fn exec_draw(&mut self) -> Result<bool> {
        self.is_draw_disabled = !self.is_draw_disabled;
        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn any_cpu_with_rom(rom: &[u8]) -> CPU {
        let mut cpu = CPU::new();
        cpu.load_rom(rom);
        cpu
    }

    fn assert_v_buffer_at(cpu: &CPU, position: (usize, usize), value: bool) {
        let i = position.1 * WIDTH + position.0;
        assert_eq!(cpu.v_buffer[i], value);
    }

    #[test]
    fn test_load_rom() {
        let rom: &[u8] = &[0x03, 0x01, 0x44, 0x50];
        let cpu = any_cpu_with_rom(rom);
        assert_eq!(cpu.src, Program::from(rom));
    }

    #[test]
    fn test_exec_move_cursor() {
        let rom: &[u8] = &[0x02, 0x01, 0x50]; // move 1,-1
        let mut cpu = any_cpu_with_rom(rom);

        let res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.cursor(), (WIDTH / 2 + 1, HEIGHT / 2 - 1));
        assert_v_buffer_at(&cpu, (WIDTH / 2, HEIGHT / 2), true);
    }

    #[test]
    fn test_exec_flip() {
        let rom: &[u8] = &[0x02, 0x01, 0x35]; // flip 1,0; move 1,-1
        let mut cpu = any_cpu_with_rom(rom);

        let mut res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.flip, (-1, 1));

        res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.cursor(), (WIDTH / 2 - 1, HEIGHT / 2 - 1));
    }

    #[test]
    fn test_exec_mirror() {
        let rom: &[u8] = &[0x02, 0x01, 0x75]; // mirror; move 1,-1
        let mut cpu = any_cpu_with_rom(rom);

        let mut res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.is_mirror, true);

        res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.cursor(), (WIDTH / 2 - 1, HEIGHT / 2 + 1));
    }

    #[test]
    fn test_exec_draw() {
        let rom: &[u8] = &[0x02, 0x01, 0xd5]; // draw; move 1,-1
        let mut cpu = any_cpu_with_rom(rom);

        let mut res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.is_draw_disabled, true);

        res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_v_buffer_at(&cpu, (WIDTH / 2 + 1, HEIGHT / 2 - 1), false);
    }
}
