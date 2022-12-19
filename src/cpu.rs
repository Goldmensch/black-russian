use crate::{mapper::Mapper, memory::Memory};

use self::instructions::{Instruction, AddressingMode, OpCode};

const STACK_OFFSET: usize = 0x100;

mod instructions;

pub struct Cpu<'a> {
    memory: Memory<'a>,
    mapper: &'a dyn Mapper,
    pc: usize,
    sp: u8,
    cf: u8,
    nf: u8
}

impl Cpu<'_> {
    pub fn new(mapper: &dyn Mapper) -> Cpu<'_> {
        let memory = Memory::cpu(mapper);
        let mut cpu = Cpu {
            pc: *mapper.pgr_window().start(),
            memory,
            mapper,
            sp: 255,
            cf: 0,
            nf: 0
        };
        mapper.init_memory(&mut cpu.memory);
        cpu
    }

    pub fn push(&mut self, data: u8) {
        let index = STACK_OFFSET + self.sp as usize;
        self.memory.write(index, data);
        self.sp -= 1;
    }

    fn next_opcode(&mut self) -> u8 {
        let index = self.pc;
        self.pc += 1;
        self.memory.read(index)
    }

    fn s_opcode_data(&mut self) -> u8 {
        self.next_opcode()
    }

    fn d_opcode_data(&mut self) -> u16 {
        self.s_opcode_data() as u16 | (self.s_opcode_data() as u16) << 8
    }

    pub fn step(&mut self) {
        let raw_opcode = self.next_opcode();
        let instruction = Instruction::from(raw_opcode);

        //println!("Debug: instruction: {:?}, raw_code: {:#x}", instruction, raw_opcode);

        let data = match instruction.addressing_mode {
            AddressingMode::Absolute => {
                let address = self.d_opcode_data() as usize;
                (address, self.memory.read(address))
            }
            AddressingMode::ZeroPage => {
                let address = self.s_opcode_data() as usize;
                (address, self.memory.read(address))
            },
            other => todo!("Unimplemented addressing mode: {:?}, opcode: {:?}, raw_opcode: {:#x}", other, instruction.op_code, raw_opcode)
        };

        println!("Debug: instruction: {:?}, raw_code: {:#x}, data: {:?}", instruction, raw_opcode, data);

        match instruction.op_code {
            OpCode::Jsr => self.op_jsr(data.0),
            OpCode::Lsr => self.op_lsr(data.0, data.1),
        };
    }

    fn op_jsr(&mut self, data: usize) {
        let return_pc = self.pc - 1;
        self.push((return_pc & 0xFF00 >> 8) as u8);
        self.push((return_pc & 0x00FF) as u8);
        self.pc = data as usize;
    }

    fn op_lsr(&mut self, index: usize, data: u8) {
        self.cf = data & 1;
        self.memory.write(index, data >> 1)
    }
}
