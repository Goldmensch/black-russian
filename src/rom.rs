use core::panic;

use nom::{IResult, bytes::complete::{take, tag}, sequence::tuple, number::complete::{be_u32, be_u16, be_u8}, Finish};
use bitflags::bitflags;

extern crate nom;
extern crate bitflags;

bitflags! {
    struct Flag: u16 {
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

pub struct Rom<'a> {
    pub mapper: u16,
    pub prg: &'a [u8],
    pub chr: &'a [u8]
}

impl Rom<'_> {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Rom<'_>> {
        let (left, (_, prg_size, chr_size, flags, _)) = tuple((
            tag(b"NES\x1A"), // contant NES<eof>
            be_u8, // size of prg rom
            be_u8, // size of chr rom
            be_u16, // flags 6 and 7
            take(8u32) // flags 8 to 10 and unused padding (5 byte)
        ))(input)?;

        let (flags, mapper) = (Flag::from_bits_truncate(flags), flags & 0x0F | (flags & 0xF000 << 4));
        let (left, (_, prg, chr, _, _)) = tuple((
            take(if_set(&flags, Flag::TRAINER, 512)), // Trainer
            take(kbytes(16) * prg_size as u32), // PRG ROM
            take(kbytes(8) * chr_size as u32), // CHR ROM
            take(if_set(&flags, Flag::PLAY_CHOICE_10, kbytes(8))), // PlayChoice INST-ROM
            take(if_set(&flags, Flag::PLAY_CHOICE_10, kbytes(32)))
        ))(left)?;
        let rom = Rom {
            mapper,
            prg,
            chr
        };
        Ok((left, rom))
    }
}

fn kbytes(size: u32) -> u32 {
    size * 1024
}

fn if_set(flags: &Flag, flag: Flag, size: u32) -> u32 {
    if flags.contains(flag) {size} else {0}
}