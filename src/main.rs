use std::{env, fs, any::Any};

use nom::{error::ErrorKind, Err::{Failure, Incomplete, Error}};
use rom::Rom;

mod cpu;
mod rom;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let binary_url = args.get(1).ok_or("You have to specify an NES rom")?;

    let binary = fs::read(binary_url).map_err(|_| "Error while loading binary.")?;

    let (_, rom) = Rom::parse(&binary).map_err(|e| {
        let error = match &e {
            Incomplete(v) => "Incomplete error: Should not occour, something really strange is going on",
            Failure(e) | Error(e) => e.code.description()
        };
        format!("Parsing error occoured: {}", error)
    })?;

    let mut cpu = cpu::Cpu::new(rom);

    loop {
        cpu.step()
    }
}