use super::CollectionImpl;
use crate::types::nodes::{NodeLink, NodeSocket};
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.NodeLinks.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeLinks(PyObject);

impl<'py> CollectionImpl<'py> for NodeLinks {
    type Item = NodeLink;
}

impl NodeLinks {
    bind_python! { self.new() => pub fn new(&self, py: Python, input: &NodeSocket, output: &NodeSocket, verify_limits: bool) -> Result<NodeLink> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, link: &NodeLink) }
    bind_python! { self.clear() => pub fn clear(&self, py: Python) }
}

impl From<pyo3::PyObject> for NodeLinks {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeLinks {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeLinks {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
