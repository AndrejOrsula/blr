use crate::{
    enums::Dtype,
    result::Result,
    types::{BpyID, Node},
};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyAny, PyObject, Python, ToPyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.NodeSocket.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodeSocket(PyObject);

impl BpyID for NodeSocket {}

impl NodeSocket {
    pub fn data_type(&self, py: Python) -> Result<Dtype> {
        Ok(self.getattr(py, intern!(py, "type"))?.extract(py)?)
    }
    pub fn set_data_type(&mut self, py: Python, value: Dtype) -> Result<()> {
        self.setattr(py, intern!(py, "type"), value.to_object(py))?;
        Ok(())
    }

    bind_python! { self.bl_idname => pub fn bl_idname(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_idname = pub fn set_bl_idname(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_label => pub fn bl_label(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_label = pub fn set_bl_label(&mut self, py: Python, value: &str) }
    bind_python! { self.bl_subtype_label => pub fn bl_subtype_label(&self, py: Python) -> Result<String> }
    bind_python! { self.bl_subtype_label = pub fn set_bl_subtype_label(&mut self, py: Python, value: &str) }
    bind_python! { self.description => pub fn description(&self, py: Python) -> Result<String> }
    bind_python! { self.description = pub fn set_description(&mut self, py: Python, value: &str) }
    bind_python! { self.display_shape => pub fn display_shape(&self, py: Python) -> Result<String> }
    bind_python! { self.display_shape = pub fn set_display_shape(&mut self, py: Python, value: &str) }
    bind_python! { self.enabled => pub fn enabled(&self, py: Python) -> Result<bool> }
    bind_python! { self.enabled = pub fn set_enabled(&mut self, py: Python, value: bool) }
    bind_python! { self.hide => pub fn hide(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide = pub fn set_hide(&mut self, py: Python, value: bool) }
    bind_python! { self.hide_value => pub fn hide_value(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_value = pub fn set_hide_value(&mut self, py: Python, value: bool) }
    bind_python! { self.identifier => pub fn identifier(&self, py: Python) -> Result<String> }
    bind_python! { self.is_linked => pub fn is_linked(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_multi_input => pub fn is_multi_input(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_output => pub fn is_output(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_unavailable => pub fn is_unavailable(&self, py: Python) -> Result<bool> }
    bind_python! { self.label => pub fn label(&self, py: Python) -> Result<String> }
    bind_python! { self.link_limit => pub fn link_limit(&self, py: Python) -> Result<u16> }
    bind_python! { self.name => pub fn name(&self, py: Python) -> Result<String> }
    bind_python! { self.name = pub fn set_name(&mut self, py: Python, value: &str) }
    bind_python! { self.node => pub fn node(&self, py: Python) -> Result<Node> }
    bind_python! { self.show_expanded => pub fn show_expanded(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_expanded = pub fn set_show_expanded(&mut self, py: Python, value: bool) }
    bind_python! { self.links => pub fn links<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.draw() => pub fn draw(&self, py: Python, context: &PyAny, layout: &PyAny, node: &PyAny, text: &PyAny) }
    bind_python! { self.draw_color() => pub fn draw_color(&self, py: Python, context: &PyAny, node: &PyAny) -> Result<[f32; 4]> }
}

impl From<pyo3::PyObject> for NodeSocket {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodeSocket {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodeSocket {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for NodeSocket {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
