use super::Object;
use crate::result::Result;
use derive_more::{Deref, DerefMut, Display};
use pyo3::Python;

/// Wrapper for Empty (identical to Object)
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Empty(pub(crate) Object);

impl Empty {
    pub fn try_from_active(py: Python) -> Result<Self> {
        Object::from_active(py)?.try_into()
    }
}

impl From<pyo3::PyObject> for Empty {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value.into())
    }
}

impl From<&pyo3::PyAny> for Empty {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Empty {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Empty {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
