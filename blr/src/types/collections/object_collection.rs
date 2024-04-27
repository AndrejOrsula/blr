use super::CollectionImpl;
use crate::types::{BpyID, Object};
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.BlendDataObjects.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct ObjectCollection(PyObject);

impl<'py> CollectionImpl<'py> for ObjectCollection {
    type Item = Object;
}

impl ObjectCollection {
    bind_python! { self.new() => pub fn new(&self, py: Python, name: &str, object_data: impl BpyID) -> Result<Object> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, object: &Object, do_unlink: bool, do_id_user: bool, do_ui_user: bool) }
    bind_python! { self.tag() => pub fn tag(&self, py: Python, value: bool) }
}

impl From<pyo3::PyObject> for ObjectCollection {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for ObjectCollection {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for ObjectCollection {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
