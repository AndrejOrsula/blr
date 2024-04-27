use crate::types::BpyID;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Surface.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Surface(PyObject);

impl BpyID for Surface {}

impl Surface {
    bind_python! {}
}

impl From<pyo3::PyObject> for Surface {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Surface {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Surface {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Surface {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
