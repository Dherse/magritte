use std::fmt::Write;

use magritte_types::Expr;
use pyo3::Python;

pub trait Pivot<'a> {
    fn pivot(&self, replace: &str, target: &str) -> Expr<'static>;
}

impl<'a> Pivot<'a> for Expr<'a> {
    /// Pivots the expression to solve the expression
    fn pivot(&self, replace: &str, target: &str) -> Expr<'static> {
        initialize_python().expect("failed to initialize python");

        let as_string = format!("{} - {}", self.to_string(), replace);

        let result = Python::with_gil(|py| {
            let mut code = "import sympy\n".to_string();

            writeln!(code, "{0} = sympy.Symbol('{0}')", replace).unwrap();

            for variable in self.variables() {
                writeln!(code, "{0} = sympy.Symbol('{0}')", variable).unwrap();
            }

            for constant in self.constants() {
                writeln!(code, "{0} = sympy.Symbol('{0}')", constant).unwrap();
            }

            writeln!(code, "eq = {}", as_string).unwrap();

            py.run(&code, None, None)?;

            py.eval(&format!("str(sympy.solveset(eq, {}))", target), None, None)?
                .extract::<String>()
        })
        .expect("failed to pivot the expression");

        magritte_parse::parse_expr(&result[1..result.len() - 1])
            .expect("failed to parse the result")
            .1
    }
}

pub fn initialize_python() -> pyo3::PyResult<()> {
    // Due to https://github.com/ContinuumIO/anaconda-issues/issues/11439,
    // we first need to set PYTHONHOME. To do so, we will look for whatever
    // directory on PATH currently has python.exe.
    let python_exe = which::which("python").unwrap();
    let python_home = python_exe.parent().unwrap();

    // The Python C API uses null-terminated UTF-16 strings, so we need to
    // encode the path into that format here.
    // We could use the Windows FFI modules provided in the standard library,
    // but we want this to work cross-platform, so we do things more manually.
    let mut python_home = python_home.to_str().unwrap().encode_utf16().collect::<Vec<u16>>();
    python_home.push(0);
    unsafe {
        pyo3::ffi::Py_SetPythonHome(python_home.as_ptr());
    }

    // Once we've set the configuration we need, we can go on and manually
    // initialize PyO3.
    pyo3::prepare_freethreaded_python();

    Ok(())
}
