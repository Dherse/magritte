//! # Magritte VK-gen
//! Vulkan bindings generator for the Magritte multi-backend system.
#![feature(box_patterns)]
#![warn(clippy::pedantic, clippy::cargo)]
#![deny(missing_docs)]

use std::{collections::BTreeMap, error::Error, fmt::Write, io::stderr, ops::Not, path::PathBuf};

use cargo_toml::Manifest;
use clap::Parser;
use magritte_vkgen::{codegen::CodeOut, parse_documentation, parse_registry, rustmft::run_rustfmt, source::Source};
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

    let mut doc = if let Some(doc) = documentation_thread {
        doc.join().expect("failed to wait for thread")?
    } else {
        magritte_vkgen::doc::Documentation::default()
    };

    info!("Got documentation");

    rayon::ThreadPoolBuilder::new()
        .stack_size(64 << 20)
        .build_global()
        .unwrap();

    info!("Built the thread pool");

    let path = PathBuf::from(BINDING_OUT_PATH);
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
        let contents = std::fs::read(CARGO_TOML_PATH).unwrap();
        let mut manifest = Manifest::from_slice(&contents).unwrap();

        let mut features: BTreeMap<String, Vec<String>> = BTreeMap::default();

        features.insert(
            "window".to_string(),
            vec![
                "raw-window-handle".to_string(),
                "VK_KHR_surface".to_string(),
                "VK_KHR_win32_surface".to_string(),
                "VK_KHR_wayland_surface".to_string(),
                "VK_KHR_xlib_surface".to_string(),
                "VK_KHR_xcb_surface".to_string(),
                "VK_KHR_android_surface".to_string(),
                "VK_MVK_macos_surface".to_string(),
                "VK_EXT_metal_surface".to_string(),
            ],
        );

        features.insert("validation".to_string(), vec!["log".to_string(), "VK_EXT_debug_utils".to_string()]);

        features.insert(
            "default".to_string(),
            vec![
                "libloading".to_string(),
                "smallvec".to_string(),
                "window".to_string(),
                "validation".to_string(),
            ],
        );

        source.generate_feature_set(&mut features);

        manifest.features = features;

        let out = toml::ser::to_string_pretty(&manifest).unwrap();

        std::fs::write(CARGO_TOML_PATH, out.as_bytes()).unwrap();
    }

    Ok(())
}
