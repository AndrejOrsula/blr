use super::{ModifierImpl, ModifierType};
use crate::types::BpyStruct;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.SimpleDeformModifier.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct SimpleDeformModifier(PyObject);

impl BpyStruct for SimpleDeformModifier {}
impl ModifierImpl for SimpleDeformModifier {
    const MODIFIER_TYPE: ModifierType = ModifierType::SimpleDeform;
}

impl SimpleDeformModifier {
    bind_python! {}
}

impl From<pyo3::PyObject> for SimpleDeformModifier {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for SimpleDeformModifier {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for SimpleDeformModifier {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for SimpleDeformModifier {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
