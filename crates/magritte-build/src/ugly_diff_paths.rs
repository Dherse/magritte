use std::{io, path::Path};

pub fn ugly_diff_paths<P1: AsRef<Path>, P2: AsRef<Path>>(path: P1, base: P2) -> Result<String, io::Error> {
    let path = path.as_ref();
    let base = base.as_ref();

    if !path.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "path does not exist"));
    }

    if !base.exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "base does not exist"));
    }

    for (i, base) in base.ancestors().enumerate() {
        if path.starts_with(base) {
            let path = path.strip_prefix(base).expect("failed to strip prefix").to_path_buf();
            return Ok(format!("{}/{}", "../".repeat(i - 1), path.display()).replace("//", "/"));
        }
    }

    return Err(io::Error::new(io::ErrorKind::NotFound, "No relative path found"));
}
