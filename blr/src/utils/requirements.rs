use crate::{error::BlError, result::Result, utils::version::bpy_version};
use pyo3::{types::PyModule, Python};
use semver::{Version, VersionReq};

/// Verifies that the current environment meets the requirements to use the Python API of Blender.
pub fn verify_bpy_requirements() -> Result<()> {
    const REQ_SEMVER_PYTHON: &str = "3.10";
    const REQ_SEMVER_BPY: &str = ">=3.5";

    // Version of Python
    let req_python = VersionReq::parse(REQ_SEMVER_PYTHON).unwrap();
    let python_version: Version = Python::with_gil(|py| {
        let python_version = py.version_info();
        Version::new(
            u64::from(python_version.major),
            u64::from(python_version.minor),
            u64::from(python_version.patch),
        )
    });
    if !req_python.matches(&python_version) {
        return Err(BlError::DependencyError(format!(
            "The Python module of Blender (`bpy`) requires Python \"{REQ_SEMVER_PYTHON}\" but the current version is \"{python_version}\""
        )));
    }

    // Check if `bpy` module can be imported
    Python::with_gil(|py| -> Result<()> {
        PyModule::import(py, "bpy")?;
        Ok(())
    })
    .map_err(|err| {
        BlError::DependencyError(format!(
            "Unable to import the Python module of Blender (`bpy`): {err}"
        ))
    })?;

    // Version of `bpy` module
    let req_bpy = VersionReq::parse(REQ_SEMVER_BPY).unwrap();
    let bpy_version = pyo3::Python::with_gil(bpy_version)?;
    if !req_bpy.matches(&bpy_version) {
        return Err(BlError::DependencyError(format!(
            "The required version of the Python module of Blender (`bpy`) is \"{REQ_SEMVER_BPY}\" but the current version is \"{bpy_version}\""
        )));
    }

    Ok(())
}
