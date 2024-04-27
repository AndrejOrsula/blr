use super::CollectionImpl;
use crate::types::nodes::NodeTree;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.BlendDataNodeTrees.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeTrees(PyObject);

impl<'py> CollectionImpl<'py> for NodeTrees {
    type Item = NodeTree;
}

impl NodeTrees {
    bind_python! { self.new() => pub fn new(&self, py: Python, r#type: &str) -> Result<NodeTree> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, tree: &NodeTree, do_unlink: bool, do_id_user: bool, do_ui_user: bool) }
    bind_python! { self.tag() => pub fn tag(&self, py: Python, value: bool) }
}

impl From<pyo3::PyObject> for NodeTrees {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeTrees {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeTrees {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
