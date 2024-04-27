use crate::types::BpyID;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Curves.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Curves(PyObject);

impl BpyID for Curves {}

impl Curves {
    bind_python! {}
}

impl From<pyo3::PyObject> for Curves {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Curves {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Curves {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Curves {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
