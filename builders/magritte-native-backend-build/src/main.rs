use std::{error::Error, path::PathBuf};

use clap::Parser;
use magritte_native_backend_build::BackendBuilder;

/// Transforms the vk.xml file and documentation into the magritte native backend.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The `magritte.ron` file
    vk_ron: PathBuf,

    /// The built doc directory
    doc: PathBuf,

    /// The output ron file to write to
    out: PathBuf,
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();

    BackendBuilder::default()
        .with_doc(args.doc)
        .with_ron(args.vk_ron)
        .with_out(args.out)
        .build()?;

    Ok(())
}
