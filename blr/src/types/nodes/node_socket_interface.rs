use crate::{
    enums::Dtype,
    result::Result,
    types::{
        nodes::{Node, NodeSocket},
        BpyID,
    },
};

use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyAny, PyObject, Python, ToPyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.NodeSocketInterface.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeSocketInterface(PyObject);

impl BpyID for NodeSocketInterface {}

impl NodeSocketInterface {
    pub fn data_type(&self, py: Python) -> Result<Dtype> {
        Ok(self.getattr(py, intern!(py, "type"))?.extract(py)?)
    }
    pub fn set_data_type(&mut self, py: Python, value: Dtype) -> Result<()> {
        self.setattr(py, intern!(py, "type"), value.to_object(py))?;
        Ok(())
    }

    bind_python! { self.attribute_domain => pub fn attribute_domain(&self, py: Python) -> Result<String> }
    bind_python! { self.attribute_domain = pub fn set_attribute_domain(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_label => pub fn bl_label(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_label = pub fn set_bl_label(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_socket_idname => pub fn bl_socket_idname(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_socket_idname = pub fn set_bl_socket_idname(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_subtype_label => pub fn bl_subtype_label(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_subtype_label = pub fn set_bl_subtype_label(&mut self, py: Python, value: &str) }
    bind_python! { self.default_attribute_name => pub fn default_attribute_name(&self, py: Python) -> Result<String> }
    bind_python! { self.default_attribute_name = pub fn set_default_attribute_name(&mut self, py: Python, value: &str) }
    bind_python! { self.description => pub fn description(&self, py: Python) -> Result<String> }
    bind_python! { self.description = pub fn set_description(&mut self, py: Python, value: &str) }
    bind_python! { self.hide_in_modifier => pub fn hide_in_modifier(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_in_modifier = pub fn set_hide_in_modifier(&mut self, py: Python, value: bool) }
    bind_python! { self.hide_value => pub fn hide_value(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_value = pub fn set_hide_value(&mut self, py: Python, value: bool) }
    bind_python! { self.identifier => pub fn identifier(&self, py: Python) -> Result<String> }
    bind_python! { self.is_output => pub fn is_output(&self, py: Python) -> Result<bool> }
    bind_python! { self.name => pub fn name(&self, py: Python) -> Result<String> }
    bind_python! { self.name = pub fn set_name(&mut self, py: Python, value: &str) }
    bind_python! { self.draw() => pub fn draw(&self, py: Python, context: &PyAny, layout: &PyAny) }
    bind_python! { self.draw_color() => pub fn draw_color(&self, py: Python, context: &PyAny) -> Result<[f32; 4]> }
    bind_python! { self.init_socket() => pub fn init_socket(&self, py: Python, node: Node, socket: NodeSocket, data_path: String) }
    bind_python! { self.from_socket() => pub fn from_socket(&self, py: Python, node: Node, socket: NodeSocket) }
}

impl From<pyo3::PyObject> for NodeSocketInterface {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeSocketInterface {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeSocketInterface {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for NodeSocketInterface {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
