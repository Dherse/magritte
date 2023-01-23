#![feature(io_error_other, extend_one)]

pub mod native;

use std::{path::{PathBuf, Path}, error::Error, io};

use clap::Parser;
use magritte_build::Visitable;
use magritte_types::{Bitflag, Enum, Source};
use native::NativeBackendVisitor;

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

    CoreBuilder::default()
        .with_doc(args.doc)
        .with_ron(args.vk_ron)
        .with_out(args.out)
        .build()?;

    Ok(())
}

#[derive(Default)]
pub struct CoreBuilder {
    magritte_ron: Option<PathBuf>,
    magritte_doc: Option<PathBuf>,

    out: Option<PathBuf>,
}

impl CoreBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_ron<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.magritte_ron = Some(path.as_ref().to_owned());

        self
    }

    pub fn with_doc<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.magritte_doc = Some(path.as_ref().to_owned());

        self
    }

    pub fn with_out<P: AsRef<Path>>(mut self, path: P) -> Self {
        self.out = Some(path.as_ref().to_owned());

        self
    }

    pub fn build(self) -> Result<(), Box<dyn Error>> {
        let ron_path = self.magritte_ron.ok_or_else(|| io::Error::other("missing RON file"))?;
        let doc = self
            .magritte_doc
            .ok_or_else(|| io::Error::other("missing documentation folder"))?
            .canonicalize()?;

        let out = self
            .out
            .ok_or_else(|| io::Error::other("missing documentation folder"))?;

        let ron_source = std::fs::read_to_string(ron_path)?;
        let mut source: Source = ron::from_str(&ron_source)?;

        source.bitflags_mut().iter_mut().for_each(Bitflag::clear_duplicates);
        source.enums_mut().iter_mut().for_each(Enum::clear_duplicates);

        let native = out.join("native/");
        source.visit_all(&mut NativeBackendVisitor { doc, out: native });

        Ok(())
    }
}