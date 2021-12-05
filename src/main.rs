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

fn decode(code: u32) -> Instruction {
    let opcode = code & 0b111_1111;
    let funct3 = code >> 12 & 0b111;
    let funct7 = code >> 25 & 0b111_1111;
    match opcode {
        // LUI
        0b0110111 => Instruction::LUI(UType(code)),
        // AUIPC
        0b0010111 => Instruction::AUIPC(UType(code)),
        // JAL
        0b1101111 => Instruction::JAL(JType(code)),
        // JALR
        0b1100111 => Instruction::JALR(IType(code)),
        // BRANCH
        0b1100011 => match funct3 {
            0b000 => Instruction::BEQ(BType(code)),
            0b001 => Instruction::BNE(BType(code)),
            0b100 => Instruction::BLT(BType(code)),
            0b101 => Instruction::BGE(BType(code)),
            0b110 => Instruction::BLTU(BType(code)),
            0b111 => Instruction::BGEU(BType(code)),
            _ => unreachable!(),
        },
        // LOAD
        0b0000011 => match funct3 {
            0b000 => Instruction::LB(IType(code)),
            0b001 => Instruction::LH(IType(code)),
            0b010 => Instruction::LW(IType(code)),
            0b100 => Instruction::LBU(IType(code)),
            0b101 => Instruction::LHU(IType(code)),
            _ => unreachable!(),
        },
        // STORE
        0b0100011 => match funct3 {
            0b000 => Instruction::SB(SType(code)),
            0b001 => Instruction::SH(SType(code)),
            0b010 => Instruction::SW(SType(code)),
            _ => unreachable!(),
        },
        // OP-IMM
        0b0010011 => match funct3 {
            0b000 => Instruction::ADDI(IType(code)),
            0b010 => Instruction::SLTI(IType(code)),
            0b011 => Instruction::SLTIU(IType(code)),
            0b100 => Instruction::XORI(IType(code)),
            0b110 => Instruction::ORI(IType(code)),
            0b111 => Instruction::ANDI(IType(code)),
            0b001 => Instruction::SLLI(IType(code)),
            0b101 => match funct7 {
                0b0000000 => Instruction::SRLI(IType(code)),
                0b0100000 => Instruction::SRAI(IType(code)),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        },
        // OP
        0b0110011 => match funct3 {
            0b000 => match funct7 {
                0b0000000 => Instruction::ADD(RType(code)),
                0b0100000 => Instruction::SUB(RType(code)),
                _ => unreachable!(),
            },
            0b001 => Instruction::SLL(RType(code)),
            0b010 => Instruction::SLT(RType(code)),
            0b011 => Instruction::SLTU(RType(code)),
            0b100 => Instruction::XOR(RType(code)),
            0b101 => match funct7 {
                0b0000000 => Instruction::SRL(RType(code)),
                0b0100000 => Instruction::SRA(RType(code)),
                _ => unreachable!(),
            },
            0b110 => Instruction::OR(RType(code)),
            0b111 => Instruction::AND(RType(code)),
            _ => unreachable!(),
        },
        // FENCE
        0b0001111 => Instruction::FENCE,
        // SYSTEM
        0b1110011 => match funct3 {
            0b000 => match code >> 20 & 0xffff {
                0b0000_0000_0000 => Instruction::ECALL,
                0b0000_0000_0001 => Instruction::EBREAK,
                // Trap-Return Instructions
                0b0000_0000_0010 => Instruction::URET,
                0b0001_0000_0010 => Instruction::SRET,
                0b0011_0000_0010 => Instruction::MRET,
                // Interrupt-Management Instructions
                0b0001_0000_0101 => Instruction::WFI,
                _ => unreachable!(),
            },
            0b001 => Instruction::CSRRW,
            0b010 => Instruction::CSRRS,
            0b011 => Instruction::CSRRC,
            0b101 => Instruction::CSRRWI,
            0b110 => Instruction::CSRRSI,
            0b111 => Instruction::CSRRCI,
            _ => unreachable!(),
        },
        _ => panic!("**ERROR: CANNOT DECODE THIS**"),
    }
}