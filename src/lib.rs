use anyhow::{Context, bail};

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
        match self {
            Self::SLL
            | Self::ROL
            | Self::SRL
            | Self::SRA
            | Self::AND
            | Self::OR
            | Self::NOR
            | Self::XOR
            | Self::ADD
            | Self::SUB
            | Self::SLT
            | Self::SLTU
            | Self::JR => InstuctionType::RType,
            Self::ANDI
            | Self::ORI
            | Self::ADDI
            | Self::SLTI
            | Self::LW
            | Self::SW
            | Self::BEQ
            | Self::BNE => InstuctionType::IType,
            Self::J | Self::JAL | Self::LUI => InstuctionType::JType,
        }
    }

    pub fn func(&self) -> InstructionFunc {
        match self {
            Self::SLL => InstructionFunc::SLL,
            Self::ROL => InstructionFunc::ROL,
            Self::SRL => InstructionFunc::SRL,
            Self::SRA => InstructionFunc::SRA,
            Self::AND => InstructionFunc::AND,
            Self::OR => InstructionFunc::OR,
            Self::NOR => InstructionFunc::NOR,
            Self::XOR => InstructionFunc::XOR,
            Self::ADD => InstructionFunc::ADD,
            Self::SUB => InstructionFunc::SUB,
            Self::SLT => InstructionFunc::SLT,
            Self::SLTU => InstructionFunc::SLTU,
            Self::JR => InstructionFunc::JR,
            _ => InstructionFunc::SLL,
        }
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

pub enum Register {
    R0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
}

impl Register {
    pub fn val(&self) -> u16 {
        match self {
            Self::R0 => 0b000,
            Self::R1 => 0b001,
            Self::R2 => 0b010,
            Self::R3 => 0b011,
            Self::R4 => 0b100,
            Self::R5 => 0b101,
            Self::R6 => 0b110,
            Self::R7 => 0b111,
        }
    }

    pub fn parse(reg: &str) -> anyhow::Result<Self> {
        Ok(match reg {
            "$0" | "$zero" => Self::R0,
            "$1" | "$s0" => Self::R1,
            "$2" | "$s1" => Self::R2,
            "$3" | "$s2" => Self::R3,
            "$4" | "$at" => Self::R4,
            "$5" | "$a0" => Self::R5,
            "$6" | "$v0" => Self::R6,
            "$7" | "$ra" => Self::R7,
            reg => bail!("Unknown register: '{}'", reg),
        })
    }
}

impl TryInto<Register> for &str {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Register, Self::Error> {
        Register::parse(self)
    }
}

pub enum PseudoInstruction {
    NOP,
    LI,
    BLT,
    BGT,
    BLE,
    BGE,
    BLTZ,
    BGTZ,
    BLEZ,
    BGEZ,
    LA,
    ROR,
}

pub enum Instruction {
    Native(InstructionOp),
    Pseudo(PseudoInstruction),
}

impl Instruction {
    pub fn parse_inst(inst: &str) -> anyhow::Result<Self> {
        Ok(match inst {
            "sll" => Self::Native(InstructionOp::SLL),
            "rol" => Self::Native(InstructionOp::ROL),
            "srl" => Self::Native(InstructionOp::SRL),
            "sra" => Self::Native(InstructionOp::SRA),
            "and" => Self::Native(InstructionOp::AND),
            "or" => Self::Native(InstructionOp::OR),
            "nor" => Self::Native(InstructionOp::NOR),
            "xor" => Self::Native(InstructionOp::XOR),
            "add" => Self::Native(InstructionOp::ADD),
            "sub" => Self::Native(InstructionOp::SUB),
            "slt" => Self::Native(InstructionOp::SLT),
            "sltu" => Self::Native(InstructionOp::SLTU),
            "jr" => Self::Native(InstructionOp::JR),
            "andi" => Self::Native(InstructionOp::ANDI),
            "ori" => Self::Native(InstructionOp::ORI),
            "addi" => Self::Native(InstructionOp::ADDI),
            "slti" => Self::Native(InstructionOp::SLTI),
            "lw" => Self::Native(InstructionOp::LW),
            "sw" => Self::Native(InstructionOp::SW),
            "beq" => Self::Native(InstructionOp::BEQ),
            "bne" => Self::Native(InstructionOp::BNE),
            "j" => Self::Native(InstructionOp::J),
            "jal" => Self::Native(InstructionOp::JAL),
            "lui" => Self::Native(InstructionOp::LUI),
            "nop" => Self::Pseudo(PseudoInstruction::NOP),
            "li" => Self::Pseudo(PseudoInstruction::LI),
            "blt" => Self::Pseudo(PseudoInstruction::BLT),
            "bgt" => Self::Pseudo(PseudoInstruction::BGT),
            "ble" => Self::Pseudo(PseudoInstruction::BLE),
            "bge" => Self::Pseudo(PseudoInstruction::BGE),
            "bltz" => Self::Pseudo(PseudoInstruction::BLTZ),
            "bgtz" => Self::Pseudo(PseudoInstruction::BGTZ),
            "blez" => Self::Pseudo(PseudoInstruction::BLEZ),
            "bgez" => Self::Pseudo(PseudoInstruction::BGEZ),
            "la" => Self::Pseudo(PseudoInstruction::LA),
            "ror" => Self::Pseudo(PseudoInstruction::ROR),
            inst => bail!("Unknown instruction: '{}'", inst),
        })
    }

    pub fn encode(&self, args: &str) -> anyhow::Result<Vec<u16>> {
        let args = args.replace(",", " ");
        let mut args = args.split(" ").filter(|xd| xd.is_empty());

        Ok(match self {
            Self::Native(op) => match op {
                InstructionOp::SLL
                | InstructionOp::ROL
                | InstructionOp::SRL
                | InstructionOp::SRA
                | InstructionOp::AND
                | InstructionOp::OR
                | InstructionOp::NOR
                | InstructionOp::XOR
                | InstructionOp::ADD
                | InstructionOp::SUB
                | InstructionOp::SLT
                | InstructionOp::SLTU => {
                    let rd: Register = args
                        .next()
                        .context("Not enough arguments for instruction.")?
                        .try_into()?;
                    let rs: Register = args
                        .next()
                        .context("Not enough arguments for instruction.")?
                        .try_into()?;
                    let rt: Register = args
                        .next()
                        .context("Not enough arguments for instruction.")?
                        .try_into()?;

                    vec![
                        op.val() << 12
                            | rs.val() << 9
                            | rt.val() << 6
                            | rd.val() << 3
                            | op.func().val(),
                    ]
                }
                InstructionOp::JR => {
                    vec![op.val() << 12 | op.func().val()]
                }
                InstructionOp::ANDI
                | InstructionOp::ORI
                | InstructionOp::ADDI
                | InstructionOp::SLTI
                | InstructionOp::LW
                | InstructionOp::SW
                | InstructionOp::BEQ
                | InstructionOp::BNE => {
                    let rt: Register = args
                        .next()
                        .context("Not enough arguments for instruction.")?
                        .try_into()?;
                    let rs: Register = args
                        .next()
                        .context("Not enough arguments for instruction.")?
                        .try_into()?;

                    let immm: i16 = args.next().context("wowie pls immediate me")?.parse()?;
                    if immm & 0x3F != 0 {
                        bail!("Invalid immediate size.");
                    }

                    vec![op.val() << 12 | rs.val() << 9 | rt.val() << 6 | immm as u16]
                }
                InstructionOp::J | InstructionOp::JAL | InstructionOp::LUI => {
                    let immm: i16 = args.next().context("wowie pls immediate me")?.parse()?;
                    if immm & 0xFFF != 0 {
                        bail!("Invalid immediate size.");
                    }

                    vec![op.val() << 12 | immm as u16]
                }
            },
            Self::Pseudo(op) => match op {
                PseudoInstruction::NOP => {
                    vec![0]
                }
                _ => todo!(),
            },
        })
    }
}
