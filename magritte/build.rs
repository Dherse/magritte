#[cfg(feature = "video_bindgen")]
extern crate bindgen;

#[cfg(feature = "video_bindgen")]
use std::path::PathBuf;

#[cfg(feature = "video_bindgen")]
fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from("src/generated/video.rs");
    bindings.write_to_file(out_path).expect("Couldn't write bindings!");
}

#[cfg(not(feature = "video_bindgen"))]
pub fn main() {}
