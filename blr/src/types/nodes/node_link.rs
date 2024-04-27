use super::{Node, NodeSocket};
use crate::types::BpyID;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.NodeLink.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeLink(PyObject);

impl BpyID for NodeLink {}

impl NodeLink {
    bind_python! { self.from_node => pub fn from_node(&self, py: Python) -> Result<Node> }
    bind_python! { self.from_socket => pub fn from_socket(&self, py: Python) -> Result<NodeSocket> }
    bind_python! { self.is_hidden => pub fn is_hidden(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_muted => pub fn is_muted(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_muted = pub fn set_is_muted(&mut self, py: Python, value: bool) }
    bind_python! { self.is_valid => pub fn is_valid(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_valid = pub fn set_is_valid(&mut self, py: Python, value: bool) }
    bind_python! { self.to_node => pub fn to_node(&self, py: Python) -> Result<Node> }
    bind_python! { self.to_socket => pub fn to_socket(&self, py: Python) -> Result<NodeSocket> }
}

impl From<pyo3::PyObject> for NodeLink {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeLink {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeLink {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for NodeLink {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
