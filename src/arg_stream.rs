use anyhow::Context;

use crate::Register;

pub struct ArgStream<'a>(Box<dyn Iterator<Item = &'a str> + 'a>);

impl<'a> ArgStream<'a> {
    pub fn new(args: &'a str) -> Self {
        Self(Box::new(args.split([',', ' ']).filter(|a| !a.is_empty())))
    }

    pub fn next(&mut self) -> anyhow::Result<&'a str> {
        self.0
            .next()
            .context("Not enough arguments for instruction")
    }

    pub fn next_reg(&mut self) -> anyhow::Result<Register> {
        let arg = self.next()?;
        arg.try_into()
            .with_context(|| format!("Invalid input for register: '{}'", arg))
    }

    pub fn next_imm(&mut self) -> anyhow::Result<i16> {
        let arg = self.next()?;
        Ok(arg.parse()?)
    }
}
