use std::{env, fs};

mod cpu;
mod mapper;
mod memory;
mod nesfile;

fn main() -> Result<(), String> {
    let binary_path: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .ok_or_else(|| "You have to specify an NES rom".to_owned())?
        .to_owned();
    let nes_file = fs::read(&binary_path)
        .map_err(|e| format!("Error while opening file: {}", e))
        .and_then(nesfile::parse_rom)?;

    println!(
        "Starting binary {} with mapper {} and flags {:?}",
        binary_path, nes_file.mapper, nes_file.flags
    );

    let mapper = mapper::choose_mapper(nes_file);
    let mut cpu = cpu::Cpu::new(&mapper);

    loop {
        cpu.step()
    }
}
