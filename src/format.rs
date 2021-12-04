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
