//! # Magritte VK-gen
//! Vulkan bindings generator for the Magritte multi-backend system.
#![feature(box_patterns)]
#![warn(clippy::pedantic, clippy::cargo)]
#![deny(missing_docs)]

use std::{error::Error, io::stderr, ops::Not, sync::Arc};

use clap::Parser;
use magritte_vkgen::{
    codegen::VisitorFlags,
    parse_documentation, parse_registry,
    source::Source,
    visitors::{DocVisitor, FeatureVisitor, HandleVisitor, NativeVisitor},
};
use mimalloc::MiMalloc;
use tracing::{info, span, Level};
use tracing_subscriber::{fmt::time::UtcTime, EnvFilter};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// The path to `vk.xml`.
const VK_XML_PATH: &str = "vendors/Vulkan-Headers/registry/vk.xml";

/// The path to the generated Vulkan HTML man pages.
const DOC_IN_PATH: &str = "vendors/Vulkan-Docs/gen/out/man/html/";

/// The path where the generated bindings will be written to.
const BINDING_OUT_PATH: &str = "magritte/src/generated/";

/// The path where the generated bindings will be written to.
const DOC_OUT_PATH: &str = "magritte/doc/";

/// The path where the generated bindings will be written to.
const CARGO_TOML_PATH: &str = "magritte/Cargo.toml";

#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Disables the documentation generation
    #[clap(short, long)]
    no_doc: bool,
}

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    rayon::ThreadPoolBuilder::new()
        .stack_size(64 << 20)
        .build_global()
        .unwrap();

    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_timer(UtcTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(stderr)
        .init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    info!("Tracing enabled");

    let vk_xml = std::fs::read_to_string(VK_XML_PATH)?;

    let registry_thread = std::thread::spawn(move || parse_registry(&vk_xml));

    let documentation_thread = args
        .no_doc
        .not()
        .then(|| std::thread::spawn(move || parse_documentation(DOC_IN_PATH)));

    let registry = registry_thread.join().expect("failed to wait for thread")?;

    info!("Got registry");

    let source = Source::new(&registry);

    info!("Processed registry");

    let doc = if let Some(doc) = documentation_thread {
        doc.join().expect("failed to wait for thread")?
    } else {
        magritte_vkgen::doc::Documentation::default()
    };

    info!("Got documentation");
    info!("Built the thread pool");

    let mut doc_visitor = DocVisitor::new(doc, BINDING_OUT_PATH);
    source.visit(&mut doc_visitor, VisitorFlags::ORIGINS | VisitorFlags::ALL_OBJECTS);
    source.visit(&mut doc_visitor, VisitorFlags::EXTENSIONS);
    source.visit(&mut doc_visitor, VisitorFlags::VERSIONS);
    let doc = Arc::new(doc_visitor);
    info!("Visited documentation");

    let mut extension_visitor = FeatureVisitor::new();
    source.visit(&mut extension_visitor, VisitorFlags::EXTENSIONS);
    extension_visitor.write_extensions_mod(BINDING_OUT_PATH);
    extension_visitor.write(CARGO_TOML_PATH);
    info!("Visited extensions & versions for feature flag generation");

    let mut handle_visitor = HandleVisitor::new(BINDING_OUT_PATH);
    source.visit(&mut handle_visitor, VisitorFlags::ORIGINS | VisitorFlags::HANDLES);
    handle_visitor.write();
    info!("Visited handles");

    let mut native_visitor = NativeVisitor::new(Arc::clone(&doc), BINDING_OUT_PATH);
    source.visit(&mut native_visitor, VisitorFlags::all());
    info!("Visited native code");

    /*let path = PathBuf::from(BINDING_OUT_PATH);
    source
        .generate_code(&mut doc)
        .into_par_iter()
        .for_each(|CodeOut(origin, imports, header, code)| {
            let mut out = String::with_capacity(1 << 20);

            write!(out, "{}", header).unwrap();
            write!(out, "{}", imports.to_token_stream()).unwrap();
            write!(out, "{}", code).unwrap();

            let out = run_rustfmt(out).unwrap();

            std::fs::write(origin.as_file_path(&path), &out).unwrap();

            std::mem::forget(out);
        });

    {
        let mut path = path.clone();
        path.push("extensions.rs");

        let code = run_rustfmt(source.generate_extension_mod()).unwrap();
        std::fs::write(&path, &code).unwrap();
    }

    {
        let mut path = path.clone();
        path.push("extensions.rs");

        let code = run_rustfmt(source.generate_extension_mod()).unwrap();
        std::fs::write(&path, &code).unwrap();
    }

    {
        let mut path = path.clone();
        path.push("handles.rs");

        let code = run_rustfmt(source.generate_handles_mod()).unwrap();
        std::fs::write(&path, &code).unwrap();
    }*/

    Ok(())
}
