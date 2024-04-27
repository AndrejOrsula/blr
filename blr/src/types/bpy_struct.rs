use pyo3::{PyAny, PyObject};
use pyo3_macros_more::bind_python;
use std::ops::{Deref, DerefMut};

/// Wrapper all subclasses of around <https://docs.blender.org/api/latest/bpy.types.bpy_struct.html>
pub trait BpyStruct:
    Deref<Target = PyObject> + DerefMut<Target = PyObject> + pyo3::ToPyObject
{
    bind_python! { self.as_pointer() => fn as_pointer(&self, py: Python) -> Result<usize> }
    bind_python! { self.driver_add() => fn driver_add<'py>(&'py self, py: Python<'py>, path: &str, index: isize) -> Result<&'py PyAny> }
    bind_python! { self.driver_remove() => fn driver_remove(&self, py: Python, path: &str, index: isize) -> Result<bool> }
    bind_python! { self.get() => fn get<'py>(&'py self, py: Python<'py>, key: &str) -> Result<&'py PyAny> }
    bind_python! { self.id_properties_clear() => fn id_properties_clear(&self, py: Python) }
    bind_python! { self.id_properties_ensure() => fn id_properties_ensure<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.id_properties_ui() => fn id_properties_ui<'py>(&'py self, py: Python<'py>, key: &str) -> Result<&'py PyAny> }
    bind_python! { self.is_property_hidden() => fn is_property_hidden(&self, py: Python, property: &PyAny) -> Result<bool> }
    bind_python! { self.is_property_overridable_library() => fn is_property_overridable_library(&self, py: Python, property: &PyAny) -> Result<bool> }
    bind_python! { self.is_property_readonly() => fn is_property_readonly(&self, py: Python, property: &PyAny) -> Result<bool> }
    bind_python! { self.is_property_set() => fn is_property_set(&self, py: Python, property: &PyAny, ghost: bool) -> Result<bool> }
    bind_python! { self.items() => fn items<'py>(&'py self, py: Python<'py>) -> Result<Vec<(String, &'py PyAny)>> }
    bind_python! { self.keyframe_delete() => fn keyframe_delete(&self, py: Python, data_path: &str, index: isize, frame: f32, group: &str) -> Result<bool> }
    bind_python! { self.keyframe_insert() => fn keyframe_insert(&self, py: Python, data_path: &str, index: isize, frame: f32, group: &str, options: &PyAny) -> Result<bool> }
    bind_python! { self.keys() => fn keys(&self, py: Python) -> Result<Vec<String>> }
    bind_python! { self.path_from_id() => fn path_from_id(&self, py: Python, property: &str) -> Result<String> }
    bind_python! { self.path_resolve() => fn path_resolve(&self, py: Python, path: &str, coerce: bool) }
    bind_python! { self.pop() => fn pop<'py>(&'py self, py: Python<'py>, key: &str) -> Result<&'py PyAny> }
    bind_python! { self.property_overridable_library_set() => fn property_overridable_library_set(&self, py: Python, property: &PyAny,overridable: bool)  -> Result<bool> }
    bind_python! { self.property_unset() => fn property_unset(&self, py: Python, property: &PyAny) }
    bind_python! { self.type_recast() => fn type_recast<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.values() => fn values<'py>(&'py self, py: Python<'py>) -> Result<Vec<&'py PyAny>> }
}
