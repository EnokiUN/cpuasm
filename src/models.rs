use std::collections::HashMap;

use anyhow::{Context, bail};

use crate::arg_stream::ArgStream;

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
            "$1" | "$at" => Self::R1,
            "$2" | "$s0" => Self::R2,
            "$3" | "$s1" => Self::R3,
            "$4" | "$s2" => Self::R4,
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
            inst => bail!("Unknown instruction: '{}'", inst),
        })
    }

    pub fn encode(
        &self,
        args: &str,
        symbols: &HashMap<String, u16>,
        current_addr: u16,
        partial: bool,
    ) -> anyhow::Result<Vec<u16>> {
        let mut args = ArgStream::new(args);

        // I decided to un-isolate these
        let resolve_jump = |target: &str| -> anyhow::Result<u16> {
            if partial {
                Ok(0)
            } else if let Ok(val) = target.parse::<u16>() {
                Ok(val)
            } else if let Some(&addr) = symbols.get(target) {
                Ok(addr)
            } else {
                bail!("Unresolved jump/address label: {}", target)
            }
        };

        let resolve_branch = |target: &str, branch_inst_addr: u16| -> anyhow::Result<i16> {
            if partial {
                Ok(0)
            } else if let Ok(val) = target.parse::<i16>() {
                Ok(val)
            } else if let Some(&addr) = symbols.get(target) {
                Ok(addr as i16 - branch_inst_addr as i16 - 1)
            } else {
                bail!("Unresolved branch label: {}", target)
            }
        };

        Ok(match self {
            Self::Native(op) => match op {
                InstructionOp::SLL
                | InstructionOp::ROL
                | InstructionOp::SRL
                | InstructionOp::SRA => {
                    let rd = args.next_reg()?;
                    let rs = args.next_reg()?;
                    let mut insts = vec![];
                    let shamt = args.next().context("Missing shift amount")?;

                    let shamt: Register = match shamt.try_into() {
                        Ok(reg) => reg,
                        Err(_) => {
                            let imm: i16 = shamt.parse()?;
                            if !(0..=15).contains(&imm) {
                                bail!("Invalid shift immediate: must be 0-15");
                            }

                            insts.push(
                                InstructionOp::ORI.val() << 12
                                    | Register::R0.val() << 9
                                    | Register::R1.val() << 6
                                    | (imm as u16 & 0xf),
                            );
                            Register::R1
                        }
                    };

                    insts.push(
                        op.val() << 12
                            | rs.val() << 9
                            | shamt.val() << 6
                            | rd.val() << 3
                            | op.func().val(),
                    );
                    insts
                }
                InstructionOp::AND
                | InstructionOp::OR
                | InstructionOp::NOR
                | InstructionOp::XOR
                | InstructionOp::ADD
                | InstructionOp::SUB
                | InstructionOp::SLT
                | InstructionOp::SLTU => {
                    let rd = args.next_reg()?;
                    let rs = args.next_reg()?;
                    let rt = args.next_reg()?;

                    vec![
                        op.val() << 12
                            | rs.val() << 9
                            | rt.val() << 6
                            | rd.val() << 3
                            | op.func().val(),
                    ]
                }
                InstructionOp::JR => {
                    let rs = args.next_reg()?;
                    vec![op.val() << 12 | rs.val() << 9 | op.func().val()]
                }
                InstructionOp::ANDI
                | InstructionOp::ORI
                | InstructionOp::ADDI
                | InstructionOp::SLTI
                | InstructionOp::LW
                | InstructionOp::SW => {
                    let rt = args.next_reg()?;
                    let rs = args.next_reg()?;
                    let imm = args.next_imm()?;

                    if !(-32..=31).contains(&imm) {
                        bail!("Immediate {} out of bounds for 6-bit field", imm);
                    }

                    vec![op.val() << 12 | rs.val() << 9 | rt.val() << 6 | (imm as u16 & 0x3F)]
                }
                InstructionOp::BEQ | InstructionOp::BNE => {
                    let rt = args.next_reg()?;
                    let rs = args.next_reg()?;
                    let target = args.next().context("Missing branch target")?;
                    let offset = resolve_branch(target, current_addr)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset {} out of 6-bit range", offset);
                    }

                    vec![op.val() << 12 | rs.val() << 9 | rt.val() << 6 | (offset as u16 & 0x3F)]
                }
                InstructionOp::J | InstructionOp::JAL | InstructionOp::LUI => {
                    let target = args.next().context("Missing jump/LUI target")?;
                    let addr = resolve_jump(target)?;

                    if addr > 0xFFF {
                        bail!("Address out of 12-bit range");
                    }

                    vec![op.val() << 12 | (addr & 0xFFF)]
                }
            },
            Self::Pseudo(op) => match op {
                PseudoInstruction::NOP => vec![0],
                PseudoInstruction::LI => {
                    let rs = args.next_reg()?;
                    let imm = args.next_imm()?;

                    vec![
                        InstructionOp::LUI.val() << 12 | ((imm as u16 >> 4) & 0xFFF),
                        InstructionOp::ORI.val() << 12
                            | Register::R1.val() << 9
                            | rs.val() << 6
                            | (imm as u16 & 0xF),
                    ]
                }
                PseudoInstruction::LA => {
                    let rd = args.next_reg()?;
                    let target = args.next().context("Missing LA target")?;
                    let addr = resolve_jump(target)?;

                    vec![
                        InstructionOp::LUI.val() << 12 | ((addr >> 4) & 0xFFF),
                        InstructionOp::ORI.val() << 12
                            | Register::R1.val() << 9
                            | rd.val() << 6
                            | (addr & 0xF),
                    ]
                }
                PseudoInstruction::BLT => {
                    let rs = args.next_reg()?;
                    let rt = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rs.val() << 9
                            | rt.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BNE.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BGT => {
                    let rs = args.next_reg()?;
                    let rt = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rt.val() << 9
                            | rs.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BNE.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BLE => {
                    let rs = args.next_reg()?;
                    let rt = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rt.val() << 9
                            | rs.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BEQ.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BGE => {
                    let rs = args.next_reg()?;
                    let rt = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rs.val() << 9
                            | rt.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BEQ.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BLTZ => {
                    let rs = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rs.val() << 9
                            | Register::R0.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BNE.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BGTZ => {
                    let rs = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | Register::R0.val() << 9
                            | rs.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BNE.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BLEZ => {
                    let rs = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | Register::R0.val() << 9
                            | rs.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BEQ.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
                PseudoInstruction::BGEZ => {
                    let rs = args.next_reg()?;
                    let target = args.next().context("Missing target")?;
                    let offset = resolve_branch(target, current_addr + 1)?;

                    if !(-32..=31).contains(&offset) {
                        bail!("Branch offset out of bounds");
                    }

                    vec![
                        InstructionOp::SLT.val() << 12
                            | rs.val() << 9
                            | Register::R0.val() << 6
                            | Register::R1.val() << 3
                            | InstructionOp::SLT.func().val(),
                        InstructionOp::BEQ.val() << 12
                            | Register::R1.val() << 9
                            | Register::R0.val() << 6
                            | (offset as u16 & 0x3F),
                    ]
                }
            },
        })
    }
}
