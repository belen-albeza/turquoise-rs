use crate::program::{Command, Rule};
use std::error;
use std::fmt;

const WIDTH: usize = 272;
const HEIGHT: usize = 192;

const MEM_SIZE: usize = 0x8000;

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
    memory: [u16; MEM_SIZE],
    src: Vec<Rule>,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            cursor: (WIDTH / 2, HEIGHT / 2),
            v_buffer: [false; WIDTH * HEIGHT],
            pc: 0,
            memory: [0; MEM_SIZE],
            src: vec![],
        }
    }

    pub fn load_rom(&mut self, rom: &[u8]) {
        // let rom: Vec<u16> = raw_rom
        //     .chunks(2)
        //     .map(|chunk| match chunk.len() {
        //         2 => (chunk[0] as u16) << 8 | (chunk[1] as u16),
        //         1 => chunk[0] as u16,
        //         _ => unreachable!("Error reading ROM"),
        //     })
        //     .collect();
        // self.memory[0..rom.len()].copy_from_slice(&rom);
        self.src = vec![]
    }

    pub fn tick(&mut self) -> Result<()> {
        Ok(())
    }

    pub fn cursor(&self) -> (usize, usize) {
        self.cursor
    }

    pub fn v_buffer(&self) -> &[bool; (WIDTH * HEIGHT) as usize] {
        &self.v_buffer
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

    // #[test]
    // fn test_load_rom() {
    //     let rom: &[u8] = &[0x03, 0x01, 0x44, 0x50];
    //     let cpu = any_cpu_with_rom(rom);
    //     assert_eq!(cpu.memory[0..2], vec![0x0301, 0x4450]);
    // }
    // #[test]
    // fn test_load_rom() {
    //     let rom: &[u8] = &[0x03, 0x01, 0x44, 0x50];
    //     let cpu = any_cpu_with_rom(rom);
    //     assert_eq!(
    //         cpu.src,
    //         vec![Rule {
    //             cycles: 1,
    //             body: vec![
    //                 Command::Move(0, -1),
    //                 Command::Move(0, -1),
    //                 Command::Move(1, -1),
    //             ]
    //         }]
    //     );
    // }
}
