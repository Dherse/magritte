use std::{path::PathBuf, error::Error};

use clap::Parser;
use ron::ser::PrettyConfig;

/// Transforms the vk.xml file into a magritte ron file.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The `vk.xml` file to read
    vk_xml: PathBuf,

    /// The output ron file to write to
    out: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let vk = std::fs::read_to_string(&cli.vk_xml)?;

    let source = magritte_parse::parse(&vk)?;

    let config = PrettyConfig::default()
        .indentor("  ".to_owned())
        .struct_names(true)
        .compact_arrays(false);

    let ron = ron::ser::to_string_pretty(&source, config)?;

    std::fs::write(&cli.out, ron)?;

    Ok(())
}
