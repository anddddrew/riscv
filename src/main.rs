mod format;
mod instructions
mod util

use formats::{BType, IType, JType, RType, SType, UType};
use instructions::Instruction;
use util::{dump_registers, load_elf, sign_extend};
use util::{load_word, store_yte, store_half_word, store_word};

const PC: usize = 32;
const MEM_SIZE: usize = 0x10000;
const MEM_START: usize = 0x8000000;

type Registers = [u32; 33];
type Memory = [u8; MEM_SIZE];

pub fn decode(code: u32) -> Instruction {
    let opcode = code & 0b111_1111;
    let funct3 = code >> 12 & 0b111;
    let funct7 = code >> 25 & 0b111_1111;

    match opcode {
        0b0110111 => Instruction::LUI(UType(code)),

        0b0010111 => Instruction::AUIPC(UType(code)),
    }
}
