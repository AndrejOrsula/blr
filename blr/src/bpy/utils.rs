//! Bindings for [`bpy.utils`](https://docs.blender.org/api/latest/bpy.utils.html).
use super::{bind_python, Path, PathBuf, PyAny};

bind_python! { bpy.utils.register_class() => pub fn register_class(py: Python, cls: &PyAny) }
bind_python! { bpy.utils.unregister_class() => pub fn unregister_class(py: Python, cls: &PyAny) }
bind_python! { bpy.utils.manual_map() => pub fn manual_map(py: Python) -> Result<&PyAny> }
bind_python! { bpy.utils.register_manual_map() => pub fn register_manual_map(py: Python, manual_hook: &PyAny) }
bind_python! { bpy.utils.unregister_manual_map() => pub fn unregister_manual_map(py: Python, manual_hook: &PyAny) }
bind_python! { bpy.utils.register_classes_factory() => pub fn register_classes_factory(py: Python, classes: &PyAny) }
bind_python! { bpy.utils.register_submodule_factory() => pub fn register_submodule_factory<'py>(py: Python<'py>, module_name: &str, submodule_names: Vec<&str>) -> Result<(&'py PyAny, &'py PyAny)> }
bind_python! { bpy.utils.register_tool() => pub fn register_tool(py: Python, tool_cls: &PyAny, separator: bool, group: bool) }
bind_python! { bpy.utils.unregister_tool() => pub fn unregister_tool(py: Python, tool_cls: &PyAny) }
bind_python! { bpy.utils.blend_paths() => pub fn blend_paths(py: Python) -> Result<Vec<PathBuf>> }
bind_python! { bpy.utils.resource_path() => pub fn resource_path(py: Python, r#type: &str) -> Result<PathBuf> }
bind_python! { bpy.utils.user_resource() => pub fn user_resource(py: Python, resource_type: &str) -> Result<PathBuf> }
bind_python! { bpy.utils.system_resource() => pub fn system_resource(py: Python, r#type: &str) -> Result<PathBuf> }
bind_python! { bpy.utils.is_path_builtin() => pub fn is_path_builtin(py: Python, path: &Path) -> Result<bool> }
bind_python! { bpy.utils.modules_from_path() => pub fn modules_from_path<'py>(py: Python<'py>, path: &Path) -> Result<Vec<&'py PyAny>> }
bind_python! { bpy.utils.load_scripts() => pub fn load_scripts(py: Python, reload_scripts: bool, refresh_scripts: bool) }
bind_python! { bpy.utils.script_paths() => pub fn script_paths(py: Python) -> Result<Vec<PathBuf>> }
bind_python! { bpy.utils.script_path_user() => pub fn script_path_user(py: Python) -> Result<Vec<PathBuf>> }
bind_python! { bpy.utils.script_paths_pref() => pub fn script_paths_pref(py: Python) -> Result<Vec<PathBuf>> }
bind_python! { bpy.utils.refresh_script_paths() => pub fn refresh_script_paths(py: Python) }
bind_python! { bpy.utils.execfile() => pub fn execfile<'py>(py: Python<'py>, filepath: &Path, r#mod: &'py PyAny) -> Result<&'py PyAny> }
