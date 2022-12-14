use crate::rom::Rom;

pub struct Cpu<'a> {
    rom: Rom<'a>,
    pc: u16,
}
  
impl Cpu<'_> {
    pub fn new(rom: Rom<'_>) -> Cpu<'_> {
        Cpu { 
            rom,
            pc: 0 
        }
    }
  
    fn decode_next_opcode(&mut self) -> u8 {
      self.pc += 1;
      self.rom.prg[self.pc as usize]
    }
  
    pub fn step(&mut self) {
      match self.decode_next_opcode() {
            code => panic!("Unknown op code {code:#X}"),
      }
    }
 }

enum Ops {}
