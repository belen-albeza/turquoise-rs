#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Command {
    PushPop,
    Move(i8, i8),
    Flip(u8, u8),
    Mirror,
    Color,
    Draw,
    Scale(i8),
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        match value & 0x0F {
            0x0 => Self::PushPop,
            0x1 => Self::Move(1, 0),
            0x2 => Self::Move(-1, 0),
            0x3 => Self::Flip(1, 0),
            0x4 => Self::Move(0, -1),
            0x5 => Self::Move(1, -1),
            0x6 => Self::Move(-1, -1),
            0x7 => Self::Mirror,
            0x8 => Self::Move(0, 1),
            0x9 => Self::Move(1, 1),
            0xA => Self::Move(-1, 1),
            0xB => Self::Flip(0, 1),
            0xC => Self::Color,
            0xD => Self::Draw,
            0xE => Self::Scale(1),
            0xF => Self::Scale(-1),
            _ => unreachable!("Invalid opcode"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Rule {
    cycles: usize, // # of times to run this rule
    body: Vec<Command>,
    pc: usize,
}

impl From<&[u8]> for Rule {
    fn from(source: &[u8]) -> Self {
        let len = source[0];
        let cycles = source[1] as usize;
        let commands_iter = half_bytes(&source[2..], len as usize);

        Rule {
            cycles,
            body: commands_iter.map(Command::from).collect(),
            pc: 0,
        }
    }
}

impl Iterator for Rule {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pc >= self.body.len() * self.cycles {
            return None;
        }

        let i = self.pc % self.body.len();
        self.pc += 1;
        self.body.get(i).copied()
    }
}

struct HalfBytesChunker {
    len: usize,
    source: Vec<u8>,
    i: usize,
    buffer: Option<u8>,
}

impl Iterator for HalfBytesChunker {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.len {
            return None;
        }

        if let Some(value) = self.buffer {
            self.i += 1;
            self.buffer = None;
            return Some(value);
        }

        let byte = self.source[self.i / 2];
        self.buffer = Some(byte & 0x0F);
        let current = (byte >> 4) & 0x0F;

        self.i += 1;

        Some(current)
    }
}

fn half_bytes(source: &[u8], len: usize) -> HalfBytesChunker {
    HalfBytesChunker {
        len,
        source: source.to_owned(),
        i: 0,
        buffer: None,
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    rules: Vec<Rule>,
    rule_pc: usize,
}

impl Default for Program {
    fn default() -> Self {
        Self {
            rules: vec![],
            rule_pc: 0,
        }
    }
}

impl From<&[u8]> for Program {
    fn from(source: &[u8]) -> Self {
        let rules_iter = rules(source);

        Program {
            rules: rules_iter.map(Rule::from).collect(),
            rule_pc: 0,
        }
    }
}

impl Iterator for Program {
    type Item = Command;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rule_pc >= self.rules.len() {
            return None;
        }

        if let Some(cmd) = self.rules[self.rule_pc].next() {
            Some(cmd)
        } else {
            self.rule_pc += 1;
            self.next()
        }
    }
}

struct RulesChunker {
    source: Vec<u8>,
    i: usize,
}

impl Iterator for RulesChunker {
    type Item = Rule;

    fn next(&mut self) -> Option<Self::Item> {
        let cmd_count = *self.source.get(self.i).unwrap_or(&0);
        if cmd_count <= 0 {
            return None;
        }

        let len = ((cmd_count as f64 / 2.0).ceil() + 1.0 + 1.0) as usize;
        let rule = Rule::from(&self.source[self.i..self.i + len]);
        self.i += len;

        Some(rule)
    }
}

fn rules(source: &[u8]) -> RulesChunker {
    RulesChunker {
        source: source.to_owned(),
        i: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_from_u8() {
        assert_eq!(Command::from(0x0u8), Command::PushPop);
        assert_eq!(Command::from(0x1u8), Command::Move(1, 0));
        assert_eq!(Command::from(0x2u8), Command::Move(-1, 0));
        assert_eq!(Command::from(0x3u8), Command::Flip(1, 0));
        assert_eq!(Command::from(0x4u8), Command::Move(0, -1));
        assert_eq!(Command::from(0x5u8), Command::Move(1, -1));
        assert_eq!(Command::from(0x6u8), Command::Move(-1, -1));
        assert_eq!(Command::from(0x7u8), Command::Mirror);
        assert_eq!(Command::from(0x8u8), Command::Move(0, 1));
        assert_eq!(Command::from(0x9u8), Command::Move(1, 1));
        assert_eq!(Command::from(0xAu8), Command::Move(-1, 1));
        assert_eq!(Command::from(0xBu8), Command::Flip(0, 1));
        assert_eq!(Command::from(0xCu8), Command::Color);
        assert_eq!(Command::from(0xDu8), Command::Draw);
        assert_eq!(Command::from(0xEu8), Command::Scale(1));
        assert_eq!(Command::from(0xFu8), Command::Scale(-1));
    }

    #[test]
    fn test_rule_from_slice() {
        assert_eq!(
            Rule::from([0x03u8, 0x01u8, 0x44u8, 0x50u8].as_slice()),
            Rule {
                cycles: 1,
                body: vec![
                    Command::Move(0, -1),
                    Command::Move(0, -1),
                    Command::Move(1, -1),
                ],
                pc: 0,
            }
        );
    }

    #[test]
    fn test_program_from_slice_single_rule() {
        assert_eq!(
            Program::from([0x03u8, 0x01u8, 0x44u8, 0x50u8].as_slice()),
            Program {
                rules: vec![Rule::from([0x03u8, 0x01u8, 0x44u8, 0x50u8].as_slice()),],
                rule_pc: 0,
            }
        );
    }

    #[test]
    fn test_program_from_slice_multiple_rules() {
        assert_eq!(
            Program::from([0x01, 0x01, 0xd0, 0x01, 0x40, 0x20, 0x01, 0x10, 0x80].as_slice()),
            Program {
                rules: vec![
                    Rule::from([0x01u8, 0x01u8, 0xd0u8].as_slice()),
                    Rule::from([0x01u8, 0x40u8, 0x20u8].as_slice()),
                    Rule::from([0x01u8, 0x10u8, 0x80u8].as_slice())
                ],
                rule_pc: 0,
            }
        );
    }
}
