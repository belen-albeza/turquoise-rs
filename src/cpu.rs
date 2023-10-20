use crate::program::{Command, Program};
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

type Color = bool;

#[derive(Debug, PartialEq, Clone)]
pub struct CPU {
    cursor: (isize, isize),
    v_buffer: [Color; WIDTH * HEIGHT],
    src: Program,
    flip: (isize, isize),
    is_mirror: bool,
    is_draw_disabled: bool,
    color: Color,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cursor: ((WIDTH / 2) as isize, (HEIGHT / 2) as isize),
            v_buffer: [false; WIDTH * HEIGHT],
            src: Program::default(),
            flip: (1, 1),
            is_mirror: false,
            is_draw_disabled: false,
            color: true,
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

    pub fn cursor(&self) -> (isize, isize) {
        self.cursor
    }

    pub fn v_buffer(&self) -> &[bool; (WIDTH * HEIGHT) as usize] {
        &self.v_buffer
    }

    fn draw_cursor(&mut self) {
        let x = self.cursor.0;
        let y = self.cursor.1;

        if x >= WIDTH as isize || y >= HEIGHT as isize || x < 0 || y < 0 {
            return;
        }

        let i = (y as usize) * WIDTH + (x as usize);
        self.v_buffer[i] = self.color;
    }

    fn exec_cmd(&mut self, cmd: &Command) -> Result<bool> {
        let shall_halt = match *cmd {
            Command::Move(x, y) => self.exec_move_cursor(x as isize, y as isize)?,
            Command::Flip(x, y) => self.exec_flip(x, y)?,
            Command::Mirror => self.exec_mirror()?,
            Command::Draw => self.exec_draw()?,
            Command::Color => self.exec_color()?,
            Command::Scale(_) => Ok(false)?, // noop
            Command::PushPop => Ok(false)?,  // noop
        };

        Ok(shall_halt)
    }

    fn exec_move_cursor(&mut self, x: isize, y: isize) -> Result<bool> {
        let delta = if self.is_mirror { (y, x) } else { (x, y) };
        self.cursor.0 = self.cursor.0 + delta.0 * self.flip.0;
        self.cursor.1 = self.cursor.1 + delta.1 * self.flip.1;

        if !self.is_draw_disabled {
            self.draw_cursor();
        }

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

    fn exec_color(&mut self) -> Result<bool> {
        self.color = !self.color;
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
        assert_eq!(
            cpu.cursor(),
            ((WIDTH / 2 + 1) as isize, (HEIGHT / 2 - 1) as isize)
        );
        assert_v_buffer_at(&cpu, (WIDTH / 2 + 1, HEIGHT / 2 - 1), true);
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
        assert_eq!(
            cpu.cursor(),
            ((WIDTH / 2 - 1) as isize, (HEIGHT / 2 - 1) as isize)
        );
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
        assert_eq!(
            cpu.cursor(),
            ((WIDTH / 2 - 1) as isize, (HEIGHT / 2 + 1) as isize)
        );
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

    #[test]
    fn test_exec_color() {
        let rom: &[u8] = &[0x04, 0x01, 0x11, 0xc2]; // move 1,0; move 1,0; color; move -1,0
        let mut cpu = any_cpu_with_rom(rom);

        // move right x2
        let _ = cpu.tick();
        let _ = cpu.tick();
        assert_v_buffer_at(&cpu, (WIDTH / 2 + 1, HEIGHT / 2), true);
        assert_v_buffer_at(&cpu, (WIDTH / 2 + 2, HEIGHT / 2), true);

        // swap fg/bg colors
        let res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.color, false);

        // move left
        let _ = cpu.tick();
        assert_v_buffer_at(&cpu, (WIDTH / 2 + 1, HEIGHT / 2), false);
    }
}
