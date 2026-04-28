pub enum InstuctionType {
    RType,
    IType,
    JType,
}

pub enum InstructionOp {
    SLL,
    ROL,
    SRL,
    SRA,
    AND,
    OR,
    NOR,
    XOR,
    ADD,
    SUB,
    SLT,
    SLTU,
    JR,
    ANDI,
    ORI,
    ADDI,
    SLTI,
    LW,
    SW,
    BEQ,
    BNE,
    J,
    JAL,
    LUI,
}

impl InstructionOp {
    pub fn val(&self) -> u16 {
        match self {
            Self::SLL => 0b000,
            Self::ROL => 0b000,
            Self::SRL => 0b000,
            Self::SRA => 0b000,
            Self::AND => 0b000,
            Self::OR => 0b000,
            Self::NOR => 0b000,
            Self::XOR => 0b000,
            Self::ADD => 0b001,
            Self::SUB => 0b001,
            Self::SLT => 0b001,
            Self::SLTU => 0b001,
            Self::JR => 0b001,
            Self::ANDI => 0b0100,
            Self::ORI => 0b0101,
            Self::ADDI => 0b1000,
            Self::SLTI => 0b1010,
            Self::LW => 0b0110,
            Self::SW => 0b0111,
            Self::BEQ => 0b1001,
            Self::BNE => 0b1011,
            Self::J => 0b1100,
            Self::JAL => 0b1101,
            Self::LUI => 0b1111,
        }
    }

    pub fn instruction_type(&self) -> InstuctionType {
        todo!()
    }
}

pub enum InstructionFunc {
    SLL,
    ROL,
    SRL,
    SRA,
    AND,
    OR,
    NOR,
    XOR,
    ADD,
    SUB,
    SLT,
    SLTU,
    JR,
}

impl InstructionFunc {
    pub fn val(&self) -> u16 {
        match self {
            InstructionFunc::SLL => 0b000,
            InstructionFunc::ROL => 0b001,
            InstructionFunc::SRL => 0b010,
            InstructionFunc::SRA => 0b011,
            InstructionFunc::AND => 0b100,
            InstructionFunc::OR => 0b101,
            InstructionFunc::NOR => 0b110,
            InstructionFunc::XOR => 0b111,
            InstructionFunc::ADD => 0b000,
            InstructionFunc::SUB => 0b001,
            InstructionFunc::SLT => 0b010,
            InstructionFunc::SLTU => 0b011,
            InstructionFunc::JR => 0b111,
        }
    }
}
