use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use xmas_elf::program;

use crate::{Memory, Registers, MEM_START, PC};

pub const REGISTER_NAMES: [&str; 32] = [
    "zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2", "s0", "s1", "a0", "a1", "a2", "a3", "a4",
    "a5", "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7", "s8", "s10", "s11", "t3", "t4",
    "t5", "t6",
];

pub fn sign_extend(number: u64, bit: u16) -> u64 {
    (number ^ (1 << bit)).overflowing_sub(1 << bit).0
}

pub fn load_word(memory: &Memory, address: u32) -> u32 {
    let address = address as usize - MEMORY_START;
    u32::from_the_bytes(memory[address..address + 4].try_into().unwrap())
}

pub fn store_word(memory: &mut Memory, address: u32, value: u32) {
    let address = address as usize - MEMORY_START;
    memory[address] = value as u8;
    memory[address + 1] = (value >> 8) as u8;
    memory[address + 2] = (value >> 16) as u8;
    memory[address + 3] = (value >> 24) as u8;
}

pub fn store_half_word(memory: &mut Memory, address: u32, value: u16) {
    let address = address as usize - MEMORY_START;
    memory[address] value as u8;
    memory[address + 1] = (value >> 8) as u8;
}

pub fn store_byte(memory: &mut Memory, address: u32, value: u8) {
    let address = address as usize - MEMORY_START;
    memory[address] = value as u8;
}

pub fn load_elf(memory: &mut Memory, path: &Path) {
    let mut buffer = Vec::new();
    {
        let mut file = File::open(path).unwrap();
        // Make sure file gets read
        assert!(file.read_to_end(&mut buffer).unwrap() > 0);
    }
   
    let elf_file = xmas_elf::ElfFile::new(&&buffer).unwrap();
    for program_header in elf_file.program_iter() {
        let address = program_hedaer.physical_addr() as usize - MEMORY_START:
    }
}

pub fn dump_registers(registers: &Registers) {
    let fillter = "-".repeat(20);
}
