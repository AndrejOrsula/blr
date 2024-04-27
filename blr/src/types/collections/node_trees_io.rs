use super::CollectionImpl;
use crate::{enums::Dtype, result::Result, types::nodes::NodeSocketInterface};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyObject, Python};
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.NodeTreeInputs.html>
pub type NodeTreeInputs = NodeTreeIO;
/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.NodeTreeOutputs.html>
pub type NodeTreeOutputs = NodeTreeIO;

#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeTreeIO(PyObject);

impl<'py> CollectionImpl<'py> for NodeTreeIO {
    type Item = NodeSocketInterface;
}

impl NodeTreeIO {
    pub fn r#move(&self, py: Python, from_index: usize, to_index: usize) -> Result<()> {
        self.as_ref(py)
            .call_method1(intern!(py, "move"), (from_index, to_index))?;
        Ok(())
    }

    bind_python! { self.new() => pub fn new(&self, py: Python, r#type: Dtype, name: &str) -> Result<NodeSocketInterface> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, link: &NodeSocketInterface) }
    bind_python! { self.clear() => pub fn clear(&self, py: Python) }
}

impl From<pyo3::PyObject> for NodeTreeIO {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeTreeIO {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeTreeIO {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
