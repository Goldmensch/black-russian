use crate::{mapper::Mapper, memory::Memory};

pub struct Cpu<'a> {
    pc: usize,
    memory: Memory<'a>,
}

impl Cpu<'_> {
    pub fn new(mapper: &dyn Mapper) -> Cpu<'_> {
        let mut cpu = Cpu {
            pc: 0,
            memory: Memory::cpu(mapper),
        };
        mapper.init_memory(&mut cpu.memory);
        cpu
    }

    fn decode_next_opcode(&mut self) -> &u8 {
        let code = self.memory.read_opcode(self.pc);
        self.pc += 1;
        code
    }

    pub fn step(&mut self) {
        match self.decode_next_opcode() {
            code => panic!("Unknown op code {code:#X}"),
        }
    }
}
