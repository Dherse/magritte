#![feature(io_error_other, extend_one, iter_collect_into, box_patterns)]

pub mod common;
pub mod edge_case;
pub mod expr;
pub mod hl;
pub mod native;
mod native_api;

use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

use clap::Parser;
use common::CommonVisitor;
use edge_case::{
    AllocationCallbackEdgeCase, BaseStructFilter, Bool32EdgeCase, ComponentMappingEdgeCase, EdgeCase, LenEdgeCase,
    OpaqueFilterEdgeCase, OptionalFieldEdgeCase, PDataEdgeCase, PNextEdgeCase, STypeFilter, SmallVecEdgeCase,
    UUIDEdgeCase, UserDataEdgeCase, VersionEdgeCase, StringArrayEdgeCase, HandleLowLevelConverterEdgeCase,
};
use hl::HighLevelVisitor;
use magritte_build::{extensions::ExtensionsVisitor, imports::Imports, Visitable, VisitorFlags};
use magritte_types::{Bitflag, Enum, Origin, Source, Struct, TypeRef};
use native::NativeVisitor;
use proc_macro2::TokenStream;

use crate::{hl::simple::is_struct_simple, native_api::NativeApiVisitor};

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
        .with_edge_case(STypeFilter)
        .with_edge_case(UUIDEdgeCase)
        .with_edge_case(PNextEdgeCase)
        .with_edge_case(PDataEdgeCase)
        .with_edge_case(LenEdgeCase)
        .with_edge_case(OptionalFieldEdgeCase)
        .with_edge_case(Bool32EdgeCase)
        .with_edge_case(BaseStructFilter)
        .with_edge_case(ComponentMappingEdgeCase)
        .with_edge_case(AllocationCallbackEdgeCase)
        .with_edge_case(OpaqueFilterEdgeCase)
        .with_edge_case(UserDataEdgeCase)
        .with_edge_case(VersionEdgeCase)
        .with_edge_case(SmallVecEdgeCase)
        .with_edge_case(StringArrayEdgeCase)
        .with_edge_case(HandleLowLevelConverterEdgeCase)
        .build()?;

    Ok(())
}

#[derive(Default)]
pub struct CoreBuilder {
    magritte_ron: Option<PathBuf>,
    magritte_doc: Option<PathBuf>,

    out: Option<PathBuf>,

    edge_cases: Vec<Box<dyn EdgeCase + Send + Sync>>,
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

    pub fn with_edge_case(mut self, edge_case: impl EdgeCase + Send + Sync + 'static) -> Self {
        self.edge_cases.push(Box::new(edge_case));

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

        let common = out.join("common/");
        let native = out.join("native/");
        let high_level = out.join("high_level/");

        // Run the visitors in parallel
        std::thread::scope(|s| {
            s.spawn(|| {
                source.visit_all(&mut CommonVisitor {
                    edge_cases: &self.edge_cases,
                    doc: doc.clone(),
                    out: common,
                    handles: Vec::with_capacity(source.handles().len()),
                });

                println!("Common done!");
            });

            s.spawn(|| {
                source.visit_all(&mut NativeVisitor {
                    edge_cases: &self.edge_cases,
                    doc: doc.clone(),
                    out: native,
                });

                println!("Native done!");
            });

            let native = out.join("native/");
            s.spawn(|| {
                let mut visitor = NativeApiVisitor {
                    out: native,
                    edge_cases: &self.edge_cases,
                    imports: Imports::new(&Origin::Unknown, "crate::native"),
                    handles: Vec::with_capacity(source.handles().len()),
                    functions: Vec::with_capacity(source.functions().len() + source.commands().len()),
                    tt: TokenStream::new(),
                };

                source.visit_all(&mut visitor);

                visitor.write();

                println!("Native done!");
            });

            s.spawn(|| {
                source.visit_all(&mut HighLevelVisitor {
                    doc: doc.clone(),
                    out: high_level,
                    edge_cases: &self.edge_cases,
                });

                println!("High level done!");
            });

            s.spawn(|| {
                let common = out.join("common/");
                let native = out.join("native/");
                let high_level = out.join("high_level/");

                let mut visitor = ExtensionsVisitor::default();
                source.visit(&mut visitor, VisitorFlags::EXTENSIONS);

                visitor.write(common);
                visitor.write(native);
                visitor.write(high_level);

                println!("Extensions done!");
            });
        });

        let (s, depth) = source
            .structs()
            .iter()
            .filter(|struct_| !struct_.origin().is_disabled())
            .filter(|struct_| self.edge_cases.type_filter(&source, TypeRef::Struct(struct_)))
            .map(|s| (s, visit_struct(&self.edge_cases, &source, s)))
            .fold((None, 0), |(max, max_depth), (s, depth)| {
                if depth > max_depth {
                    (Some(s), depth)
                } else {
                    (max, max_depth)
                }
            });

        fn visit_struct(
            edge_cases: &Vec<Box<dyn EdgeCase + Send + Sync>>,
            source: &Source<'_>,
            struct_: &Struct<'_>,
        ) -> usize {
            if is_struct_simple(edge_cases, source, struct_) {
                return 0;
            }

            struct_
                .fields()
                .iter()
                .filter(|field| field.original_name() != "pNext")
                .filter_map(|field| field.ty().base_named_type())
                .filter_map(|ty| source.resolve_type(ty))
                .map(|ty| match ty {
                    TypeRef::Struct(s) => {
                        if is_struct_simple(edge_cases, source, s) {
                            1
                        } else {
                            1 + visit_struct(edge_cases, source, s)
                        }
                    },
                    _ => 1,
                })
                .max()
                .unwrap_or(1)
        }

        let mut complex = 0;
        let mut simple = 0;
        for s in source.structs() {
            if s.origin().is_disabled() {
                continue;
            }

            if !self.edge_cases.type_filter(&source, TypeRef::Struct(s)) {
                continue;
            }

            let is_simple = is_struct_simple(&self.edge_cases, &source, s);
            if is_simple {
                println!("{}: {}", s.name(), is_simple);
            }
            complex += !is_simple as usize;
            simple += is_simple as usize;
        }

        println!("Simple: {}", simple);
        println!("Complex: {}", complex);

        println!("Struct depth: {} {:?}", depth, s);

        let mut count = 0;
        for fun in source.functions().iter().chain(source.commands().iter().map(|c| &**c)) {
            if fun.origin().is_disabled() {
                continue;
            }

            if !self.edge_cases.function_filter(&source, fun) {
                continue;
            }

            for arg in fun.arguments() {
                eprintln!("{:?}", arg.ty());
            }

            count += 1;
        }

        println!("Functions: {}", count);

        Ok(())
    }
}
