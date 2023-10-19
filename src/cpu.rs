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
    pc: usize,
    rule_pc: usize,
    src: Program,
    flip: (i8, i8),
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cursor: (WIDTH / 2, HEIGHT / 2),
            v_buffer: [false; WIDTH * HEIGHT],
            pc: 0,
            rule_pc: 0,
            src: Program::default(),
            flip: (1, 1),
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        self.src = Program::from(rom);
    }

    pub fn tick(&mut self) -> Result<bool> {
        if let Some(cmd) = self.src.get_cmd(self.rule_pc, self.pc) {
            self.draw_cursor();
            self.exec_cmd(&cmd)?;
            Ok(false)
        } else {
            Ok(true)
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
        let shall_halt = match *cmd {
            Command::Move(x, y) => self.exec_move_cursor(x as i64, y as i64)?,
            Command::Flip(x, y) => self.exec_flip(x, y)?,
            _ => {
                wasm::log(format!("unimplemented! {:?}", cmd).as_str());
                todo!("to be implemented")
            }
        };

        Ok(shall_halt)
    }

    fn exec_move_cursor(&mut self, x: i64, y: i64) -> Result<bool> {
        self.cursor.0 = ((self.cursor.0 as i64) + x * (self.flip.0 as i64)) as usize;
        self.cursor.1 = ((self.cursor.1 as i64) + y * (self.flip.1 as i64)) as usize;
        self.pc += 1;

        Ok(false)
    }

    fn exec_flip(&mut self, flip_x: u8, flip_y: u8) -> Result<bool> {
        if flip_x > 0 {
            self.flip.0 *= -1;
        }
        if flip_y > 0 {
            self.flip.1 *= -1;
        }

        self.pc += 1;

        Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::{Command, Rule};

    fn any_cpu_with_rom(rom: &[u8]) -> CPU {
        let mut cpu = CPU::new();
        cpu.load_rom(rom);
        cpu
    }

    #[test]
    fn test_load_rom() {
        let rom: &[u8] = &[0x03, 0x01, 0x44, 0x50];
        let cpu = any_cpu_with_rom(rom);
        assert_eq!(
            cpu.src,
            Program {
                rules: vec![Rule {
                    cycles: 1,
                    body: vec![
                        Command::Move(0, -1),
                        Command::Move(0, -1),
                        Command::Move(1, -1)
                    ]
                }]
            }
        );
    }

    #[test]
    fn test_exec_move_cursor() {
        let rom: &[u8] = &[0x02, 0x01, 0x50]; // move 1,-1
        let mut cpu = any_cpu_with_rom(rom);

        let res = cpu.tick();
        assert_eq!(res, Ok(false));
        assert_eq!(cpu.cursor(), (WIDTH / 2 + 1, HEIGHT / 2 - 1));
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
}
