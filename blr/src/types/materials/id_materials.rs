use crate::types::{BpyStruct, Material};
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.IDMaterials.html#bpy.types.IDMaterials>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct IdMaterials(PyObject);

impl BpyStruct for IdMaterials {}

impl IdMaterials {
    bind_python! { self.append() => pub fn append(&self, py: Python, material: &Material) }
    bind_python! { self.pop() => pub fn pop(&self, py: Python, index: i16) -> Result<Material> }
    bind_python! { self.clear() => pub fn clear(&self, py: Python) }
}

impl From<pyo3::PyObject> for IdMaterials {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for IdMaterials {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for IdMaterials {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for IdMaterials {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
