use super::CollectionImpl;
use crate::types::nodes::Node;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.Nodes.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Nodes(PyObject);

impl<'py> CollectionImpl<'py> for Nodes {
    type Item = Node;
}

impl Nodes {
    bind_python! { self.active => pub fn active(&self, py: Python) -> Result<Node> }
    bind_python! { self.active = pub fn set_active(&mut self, py: Python, value: &Node) }
    bind_python! { self.new() => pub fn new<'py>(&'py mut self, py: Python<'py>, r#type: &str) -> Result<Node> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, node: &Node) }
    bind_python! { self.clear() => pub fn clear(&self, py: Python) }
}

impl From<pyo3::PyObject> for Nodes {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Nodes {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Nodes {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
