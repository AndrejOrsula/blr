use crate::{bpy, result::Result};
use pyo3::Python;
use semver::Version;

/// Returns the version of the Python module of Blender (`bpy`).
pub fn bpy_version(py: Python) -> Result<Version> {
    let bpy_version = bpy::app::version(py)?;
    Ok(Version::new(
        u64::from(bpy_version[0]),
        u64::from(bpy_version[1]),
        u64::from(bpy_version[2]),
    ))
}

/// Returns the major version of the Python module of Blender (`bpy`).
pub fn bpy_version_major(py: Python) -> Result<u8> {
    Ok(bpy::app::version(py)?[0])
}

/// Returns the minor version of the Python module of Blender (`bpy`).
pub fn bpy_version_minor(py: Python) -> Result<u8> {
    Ok(bpy::app::version(py)?[1])
}

/// Returns the patch (version) of the Python module of Blender (`bpy`).
pub fn bpy_version_patch(py: Python) -> Result<u8> {
    Ok(bpy::app::version(py)?[2])
}
