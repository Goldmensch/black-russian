use std::ops::{RangeInclusive};

use crate::{rom::NesFile, memory::Memory};

use super::Mapper;

const KIB_16: usize = 2usize.pow(10) * 16;

enum NRomType {
    Nrom128,
    Nrom256,
}

pub struct NRom {
  nrom_type: NRomType,
  rom: NesFile

}

impl NRom {
  pub fn new(rom: NesFile) -> NRom {
    let nrom_type = if rom.prg_rom.len() > KIB_16 { NRomType::Nrom128} else {NRomType::Nrom256};
    NRom {
      nrom_type,
      rom
    }
  }
}

impl Mapper for NRom {
    fn init_memory(&self, memory: &mut Memory) {
        memory
            .data_vec()
            .splice(self.pgr_window(), self.rom.prg_rom.to_owned());
    }

    fn pgr_window(&self) -> RangeInclusive<usize> {
        0x8000..=0xFFFF
    }

    fn map_cpu_memory_index(&self, index: usize) -> usize {
        if self.pgr_window().contains(&index) {
            match self.nrom_type {
                NRomType::Nrom128 => index % KIB_16,
                NRomType::Nrom256 => index,
            }
        } else {
            index
        }
    }
}