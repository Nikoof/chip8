pub enum Instruction {
    ClearScreen,

    Jump(usize),
    Call(usize),
    Return,

    SkipEqLiteral(usize, u8),
    SkipNeqLiteral(usize, u8),
    SkipEq(usize, usize),
    SkipNeq(usize, usize),

    SetLiteral(usize, u8),
    AddLiteral(usize, u8),

    Set(usize, usize),
    Or(usize, usize),
    And(usize, usize),
    Xor(usize, usize),
    Add(usize, usize),
    Sub(usize, usize),
    Lshift(usize, usize), // ambiguous
    Rshift(usize, usize), // ambiguous
    
    SetIndex(usize),
    JumpOffset(usize), // ambiguous
    Random(usize, u8),
    Display(usize, usize, usize),

    SkipIfPressed(usize),
    SkipIfNotPressed(usize),
    GetKey(usize),

    GetDelay(usize),
    SetDelay(usize),
    SetSound(usize),

    IndexAdd(usize), // ambiguous
    IndexFont(usize),
    ToDecimal(usize),

    WriteMemory(usize),
    ReadMemory(usize),
}


macro_rules! NNN {
    ($opcode:expr) => {
        ($opcode & 0x0FFF) as usize
    }
}

macro_rules! X {
    ($opcode:expr) => {
        (($opcode & 0x0F00) >> 8) as usize 
    }
}

macro_rules! XYN {
    ($opcode:expr) => {(
        (($opcode & 0x0F00) >> 8) as usize,
        (($opcode & 0x00F0) >> 4) as usize,
        ($opcode & 0x000F) as usize 
    )}
}

macro_rules! XNN {
    ($opcode:expr) => {(
        (($opcode & 0x0F00) >> 8) as usize,
        ($opcode & 0x00FF) as u8
    )}
}

macro_rules! XY {
    ($opcode:expr) => {(
        (($opcode & 0x0F00) >> 8) as usize,
        (($opcode & 0x00F0) >> 4) as usize 
    )}
}

pub fn decode(opcode: u16) -> Option<Instruction> {
    match opcode & 0xF000 {
        0x0000 => match opcode {
            0x00E0 => Some(Instruction::ClearScreen),
            0x00EE => Some(Instruction::Return),
            _=> None
        },
        0x1000 => Some(Instruction::Jump(NNN!(opcode))),
        0x2000 => Some(Instruction::Call(NNN!(opcode))),
        0x3000 => {
            let (x, nn) = XNN!(opcode);
            Some(Instruction::SkipEqLiteral(x, nn))
        }
        0x4000 => {
            let (x, nn) = XNN!(opcode);
            Some(Instruction::SkipNeqLiteral(x, nn))
        }
        0x5000 => {
            let (x, y) = XY!(opcode);
            Some(Instruction::SkipEq(x, y))
        }
        0x9000 => {
            let (x, y) = XY!(opcode);
            Some(Instruction::SkipNeq(x, y))
        }
        0x6000 => {
            let (x, nn) = XNN!(opcode);
            Some(Instruction::SetLiteral(x, nn))
        },
        0x7000 => {
            let (x, nn) =  XNN!(opcode);
            Some(Instruction::AddLiteral(x, nn))
        },

        0x8000 => {
            let (x, y) = XY!(opcode);
            match opcode & 0x000F {
                0x0000 => Some(Instruction::Set(x, y)),
                0x0001 => Some(Instruction::Or(x, y)),
                0x0002 => Some(Instruction::And(x, y)),
                0x0003 => Some(Instruction::Xor(x, y)),
                0x0004 => Some(Instruction::Add(x, y)),
                0x0005 => Some(Instruction::Sub(x, y)),
                0x0007 => Some(Instruction::Sub(y, x)),
                0x0006 => Some(Instruction::Rshift(x, y)),
                0x000E => Some(Instruction::Lshift(x, y)),
                _ => None
            }
        }

        0xA000 => Some(Instruction::SetIndex(NNN!(opcode))),
        0xB000 => Some(Instruction::JumpOffset(NNN!(opcode))), // Might need to split this op into two (NNN/XNN).
        0xC000 => {
            let (x, nn) = XNN!(opcode);
            Some(Instruction::Random(x, nn))
        },
        0xD000 => {
            let (x, y, n) =  XYN!(opcode);
            Some(Instruction::Display(x, y, n))
        },
        0xE000 => {
            match opcode & 0x00FF {
                0x009E => Some(Instruction::SkipIfPressed(X!(opcode))),
                0x00A1 => Some(Instruction::SkipIfNotPressed(X!(opcode))),
                _ => None
            }
        }
        0xF000 => {
            let x = X!(opcode);
            match opcode & 0x00FF {
                0x0007 => Some(Instruction::GetDelay(x)),
                0x0015 => Some(Instruction::SetDelay(x)),
                0x0018 => Some(Instruction::SetSound(x)),
                0x001E => Some(Instruction::IndexAdd(x)),
                0x000A => Some(Instruction::GetKey(x)),
                0x0029 => Some(Instruction::IndexFont(x)),
                0x0033 => Some(Instruction::ToDecimal(x)),
                0x0055 => Some(Instruction::WriteMemory(x)),
                0x0065 => Some(Instruction::ReadMemory(x)),
                _ => None
            }
        }
        _ => None
    }
}
