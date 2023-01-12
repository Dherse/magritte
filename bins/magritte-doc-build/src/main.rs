#![feature(fs_try_exists)]

pub(crate) mod html;
pub(crate) mod doc;
mod visitor;

use std::{path::{PathBuf, Path}, error::Error, collections::HashMap};

use clap::Parser;
use doc::Documentation;
use magritte_build::Visitable;
use magritte_parse::Source;
use scraper::Html;
use visitor::DocVisitor;

/// Transforms the vk.xml file into a magritte ron file.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The `magritte.ron` file
    vk_ron: PathBuf,

    /// The built doc directory
    doc_dir: PathBuf,

    /// The output ron file to write to
    out: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let vk = std::fs::read_to_string(&cli.vk_ron)?;

    let source: Source = ron::de::from_str(&vk)?;

    let doc = parse_documentation(&cli.doc_dir)?;

    let mut doc_visitor = DocVisitor::new(doc, &cli.out);

    source.visit_all(&mut doc_visitor);

    Ok(())
}

/// Parses the documentation from a root folder.
pub fn parse_documentation<P: AsRef<Path>>(root: P) -> Result<Documentation, Box<dyn Error>> {
    if !std::fs::try_exists(root.as_ref())? {
        eprintln!("The built documentation doesn't exist");
        eprintln!("Did you build the Vulkan documentation?");
        std::process::exit(-1);
    }

    let files = std::fs::read_dir(root)?
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

            Some((name,  html))
        })
        .collect::<HashMap<String, Html>>();

    Ok(Documentation(files))
}