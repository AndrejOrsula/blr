//! Bindings for [`bpy.app`](https://docs.blender.org/api/latest/bpy.app.html).
use super::{bind_python, Path, PathBuf, PyAny};

bind_python! { bpy.app.binary_path => pub fn binary_path(py: Python) -> Result<PathBuf> }
bind_python! { bpy.app.binary_path = pub fn set_binary_path(py: Python, path: &Path) }
bind_python! { bpy.app.tempdir => pub fn tempdir(py: Python) -> Result<PathBuf> }
bind_python! { bpy.app.background => pub fn background(py: Python) -> Result<bool> }
bind_python! { bpy.app.factory_startup => pub fn factory_startup(py: Python) -> Result<bool> }
bind_python! { bpy.app.version_cycle => pub fn version_cycle(py: Python) -> Result<String> }
bind_python! { bpy.app.version_string => pub fn version_string(py: Python) -> Result<String> }
bind_python! { bpy.app.version => pub fn version(py: Python) -> Result<[u8; 3]> }
bind_python! { bpy.app.version_file => pub fn version_file(py: Python) -> Result<[u8; 3]> }
bind_python! { bpy.app.build_options => pub fn build_options(py: Python) -> Result<&PyAny> }
