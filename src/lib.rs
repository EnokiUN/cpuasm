mod arg_stream;
pub mod models;

use std::collections::HashMap;

use anyhow::{Context, bail};
pub use models::*;

pub enum ParseState {
    Text,
    Data,
}

pub fn parse(code: String) -> anyhow::Result<(Vec<u16>, Vec<u16>)> {
    let mut state = ParseState::Text;
    let mut symbol_table: HashMap<String, u16> = HashMap::new();

    let mut insts = vec![];
    let mut insts_unencoded = vec![];
    let mut data = vec![];

    for (real_idx, line) in code.lines().enumerate() {
        let mut line = line
            .split("#")
            .next()
            .expect("Error happened while handling comment split")
            .trim();
        if line.is_empty() {
            continue;
        };

        if line == ".data" {
            state = ParseState::Data;
            continue;
        } else if line == ".text" {
            state = ParseState::Text;
            continue;
        }

        match state {
            ParseState::Text => {
                if let Some((label, inst)) = line.split_once(':') {
                    line = inst.trim();
                    symbol_table.insert(label.trim().to_string(), insts.len() as u16);
                }
                let (inst, args) = match line.split_once(" ") {
                    Some(s) => s,
                    None => (line, ""),
                };
                let parsed = Instruction::parse_inst(inst).with_context(|| {
                    format!("Invalid instruction on line {}: '{}", real_idx, inst)
                })?;
                insts_unencoded.push((parsed, args, insts.len() as u16));
                insts.extend(
                    insts_unencoded
                        .last()
                        .expect("Fatal error fetching last unencoded instruction")
                        .0
                        .encode(args, &symbol_table, insts.len() as u16, true)
                        .with_context(|| {
                            format!("Error encountered at line {}:\n\t{}", real_idx, line)
                        })?,
                );
            }
            ParseState::Data => {
                if let Some((label, data_args)) = line.split_once(':') {
                    line = data_args.trim();
                    symbol_table.insert(label.trim().to_string(), data.len() as u16);
                }
                let (data_type, data_args) = line
                    .split_once(' ')
                    .with_context(|| format!("Invalid syntax in data section on line {}", real_idx))
                    .map(|(t, d)| (t.trim(), d.trim()))?;

                match data_type {
                    ".word" => {
                        for val_str in data_args.split(',') {
                            let val = val_str.trim().parse::<u16>().with_context(|| {
                                format!("Invalid u16 '{}' in .word on line {}", val_str, real_idx)
                            })?;
                            data.push(val);
                        }
                    }
                    ".space" => {
                        let count = data_args.parse::<usize>().with_context(|| {
                            format!("Invalid size in .space on line {}", real_idx)
                        })?;
                        data.resize(data.len() + count, 0);
                    }
                    ".asciiz" => {
                        let string_content = data_args.trim_matches('"');
                        let mut bytes = string_content.as_bytes().to_vec();
                        bytes.push(0);
                        data.extend(bytes.into_iter().map(|b| b as u16));
                    }
                    t => bail!("Invalid data '{}' type on line {}", t, real_idx),
                }
            }
        }
    }

    insts.drain(..); // drain to not have to resize it
    for (inst, args, addr) in insts_unencoded {
        insts.extend(inst.encode(args, &symbol_table, addr, false)?);
    }

    Ok((insts, data))
}
