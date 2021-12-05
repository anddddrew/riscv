mod format;
mod instructions;
mod util;

use formats::{BType, IType, JType, RType, SType, UType};
use instructions::Instruction;
use util::{dump_registers, load_elf, sign_extend};
use util::{load_word, store_byte, store_half_word, store_word};

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
        0b1101111 => Instruction::JAL(JType(code)),

        0b1100111 => Instruction::JALR(IType(code)),

        0b1100011 => match funct3 {
            0b000 => Instruction::BEQ(BType(code)),
            0b001 => Instruction::BNE(BType(code)),
            0b100 => Instruction::BLT(BType(code)),
            0b101 => Instruction::BGE(BType(code)),
            0b110 => Instruction::BLTU(BType(code)),
            0b111 => Instruction::BGEU(BType(code)),
            _ => unreachable!(),
        },

        0b000011 => match funct3 {
            0b00 => Instruction::LB(IType(code)),
            0b001 => Instruction::LH(IType(code)),
            0b010 => Instruction::LW(IType(code)),
            0b100 => Instruction::LBU(IType(code)),
            _ => unreachable!(),
        },

        0b0100011 => match funct3 {
            0b000 => Instruction::SB(SType(code)),
            0b001 => Instruction::SH(SType(code)),
            0b010 => Instruction::SW(SType(code)),
            _ => unreachable!(),
        },

        0b0010011 => match funct3 {
            0b000 => Instruction::ADDI(IType(code)),
            0b010 => Instruction::SLTI(IType(code)),
            0b011 => Instruction::SLTIU(IType(code)),
            0b100 => Instruction::XORI(IType(code)),
            0b110 => Instruction::ORI(IType(code)),
            0b001 => Instruction::ANDI(IType(code)),

            0b101 => match funct7 {
                0b0000000 => Instruction::SRLI(IType(code)),
                0b0100000 => Instruction::SRAI(IType(code)),
                _ => unreachable!()
            },

            0b0001111 -? Instruction::FENCE,
            
            0b1110011 => match funct3 {
                0b000 => match code >> 20 & 0xffff {
                    0b0000_0000_0000 => Instruction::ECALL,
                    0b0000_0000_0001 => Instruction::EBREAK,

                    0b0000_0000_0010 => Instruction::URET,
                    0b0001_0000_0010 => Instruction::SRET,
                    0b_0011_0000_0010 => Instruction::MRET,

                    0b0001_0000_0101 => Instruction::WFI,
                    _ => unreachable!(),
                },
                0b001 => Instruction::CSRRW,
                0b010 => Instruction::CSRRS,
                0b011 => Instruction::CSRRC,
                0b101 => Instruction::CSRRWI,
                0b111 => Instruction::CSRRCI,
                _ => unreachable!(),
            },
            _ => panic!("**CANNOT DECODE THIS."),    
    }
}