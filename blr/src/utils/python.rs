use crate::{result::Result, BlError};
use pyo3::{
    types::{PyDict, PyModule},
    Python,
};
use std::{fs::read_to_string, path::Path};

/// Run Python code that uses the Python API of Blender (`bpy`).
///
/// Code from a Python script can either be included at compile time using the
/// `std::include_str!` macro or at runtime using `std::fs::read_to_string` function.
///
/// # Examples
///
/// ```
/// use blr::utils::python::run_bpy_code;
/// use pyo3::Python;
///
/// Python::with_gil(|py| {
///    run_bpy_code(py, "print(f'Blender version: {bpy.app.version}')").unwrap();
/// });
/// ```
///
/// ```no_compile
/// use blr::utils::python::run_bpy_code;
/// use pyo3::Python;
///
/// let code = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/python/test.py"));
///
/// Python::with_gil(|py| {
///   run_bpy_code(py, code).unwrap();
/// });
/// ```
pub fn run_bpy_code(py: Python, code: &str) -> Result<()> {
    // Ensure that `bpy` module is in globals
    let globals = PyDict::new(py);
    globals.set_item("bpy", PyModule::import(py, "bpy")?)?;

    // Run the code
    py.run(code, Some(globals), None)?;

    Ok(())
}

/// Run a Python script that uses the Python API of Blender (`bpy`).
///
/// The script is read from the given file path at runtime.
///
/// # Examples
///
/// ```no_run
/// use blr::utils::python::run_bpy_script;
/// use pyo3::Python;
///
/// Python::with_gil(|py| {
///    run_bpy_script(py, &"test.py").unwrap();
/// });
/// ```
pub fn run_bpy_script(py: Python, filepath: impl AsRef<Path>) -> Result<()> {
    let filepath = filepath.as_ref();
    if !filepath.is_file() {
        return Err(BlError::ValueError(format!(
            "Filepath '{}' does not point to a valid file.",
            filepath.display()
        )));
    }

    run_bpy_code(py, &read_to_string(filepath)?)
}
