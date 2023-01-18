#![feature(io_error_other, extend_one)]

mod bits;
mod r#const;
mod field;
mod visitor;
mod bitflags;

use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

use magritte_build::{Visitable, VisitorFlags, extensions::ExtensionsVisitor};
use magritte_types::{Source, Bitflag, Enum};
use visitor::NativeBackendVisitor;

#[derive(Default)]
pub struct BackendBuilder {
    magritte_ron: Option<PathBuf>,
    magritte_doc: Option<PathBuf>,

    out: Option<PathBuf>,
}

impl BackendBuilder {
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

        let mut extension_visitor = ExtensionsVisitor::default();
        source.visit(&mut extension_visitor, VisitorFlags::EXTENSIONS);

        extension_visitor.write(&out);

        source.visit_all(&mut NativeBackendVisitor { doc, out });

        Ok(())
    }
}
