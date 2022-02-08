//! Function to run `rustfmt` on a piece of source code

use std::{
    io::{self, copy, Write},
    process::{Command, Stdio},
};
use tracing::warn;

/// Runs `rustfmt` on the provided code.
pub fn run_rustfmt(source: String) -> Result<String, io::Error> {
    // This is code shamefully yoinked from the bindgen repo, all credits to them.

    let rustfmt = "rustfmt"; // TODO: search for the executable;
    let mut cmd = Command::new(&*rustfmt);

    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());
    cmd.arg("--config-path=./rustfmt.toml");

    let mut child = cmd.spawn()?;
    let mut child_stdin = child.stdin.take().unwrap();
    let mut child_stdout = child.stdout.take().unwrap();

    let mut output = Vec::with_capacity(source.len());

    // Write to stdin in a new thread, so that we can read from stdout on this
    // thread. This keeps the child from blocking on writing to its stdout which
    // might block us from writing to its stdin.
    let stdin_handle = std::thread::spawn(move || {
        let _ = child_stdin.write_all(source.as_bytes());

        std::mem::forget(source);
    });

    copy(&mut child_stdout, &mut output)?;

    let status = child.wait()?;
    stdin_handle.join().expect(
        "The thread writing to rustfmt's stdin doesn't do \
            anything that could panic",
    );

    match String::from_utf8(output) {
        Ok(bindings) => match status.code() {
            Some(0) => Ok(bindings),
            Some(2) => Err(io::Error::new(
                io::ErrorKind::Other,
                "Rustfmt parsing errors.".to_string(),
            )),
            Some(3) => {
                warn!("Rustfmt could not format some lines.");
                Ok(bindings)
            },
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                "Internal rustfmt error".to_string(),
            )),
        },
        _ => Err(io::Error::new(
            io::ErrorKind::Other,
            "Non UTF-8 string received".to_string(),
        )),
    }
}
