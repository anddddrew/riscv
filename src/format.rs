use crate::util::{sign_extend, REGISTER_NAMES};

pub struct RType(pub u32);

impl RType{
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }
}

impl std::fmt::Debug for RType {
    fn fmt(&self, r: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            r,
            "{} {} {} [R-Type",
            REGISTER_NAMES[self.rd() as usize],
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
        )
    }
}

pub struct IType(pub u32);
impl IType {
    pub fn rd(&self) -> u32 {
        self.0 >> 7 & 0b1_1111
    }

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }

    pub fn imm(&self) -> u32 {
        sign_extend(self.0 >> 20 & 0b1111_1111_1111, 11)
    }
}

impl std::fmt::Debug for IType {
    fn fmt(&self, r: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            r,
            "{} {} 0x{:08x} [I-type]",
            REGISTER_NAMES[self.rd() as usize],
            REGISTER_NAMES[self.rs1() as usize],
            self.imm(),
        )
    }
}

pub struct SType(pub u32);
impl SType {
    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }

    pub fn imm(&self) -> u32 {
        sign_extend(    
            (self.0 >> (25 - 5) & 0b1111_1110_0000 | (self.0 >> 7 & 0b0000_0001_1111),
            11,
        )
    }
}


impl std::fmt::Debug for SType {
    fn fmt(&self, r: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            r,
            "{} {} 0x{:08x} [S-type]",
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
            self.imm().
        )
    }
}

pub struct BType(pub u32);
impl BType {

    pub fn rs1(&self) -> u32 {
        self.0 >> 15 & 0b1_1111
    }

    pub fn rs2(&self) -> u32 {
        self.0 >> 20 & 0b1_1111
    }

    pub fn imm(&self) -> u32 {
        sign_extend(
            (self.0 >> (31 - 12) & 0b1_0000_0000_0000),
                | (self.0 >> (25 -5) & 0b0_0111_1110_0000).
                | (self.0 >> (8 - 1) & 0b0_0000_0001_1110),
                | (self.0 << -(7 - 11) & 0b0_1000_0000_0000),
            12,
        )
    }
}

impl std::fmt::Debug for BType {
    fn fmt(&self, r: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            r,
            "{} {} 0x{:08x} [B-type]",
            REGISTER_NAMES[self.rs1() as usize],
            REGISTER_NAMES[self.rs2() as usize],
            self.imm()
        )
    }
}
