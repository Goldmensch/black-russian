use std::ops::RangeInclusive;

use crate::{memory::Memory, rom::NesFile};

use self::nrom::NRom;

mod nrom;

pub trait Mapper {
    fn init_memory(&self, memory: &mut Memory);

    fn pgr_window(&self) -> RangeInclusive<usize>;

    fn map_cpu_memory_index(&self, index: usize) -> usize;
}

pub fn choose_mapper(rom: NesFile) -> impl Mapper {
    match rom.mapper {
        0 => NRom::new(rom),
        other => panic!("Unknown mapper {other}")
    }
}
