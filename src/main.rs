use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

use anyhow::Context;
use cpuasm::parse;

fn main() -> Result<(), anyhow::Error> {
    let mut args = env::args().skip(1);
    let filename = args.next().context("Missing argument for filename")?;

    let mut logisim_out = false;
    let partial_filename = Path::new(&filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .context("Invalid filename")?;
    let mut text_out = format!("{}-text", partial_filename);
    let mut data_out = format!("{}-data", partial_filename);
    let mut little_endian = false;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--l" | "--logisim" => logisim_out = true,
            "--t" | "--text-out" => {
                text_out = args
                    .next()
                    .context("Missing argument for text output filename")?
            }
            "--d" | "--data-out" => {
                data_out = args
                    .next()
                    .context("Missing argument for data output filename")?
            }
            "--le" | "--little-endian" => little_endian = true,
            _ => {}
        }
    }

    let code = fs::read_to_string(&filename).context("Failed to read file with assembly")?;
    let (instructions, data) = parse(code)?;

    if logisim_out {
        write_logisim(text_out, &instructions)?;
        write_logisim(data_out, &data)?;
    } else {
        write_binary(text_out, &instructions, little_endian)?;
        write_binary(data_out, &data, little_endian)?;
    }

    Ok(())
}

fn write_logisim(filename: impl AsRef<Path>, words: &[u16]) -> anyhow::Result<()> {
    let mut file = File::create(filename).context("Failed to write output to file")?;
    writeln!(file, "v2.0 raw")?;
    for word in words {
        writeln!(file, "{:04x}", word)?;
    }
    Ok(())
}

fn write_binary(
    filename: impl AsRef<Path>,
    words: &[u16],
    little_endian: bool,
) -> anyhow::Result<()> {
    let mut file = File::create(filename).context("Failed to write output to file")?;
    for word in words {
        if little_endian {
            file.write_all(&word.to_le_bytes())?;
        } else {
            file.write_all(&word.to_be_bytes())?;
        }
    }
    Ok(())
}
