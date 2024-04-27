use super::{ModifierImpl, ModifierType};
use crate::{
    bpy,
    enums::Dtype,
    result::Result,
    types::{BpyStruct, CollectionImpl, NodeTree},
};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyAny, PyObject, Python, ToPyObject};
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.NodesModifier.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct NodesModifier(PyObject);

impl BpyStruct for NodesModifier {}
impl ModifierImpl for NodesModifier {
    const MODIFIER_TYPE: ModifierType = ModifierType::Nodes;
}

impl NodesModifier {
    pub fn set_node_group_to(&mut self, py: Python, node_tree: &str) -> Result<()> {
        self.set_node_group(py, &bpy::data::node_groups(py)?.get(py, node_tree)?)?;
        Ok(())
    }

    pub fn get_input_attribute_type(&self, py: Python, key: &str) -> Result<Dtype> {
        let inputs = self.node_group(py)?.inputs(py)?;
        inputs.get(py, key)?.data_type(py)
    }

    pub fn get_input_attribute<'py>(&'py self, py: Python<'py>, key: &str) -> Result<&PyAny> {
        let inputs = self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        Ok(self.get(py, &attribute_identifier)?)
    }

    pub fn get_input_attribute_name(&self, py: Python, key: &str) -> Result<String> {
        let inputs: crate::collections::node_trees_io::NodeTreeIO =
            self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        Ok(self
            .get(py, &format!("{attribute_identifier}_attribute_name"))?
            .extract()?)
    }

    pub fn get_input_use_attribute(&self, py: Python, key: &str) -> Result<bool> {
        let inputs = self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        Ok(self
            .get(py, &format!("{attribute_identifier}_use_attribute"))?
            .extract()?)
    }

    pub fn set_input_attribute_type(&self, py: Python, key: &str, value: Dtype) -> Result<()> {
        let inputs = self.node_group(py)?.inputs(py)?;
        inputs.get(py, key)?.set_data_type(py, value)
    }

    pub fn set_input_attribute(&self, py: Python, key: &str, value: impl ToPyObject) -> Result<()> {
        let inputs = self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        self.call_method1(
            py,
            intern!(py, "__setitem__"),
            (attribute_identifier, value.to_object(py)),
        )?;
        Ok(())
    }

    pub fn set_input_attribute_name(&self, py: Python, key: &str, value: &str) -> Result<()> {
        let inputs = self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        self.call_method1(
            py,
            intern!(py, "__setitem__"),
            (format!("{attribute_identifier}_attribute_name"), value),
        )?;
        Ok(())
    }

    pub fn set_input_use_attribute(&self, py: Python, key: &str, value: bool) -> Result<()> {
        let inputs = self.node_group(py)?.inputs(py)?;
        let attribute_identifier = inputs.get(py, key)?.identifier(py)?;
        self.call_method1(
            py,
            intern!(py, "__setitem__"),
            (format!("{attribute_identifier}_use_attribute"), value),
        )?;
        Ok(())
    }

    pub fn get_output_attribute_type(&self, py: Python, key: &str) -> Result<Dtype> {
        let outputs = self.node_group(py)?.outputs(py)?;
        outputs.get(py, key)?.data_type(py)
    }

    pub fn get_output_attribute_name(&self, py: Python, key: &str) -> Result<String> {
        let outputs = self.node_group(py)?.outputs(py)?;
        let attribute_identifier = outputs.get(py, key)?.identifier(py)?;
        Ok(self
            .get(py, &format!("{attribute_identifier}_attribute_name"))?
            .extract()?)
    }

    pub fn set_output_attribute_type(&self, py: Python, key: &str, value: Dtype) -> Result<()> {
        let outputs = self.node_group(py)?.outputs(py)?;
        outputs.get(py, key)?.set_data_type(py, value)
    }

    pub fn set_output_attribute_name(&self, py: Python, key: &str, value: &str) -> Result<()> {
        let outputs = self.node_group(py)?.outputs(py)?;
        let attribute_identifier = outputs.get(py, key)?.identifier(py)?;
        self.call_method1(
            py,
            intern!(py, "__setitem__"),
            (format!("{attribute_identifier}_attribute_name"), value),
        )?;
        Ok(())
    }

    bind_python! { self.node_group => pub fn node_group(&self, py: Python) -> Result<NodeTree> }
    bind_python! { self.node_group = pub fn set_node_group(&mut self, py: Python, value: &NodeTree) }
    bind_python! { self.simulation_bake_directory => pub fn simulation_bake_directory(&self, py: Python) -> Result<String> }
    bind_python! { self.simulation_bake_directory = pub fn set_simulation_bake_directory(&mut self, py: Python, value: &str) }
    bind_python! { self.items() => pub fn items<'py>(&'py self, py: Python<'py>) -> Result<Vec<(String, &'py PyAny)>> }
    bind_python! { self.keys() => pub fn keys(&self, py: Python) -> Result<Vec<String>> }
    bind_python! { self.values() => pub fn values<'py>(&'py self, py: Python<'py>) -> Result<Vec<&'py PyAny>> }
}

impl From<pyo3::PyObject> for NodesModifier {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for NodesModifier {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for NodesModifier {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for NodesModifier {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
