use std::{
    fs::File,
    io::Read, path::Path
};
use anyhow::{anyhow, Result};
use rand::random;

use crate::instruction::{Instruction, decode};

pub struct State {
    memory: [u8; 4096],
    display: [[bool; 64]; 32],
    stack: Vec<usize>,
    pc: usize,
    index: usize,
    delay: u8,
    sound: u8,
    variable: [u8; 16]
}

impl Default for State {
    fn default() -> Self {
        let mut _mem = [0u8; 4096];
        _mem[0x50..=0x9F].copy_from_slice(&[
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
        ]);
        Self {
            memory: _mem,
            display: [[false; 64]; 32],
            stack: Vec::new(),
            pc: 0x200,
            index: 0,
            delay: 0,
            sound: 0,
            variable: [0u8; 16]
        }
    }
}

impl State {
    pub fn load_program(&mut self, file_path: &str) -> Result<()> {
        let path = Path::new(file_path);
        let mut file = File::open(path)?;
        let mut buf: Vec<u8> = Vec::new();
        file.read_to_end(&mut buf)?;
        self.memory[0x200..0x200+buf.len()]
            .copy_from_slice(&buf);
        Ok(())
    }

    pub fn update(&mut self) -> Result<()> {
        let instruction = self.next_instruction()?;
        self.execute_instruction(instruction);
        Ok(())
    }

    pub fn get_points(&self) -> Vec<(f64, f64)> {
        let mut coords: Vec<(f64, f64)> = Vec::new();
        for row in 0..32 {
            for col in 0..64 {
                if self.display[row][col] {
                    coords.push((col as f64, (32 - row) as f64));
                }
            }
        }
        coords
    }

    fn next_instruction(&mut self) -> Result<Instruction> {
        let num = self.memory[self.pc..self.pc+2].try_into()?;
        let num = u16::from_be_bytes(num);
        self.pc += 2;
        if let Some(instruction) = decode(num) {
            Ok(instruction)
        } else {
            Err(anyhow!("Couldn't decode instruction code {:08X}", num))
        }
    }

    fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::ClearScreen => {
                self.display.fill([false; 64])
            },
            Instruction::Jump(n) => {
                self.pc = n as usize
            }
            Instruction::Call(n) => {
                self.stack.push(self.pc);
                self.pc = n as usize;
            }
            Instruction::Return => {
                if let Some(addr) = self.stack.pop() {
                    self.pc = addr;
                } else {
                    panic!("Invalid Return (empty call stack)");
                }
            },
            Instruction::SkipEqLiteral(x, n) => {
                if self.variable[x] == n {
                    self.pc += 2;
                } 
            },
            Instruction::SkipNeqLiteral(x, n) => {
                if self.variable[x] != n {
                    self.pc += 2;
                }
            },
            Instruction::SkipEq(x, y) => {
                if self.variable[x] == self.variable[y] {
                    self.pc += 2;
                }
            },
            Instruction::SkipNeq(x, y) => {
                if self.variable[x] != self.variable[y] {
                    self.pc += 2;
                }
            },
            Instruction::SetLiteral(x, n) => {
                self.variable[x] = n;
            },
            Instruction::AddLiteral(x, n) => {
                self.variable[x] = self.variable[x].wrapping_add(n);
            },
            Instruction::Set(x, y) => {
                self.variable[x] = self.variable[y];
            },
            Instruction::Or(x, y) => {
                self.variable[x] |= self.variable[y];
            },
            Instruction::And(x, y) => {
                self.variable[x] &= self.variable[y];
            },
            Instruction::Xor(x, y) => {
                self.variable[x] ^= self.variable[y];
            },
            Instruction::Add(x, y) => {
                let result = self.variable[x].wrapping_add(self.variable[y]);
                self.variable[0xF] = (result < self.variable[x]) as u8;
                self.variable[x] = result;
            },
            Instruction::Sub(x, y) => {
                let result = self.variable[x].wrapping_sub(self.variable[y]);
                self.variable[0xF] = (result > self.variable[x]) as u8;
                self.variable[x] = result;
            },
            // TODO: Make in-place shifting configurable
            Instruction::Lshift(x, _y) => {
                self.variable[0xF] = self.variable[x] & 0b10000000;
                self.variable[x] <<= 1;
            },
            Instruction::Rshift(x, _y) => {
                self.variable[0xF] = self.variable[x] & 0b00000001;
                self.variable[x] >>= 1;
            },
            Instruction::SetIndex(n) => {
                self.index = n;
            },
            Instruction::JumpOffset(n) => {
                // TODO: Make JumpOffset configurable
                self.pc += n + self.variable[0] as usize;
            },
            Instruction::Random(x, n) => {
                self.variable[x] = random::<u8>() & n;
            },
            Instruction::Display(x, y, n) => {
                let x = (self.variable[x] % 64) as usize;
                let y = (self.variable[y] % 32) as usize;
                self.variable[0xF] = 0;

                for row in 0..n {
                    if y + row >= 32 { break; }
                    let sprite_row = self.memory[self.index + row];
                    for col in 0..8 {
                        if x + col >= 64 { break; }
                        let sprite_pixel = ((sprite_row << col) & 0b10000000) != 0;
                        let display_pixel = self.display[y + row][x + col];
                        self.display[y + row][x + col] = sprite_pixel ^ display_pixel;
                        self.variable[0xF] = (sprite_pixel == display_pixel) as u8;
                    }
                }
            },
            Instruction::SkipIfPressed(x) => {

            },
            Instruction::SkipIfNotPressed(x) => {

            },
            Instruction::GetDelay(x) => {
                self.variable[x] = self.delay;
            },
            Instruction::SetDelay(x) => {
                self.delay = self.variable[x];
            },
            Instruction::SetSound(x) => {
                self.sound = self.variable[x];
            },
            Instruction::IndexAdd(x) => {
                let result = self.index.wrapping_add(self.variable[x] as usize);
                if result >= 0x1000 {
                    self.variable[0xF] = 1;
                }
                self.index = result; // & 0x1000 ???
            },
            Instruction::GetKey(x) => {

            },
            Instruction::IndexFont(x) => {
                const FONT_ADDR: usize = 0x50;
                let char = self.variable[x] & 0x0F;
                self.index = FONT_ADDR + char as usize;
            },
            Instruction::ToDecimal(x) => {
                let mut vx = self.variable[x];
                for i in 0..3 {
                    self.memory[self.index + 2 - i] = vx % 10; 
                    vx /= 10;
                }
            },
            Instruction::WriteMemory(x) => {
                for i in 0..=x {
                    self.memory[self.index + i] = self.variable[i];
                }
            },
            Instruction::ReadMemory(x) => {
                for i in 0..=x {
                    self.variable[i] = self.memory[self.index + i];
                }
            }
        }
    }
}
