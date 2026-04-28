use std::{env, fs};

use anyhow::Context;

fn main() -> Result<(), anyhow::Error> {
    let filename = env::args()
        .nth(1)
        .context("Missing argument for filename")?;
    let code = fs::read_to_string(&filename).context("Failed to read file with assembly")?;
    println!("{}", code);

    Ok(())
}
