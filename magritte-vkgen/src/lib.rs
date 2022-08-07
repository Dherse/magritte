//! # Magritte VK-gen
//! Vulkan bindings generator for the Magritte multi-backend system.

#![feature(
    fs_try_exists,
    if_let_guard,
    box_syntax,
    box_patterns,
    const_option_ext,
    string_remove_matches,
    extend_one
)]
#![warn(clippy::pedantic, clippy::cargo)]
#![allow(
    clippy::missing_errors_doc,
    clippy::must_use_candidate,
    clippy::missing_panics_doc,
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::default_trait_access
)]
#![deny(missing_docs)]

pub mod codegen;
pub mod doc;
pub mod expr;
pub mod imports;
pub mod name;
pub mod origin;
pub mod rustmft;
pub mod source;
pub mod symbols;
pub mod ty;

use std::{collections::HashMap, error::Error, fs::try_exists, io::Cursor};

use doc::Documentation;
use rayon::iter::{ParallelBridge, ParallelIterator};
use scraper::Html;
use source::Source;
use tracing::{error, info, span, warn, Level};
use vk_parse::Registry;

/// Further processes the registry to get the source
pub fn process_registry_into_source<'a>(registry: &'a Registry) -> Result<Source<'a>, Box<dyn Error + Send + 'static>> {
    let span = span!(Level::INFO, "registry processing");
    let _guard = span.enter();

    let source = Source::new(registry);

    info!("source created");

    Ok(source)
}

/// Parses the registry from its XML source.
pub fn parse_registry(code: &str) -> Result<Registry, String> {
    let span = span!(Level::INFO, "registry parsing");
    let _guard = span.enter();

    let (registry, warnings) = vk_parse::parse_stream(Cursor::new(&code)).map_err(|e| format!("{:?}", e))?;

    info!("Parsed registry");

    if warnings.is_empty() {
        info!("No warnings");
    }

    for warning in warnings {
        match warning {
            vk_parse::Error::UnexpectedElement { xpath, name } => {
                warn!(file = ?xpath, "unexpected element: `{}`", name);
            },
            vk_parse::Error::UnexpectedAttribute { xpath, name } => {
                warn!(file = ?xpath, "unexpected attribute: `{}`", name);
            },
            vk_parse::Error::MissingElement { xpath, name } => {
                warn!(file = ?xpath, "missing element: `{}`", name);
            },
            vk_parse::Error::MissingAttribute { xpath, name } => {
                warn!(file = ?xpath, "missing attribute: `{}`", name);
            },
            vk_parse::Error::SchemaViolation { xpath, desc } => {
                warn!(file = ?xpath, "schema violation: {}`", desc);
            },
            vk_parse::Error::UnexpectedAttributeValue { xpath, name, value } => {
                warn!(file = ?xpath, "unexpected attribute value `{} = \"{}\"`", name, value);
            },
            vk_parse::Error::ParseIntError { xpath, text, error } => {
                warn!(file = ?xpath, "failed to parse int in `{}`: {}", text, error);
            },
            vk_parse::Error::Internal { desc } => {
                warn!(context = "vk_parse", "{}", desc);
            },
            warning => error!("unknown warning: {:?}", warning),
        }
    }

    Ok(registry)
}

/// Parses the documentation from a root folder.
pub fn parse_documentation(root: &str) -> Result<Documentation, Box<dyn Error + Send + Sync + 'static>> {
    #[repr(transparent)]
    struct SafeHtml(Html);
    unsafe impl Send for SafeHtml {}

    if !try_exists(root)? {
        error!("The built documentation doesn't exist");
        error!("Did you build the Vulkan documentation?");
        std::process::exit(-1);
    }

    let read_dir = std::fs::read_dir(root)?;
    let files = read_dir
        .par_bridge()
        .filter_map(|entry| {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                return None;
            }

            let name = entry
                .file_name()
                .to_string_lossy()
                .trim_end_matches(".html")
                .to_string();

            let bytes = std::fs::read(entry.path()).unwrap();
            let string = String::from_utf8(bytes).unwrap();

            let html = Html::parse_document(&string);

            info!(name = %name, "Parse HTML man page");

            Some((name, SafeHtml(html)))
        })
        .collect::<HashMap<String, SafeHtml>>();

    Ok(Documentation(unsafe { std::mem::transmute(files) }))
}
