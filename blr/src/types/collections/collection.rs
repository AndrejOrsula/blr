use super::CollectionImpl;
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyAny, PyObject};

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.bpy_prop_collection.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Collection(PyObject);

impl<'py> CollectionImpl<'py> for Collection {
    type Item = &'py PyAny;
}

impl From<pyo3::PyObject> for Collection {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Collection {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Collection {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
