use crate::{
    enums::NodeTreeType,
    objects::GreasePencil,
    result::Result,
    types::{
        collections::{NodeLinks, NodeTreeInputs, NodeTreeOutputs, Nodes},
        BpyID,
    },
};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyAny, PyObject, Python};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.NodeTree.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeTree(PyObject);

impl BpyID for NodeTree {}

impl NodeTree {
    pub fn node_tree_type(&self, py: Python) -> Result<NodeTreeType> {
        Ok(self.getattr(py, intern!(py, "type"))?.extract(py)?)
    }

    bind_python! { self.active_input => pub fn active_input(&self, py: Python) -> Result<usize> }
    bind_python! { self.active_input = pub fn set_active_input(&mut self, py: Python, value: usize) }
    bind_python! { self.active_output => pub fn active_output(&self, py: Python) -> Result<usize> }
    bind_python! { self.active_output = pub fn set_active_output(&mut self, py: Python, value: usize) }
    bind_python! { self.animation_data => pub fn animation_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.bl_description => pub fn bl_description(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_description = pub fn set_bl_description(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_icon => pub fn bl_icon(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_icon = pub fn set_bl_icon(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_idname => pub fn bl_idname(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_idname = pub fn set_bl_idname(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_label => pub fn bl_label(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_label = pub fn set_bl_label(&mut self, py: Python, value: &str) }
    bind_python! { self.grease_pencil => pub fn grease_pencil(&self, py: Python) -> Result<GreasePencil> }
    bind_python! { self.grease_pencil = pub fn set_grease_pencil(&mut self, py: Python, value: GreasePencil) }
    bind_python! { self.inputs => pub fn inputs(&self, py: Python) -> Result<NodeTreeInputs> }
    bind_python! { self.links => pub fn links(&self, py: Python) -> Result<NodeLinks> }
    bind_python! { self.nodes => pub fn nodes(&self, py: Python) -> Result<Nodes> }
    bind_python! { self.outputs => pub fn outputs(&self, py: Python) -> Result<NodeTreeOutputs> }
    bind_python! { self.view_center => pub fn view_center(&self, py: Python) -> Result<[f32; 2]> }
    bind_python! { self.contains_tree() => pub fn contains_tree(&self, py: Python, sub_tree: &Self) -> Result<bool> }
    bind_python! { self.update() => pub fn update(&self, py: Python) }
}

impl From<pyo3::PyObject> for NodeTree {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeTree {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeTree {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for NodeTree {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
