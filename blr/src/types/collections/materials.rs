use super::CollectionImpl;
use crate::types::Material;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.BlendDataMaterials.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Materials(PyObject);

impl<'py> CollectionImpl<'py> for Materials {
    type Item = Material;
}

impl Materials {
    bind_python! { self.new() => pub fn new(&self, py: Python, name: &str) -> Result<Material> }
    bind_python! { self.create_gpencil_data() => pub fn create_gpencil_data(&self, py: Python, material: &Material) }
    bind_python! { self.remove_gpencil_data() => pub fn remove_gpencil_data(&self, py: Python, material: &Material) }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, material: &Material, do_unlink: bool, do_id_user: bool, do_ui_user: bool) }
    bind_python! { self.tag() => pub fn tag(&self, py: Python, value: bool) }
}

impl From<pyo3::PyObject> for Materials {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Materials {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Materials {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
