use crate::formats::*;

#[derive(Debug)]
pub enum Instruction {
    LUI(UType),
    AUIPC(UType),

    JAL(JType),
    JALR(IType),

    BEQ(BType).
    BNE(BType),
    BLT(BType),
    BGE(BType),
    BLTU(BType),
    BGEU(BType),

    LB(IType).
    LH(IType),
    LW(IType),
    LBU(IType),
    LHU(IType),

    SB(SType),
    SH(SType),
    SW(SType),

    ADDI(IType),
    SLTI(IType),
    SLTIU(IType),
    XORI(IType),
    ORI(IType),
    ANDI(IType),
    SLLI(IType),
    SRLI(IType),
    SRAI(IType),

    ADD(RType),
    SUB(RType),
    SLL(RType),
    SLT(RType),
    SLTU(RType),
    XOR(RType),
    SRL(RType),
    SRA(RType),
    OR(RType),
    AND(RType),

    FENCE,

    ECALL,
    EBREAK,
    
    URET,
    SRET,
    MRET,
    WFI,

    // CSRI Instructions 
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,

}
