use crate::{bpy, error::BlError, result::Result};
use pyo3::Python;

/// Autodetects and sets the path of the Blender binary to make it accessible within `bpy` module
/// via `bpy.app.binary_path`.
///
/// # Errors
///
/// Returns error if `which blender` is unable to find the path of the Blender binary.
pub fn try_autoupdate_app_binary_path(py: Python) -> Result<()> {
    if let Ok(cmd) = which::which("blender") {
        Ok(bpy::app::set_binary_path(py, &cmd)?)
    } else {
        Err(BlError::DependencyError(
            "Unable to find the path of the Blender binary".to_string(),
        ))
    }
}
