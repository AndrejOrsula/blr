use crate::types::{Scene, ViewLayer};
use pyo3::{PyAny, PyObject};
use pyo3_macros_more::bind_python;
use std::{
    collections::HashSet,
    ops::{Deref, DerefMut},
};

/// Wrapper all subclasses of around <https://docs.blender.org/api/latest/bpy.types.ID.html>
pub trait BpyID: Deref<Target = PyObject> + DerefMut<Target = PyObject> + pyo3::ToPyObject {
    bind_python! { self.asset_data => fn asset_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.is_embedded_data => fn is_embedded_data(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_evaluated => fn is_evaluated(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_library_indirect => fn is_library_indirect(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_missing => fn is_missing(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_runtime_data => fn is_runtime_data(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_runtime_data = fn set_is_runtime_data(&mut self, py: Python, value: bool) }
    bind_python! { self.library => fn library<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.library_weak_reference => fn library_weak_reference<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.name => fn name(&self, py: Python) -> Result<String> }
    bind_python! { self.name = fn set_name(&mut self, py: Python, value: &str) }
    bind_python! { self.name_full => fn name_full(&self, py: Python) -> Result<String> }
    bind_python! { self.original => fn original<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.override_library => fn override_library<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.preview => fn preview<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.tag => fn tag(&self, py: Python) -> Result<bool> }
    bind_python! { self.tag = fn set_tag(&mut self, py: Python, value: bool) }
    bind_python! { self.use_extra_user => fn use_extra_user(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_extra_user = fn set_use_extra_user(&mut self, py: Python, value: bool) }
    bind_python! { self.use_fake_user => fn use_fake_user(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_fake_user = fn set_use_fake_user(&mut self, py: Python, value: bool) }
    bind_python! { self.users => fn users(&self, py: Python) -> Result<u32> }
    bind_python! { self.evaluated_get() => fn evaluated_get<'py>(&'py self, py: Python<'py>, depsgraph: &PyAny) -> Result<&'py PyAny> }
    bind_python! { self.copy() => fn copy<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.asset_mark() => fn asset_mark(&self, py: Python) }
    bind_python! { self.asset_clear() => fn asset_clear(&self, py: Python) }
    bind_python! { self.asset_generate_preview() => fn asset_generate_preview(&self, py: Python) }
    bind_python! { self.override_create() => fn override_create<'py>(&'py self, py: Python<'py>, remap_local_usages: bool) -> Result<&'py PyAny> }
    bind_python! { self.override_hierarchy_create() => fn override_hierarchy_create<'py>(&'py self, py: Python<'py>, scene: Scene, view_layer: ViewLayer, reference: Option<impl BpyID>, do_fully_editable: bool) -> Result<&'py PyAny> }
    bind_python! { self.override_template_create() => fn override_template_create(&self, py: Python)}
    bind_python! { self.user_clear() => fn user_clear(&self, py: Python)}
    bind_python! { self.user_remap() => fn user_remap(&self, py: Python, new_id: impl BpyID) }
    bind_python! { self.make_local() => fn make_local<'py>(&'py self, py: Python<'py>, clear_proxy: bool) -> Result<&'py PyAny> }
    bind_python! { self.user_of_id() => fn user_of_id(&self, py: Python, id: impl BpyID) -> Result<u32> }
    bind_python! { self.animation_data_create() => fn animation_data_create<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.animation_data_clear() => fn animation_data_clear(&self, py: Python) }
    bind_python! { self.update_tag() => fn update_tag(&self, py: Python, refresh: HashSet<String>) }
    bind_python! { self.preview_ensure() => fn preview_ensure<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
}
