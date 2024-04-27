use super::{ModifierImpl, ModifierType};
use crate::types::BpyStruct;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.VertexWeightEditModifier.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct VertexWeightEditModifier(PyObject);

impl BpyStruct for VertexWeightEditModifier {}
impl ModifierImpl for VertexWeightEditModifier {
    const MODIFIER_TYPE: ModifierType = ModifierType::VertexWeightEdit;
}

impl VertexWeightEditModifier {
    bind_python! {}
}

impl From<pyo3::PyObject> for VertexWeightEditModifier {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for VertexWeightEditModifier {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for VertexWeightEditModifier {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for VertexWeightEditModifier {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
