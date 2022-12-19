use std::io::Cursor;

use binrw::{binread, BinRead};
use bitflags::bitflags;

bitflags! {
    #[derive(BinRead)]
    pub struct NesFlags: u16 {
        // Flag 6
        const MIRRORING = 0b1;
        const EXTRA_PERSISTENT_MEMORY = 0b10;
        const TRAINER = 0b100;
        const IGNORE_MIRRORING = 0b1000;

        // Flag 7
        const VS_UNISYSTEM = 0b1 << 8;
        const PLAY_CHOICE_10 = 0b10 << 8;
        const NES2 = 0b1100 << 8;
    }
}

#[binread]
#[br(magic = b"NES\x1A")]
pub struct NesFile {
    #[br(temp)]
    prg_rom_unit_size: u8,
    #[br(temp)]
    chr_rom_unit_size: u8,
    pub(super) flags: NesFlags,

    #[br(pad_before = 8, count = prg_rom_unit_size as u16 * 0x4000u16)]
    pub(super) prg_rom: Vec<u8>,
    #[br(count = chr_rom_unit_size as u16 * 0x2000u16)]
    pub(super) chr_rom: Vec<u8>,

    #[br(calc = ((flags.bits() & 0xF000 >> 8) | (flags.bits() & 0xF0)) as u8)]
    pub(super) mapper: u8,
}

pub fn parse_rom(input: Vec<u8>) -> Result<NesFile, String> {
    NesFile::read_be(&mut Cursor::new(input))
        .map_err(|e| format!("Something wrent wrong while parsing: {}", e))
}
