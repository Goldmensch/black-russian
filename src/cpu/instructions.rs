use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug)]
pub struct Instruction {
    pub op_code: OpCode,
    pub addressing_mode: AddressingMode,
}

impl From<u8> for Instruction {
    fn from(code: u8) -> Self {
        let position = Position::from(code);
        let op_code = OpCode::from_opcode(&position, code);
        let addressing_mode = AddressingMode::from_opcode(&position, code);
        Instruction { op_code, addressing_mode }
    }
}

#[derive(Debug)]
struct Position {
    column: u8,
    row: u8,
    category: Category,
}

impl From<u8> for Position {
    fn from(code: u8) -> Self {
        Position {
            category: Category::try_from(code & 3).unwrap(),
            row: code / 0x20,
            column: code % 0x20,
        }
    }
}

#[derive(TryFromPrimitive, IntoPrimitive, PartialEq, Debug)]
#[repr(u8)]
enum Category {
    Control,
    Alu,
    Rmw,
    Unofficial,
}

#[derive(Debug)]
pub enum AddressingMode {
    Implicit,
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZergoPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndexedIndirect,
    IndirectIndexed,
}

fn in_column(column: u8, pos: &Position) -> bool {
    pos.column == column
}

fn in_n_columns(column: u8, n: u8, pos: &Position) -> bool {
    column <= pos.column && (column + n) >= pos.column
}

fn in_last_n_rows(column: u8, n: u8, pos: &Position) -> bool {
    pos.column == column && pos.row >= n
}

impl AddressingMode {
    fn from_opcode(pos: &Position, code: u8) -> Self {
        match code {
            0 | 0x40 | 0x60 => Self::Implicit,
            _ if in_column(8, pos)
                || in_column(0x0A, pos)
                || in_column(0x18, pos)
                || in_column(0x1A, pos) =>
            {
                Self::Implicit
            }
            0x20 => Self::Absolute,
            _ if in_n_columns(0xC, 4, pos) => Self::Absolute,
            _ if in_last_n_rows(0, 4, pos)
                || in_last_n_rows(1, 4, pos)
                || in_column(9, pos)
                || in_column(0xB, pos) =>
            {
                Self::Immediate
            }
            _ if in_column(1, pos) || in_column(3, pos) => Self::ZeroPageX,
            _ if in_n_columns(4, 4, pos) || in_column(0x10, pos) => Self::ZeroPage,
            _ if in_column(0x11, pos) || in_column(0x13, pos) => Self::IndirectIndexed,
            _ if in_n_columns(0x14, 4, pos)
                || in_n_columns(0x1C, 2, pos)
                || (in_n_columns(0x1E, 2, pos) && pos.row != 0xA0 || pos.row != 0x80) =>
            {
                Self::AbsoluteX
            }
            _ if in_column(0x19, pos)
                || in_column(0x1B, pos)
                || (in_n_columns(0x1E, 2, pos) && pos.row == 0xA0 || pos.row == 0x80) =>
            {
                Self::AbsoluteY
            }
            _ => kill(code),
        }
    }
}

#[derive(TryFromPrimitive, Debug)]
#[repr(u8)]
pub enum OpCode {
    Lsr = 0x46,
    Jsr = 0x20,
}

impl OpCode {
    fn from_opcode(pos: &Position, code: u8) -> OpCode {
        let row = pos.row;
        let column = pos.column;

        let num = match code {
            0x9E | 0xCA => code,
            0x8A | 0xAA => 0x8A, // TXA
            0x9A | 0xBA => 0x9A, // TXS
            0xEA | 2 | 0x22 | 0x42 | 0x80 | 4 | 0x44 | 0x64 | 0x0C | 0x89
                if (column == 0x14 || column == 0x1C || column == 0x1A)
                    && row != 0x80
                    && row != 0xA0 =>
            {
                0x80 // NOP
            }
            _ => match column {
                2 if row <= 0x60 => kill(code),
                0x0E => kill(code),
                _ => match pos.category {
                    Category::Alu => (row * 0x20) + 1, // use first cateogory column to identify op codes
                    Category::Rmw => (row * 0x20) + 6, // use second, because first is moslty nop or kill
                    Category::Control => code,
                    Category::Unofficial => {
                        panic!("Illegal op codes are currently not supported, opcode: {:#x}", code)
                    }
                },
            },
        };
        match OpCode::try_from(num) {
            Ok(opcode) => opcode,
            Err(_) => panic!("Unknown opcode, id: {:#x}, code: {:#x}, pos: {:?}", num, code, &pos),
        }
    }
}

fn kill(code: u8) -> ! {
    panic!("The emulator was killed due to opcode: {:#x}", code)
}
