//! # Magritte VK-gen
//! Vulkan bindings generator for the Magritte multi-backend system.

#![warn(clippy::pedantic, clippy::cargo)]
#![deny(missing_docs)]

use std::{error::Error, fmt::Write, io::stderr, path::PathBuf};

use magritte_vkgen::{parse_documentation, parse_registry, rustmft::run_rustfmt, source::Source, codegen::CodeOut};
use mimalloc::MiMalloc;
use quote::ToTokens;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    tracing_subscriber::fmt()
        .with_timer(UtcTime::rfc_3339())
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(stderr)
        .init();

    let span = span!(Level::INFO, "main");
    let _guard = span.enter();

    info!("Tracing enabled");

    let vk_xml = std::fs::read_to_string(VK_XML_PATH)?;

    let registry_thread = std::thread::spawn(move || parse_registry(vk_xml));
    let documentation_thread = std::thread::spawn(move || parse_documentation(DOC_IN_PATH));

    let registry = registry_thread.join().expect("failed to wait for thread")?;

    info!("Got registry");

    let source = Source::new(&registry);

    info!("Processed registry");

    let mut doc = documentation_thread.join().expect("failed to wait for thread")?;

    info!("Got documentation");

    source.generate_code(&mut doc).into_par_iter().for_each(|CodeOut(origin, imports, code)| {
        let mut out = String::with_capacity(1 << 20);

        write!(out, "{}", imports.to_token_stream()).unwrap();
        write!(out, "{}", code).unwrap();

        let path = PathBuf::from(BINDING_OUT_PATH);

        let out = run_rustfmt(out).unwrap();
        
        std::fs::write(origin.as_file_path(&path), &out).unwrap();

        std::mem::forget(out);
    });

    Ok(())
}
