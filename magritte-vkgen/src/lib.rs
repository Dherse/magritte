//! # Magritte VK-gen
//! Vulkan bindings generator for the Magritte multi-backend system.

#![feature(
    path_try_exists,
    if_let_guard,
    box_syntax,
    box_patterns,
    const_option_ext,
    string_remove_matches,
)]
#![warn(clippy::pedantic, clippy::cargo)]
#![deny(missing_docs)]

pub mod codegen;
pub mod doc;
pub mod expr;
pub mod imports;
pub mod name;
pub mod origin;
pub mod source;
pub mod symbols;
pub mod ty;

use std::{error::Error, fs::try_exists, io::Cursor};

use ahash::AHashMap;
use doc::Documentation;
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
pub fn parse_registry(code: String) -> Result<Registry, String> {
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
    if !try_exists(root)? {
        error!("The built documentation doesn't exist");
        error!("Did you build the Vulkan documentation?");
        std::process::exit(-1);
    }

    let read_dir = std::fs::read_dir(root)?;
    let (min, max) = read_dir.size_hint();
    let mut files = AHashMap::with_capacity(max.unwrap_or(min));

    for entry in read_dir {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            continue;
        }

        let name = entry
            .file_name()
            .to_string_lossy()
            .trim_end_matches(".html")
            .to_string();

        let bytes = std::fs::read(entry.path())?;
        let string = String::from_utf8(bytes)?;

        let html = Html::parse_document(&string);

        info!(name = %name, "Parse HTML man page");

        files.insert(name, html);
    }

    Ok(Documentation(files))
}
