use crate::mapper::Mapper;

pub enum MemType {
    Cpu,
}

pub struct Memory<'a> {
    mem_type: MemType,
    data: Vec<u8>,
    mapper: &'a dyn Mapper,
}

impl Memory<'_> {
    pub fn cpu(mapper: &dyn Mapper) -> Memory<'_> {
        Memory {
            mem_type: MemType::Cpu,
            data: vec!(0u8; 0xFFFF+1),
            mapper,
        }
    }

    pub fn read(&self, index: usize) -> &u8 {
        let mapped_index = self.map_index(index);
        &self.data[mapped_index]
    }

    pub fn write(&mut self, index: usize, data: u8) {
        let mapped_index = self.map_index(index);
        self.data[mapped_index] = data;
    }

    pub fn read_opcode(&self, pc: usize) -> &u8 {
      let index = self.mapper.pgr_window().start() + pc;
      self.read(index)
    }

    pub fn data_vec(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }

    fn map_index(&self, index: usize) -> usize {
        match &self.mem_type {
            MemType::Cpu => self.map_cpu(index),
        }
    }

    fn map_cpu(&self, index: usize) -> usize {
        let index = match index {
            0x800..=0x1FFF => index % 0x800, // mirrors of 2KiB internal ram (0 - 0x7FF)
            0x2008..=0x3FFF => index % 8 + 0x2000, // mirrors of NES PPU registers (0x2000 - 0x2007)
            _ => index,
        };
        self.mapper.map_cpu_memory_index(index)
    }
}
