use super::{ModifierImpl, ModifierType};
use crate::types::BpyStruct;
use derive_more::{Deref, DerefMut, Display};
use pyo3::PyObject;
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.DecimateModifier.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct DecimateModifier(PyObject);

impl BpyStruct for DecimateModifier {}
impl ModifierImpl for DecimateModifier {
    const MODIFIER_TYPE: ModifierType = ModifierType::Decimate;
}

impl DecimateModifier {
    bind_python! { self.angle_limit => pub fn angle_limit(&self, py: Python) -> Result<f64> }
    bind_python! { self.angle_limit = pub fn set_angle_limit(&mut self, py: Python, value: f64) }
    bind_python! { self.decimate_type => pub fn decimate_type(&self, py: Python) -> Result<String> }
    bind_python! { self.decimate_type = pub fn set_decimate_type(&mut self, py: Python, value: &str) }
    bind_python! { self.delimit => pub fn delimit(&self, py: Python) -> Result<String> }
    bind_python! { self.delimit = pub fn set_delimit(&mut self, py: Python, value: &str) }
    bind_python! { self.face_count => pub fn face_count(&self, py: Python) -> Result<usize> }
    bind_python! { self.invert_vertex_group => pub fn invert_vertex_group(&self, py: Python) -> Result<bool> }
    bind_python! { self.invert_vertex_group = pub fn set_invert_vertex_group(&mut self, py: Python, value: bool) }
    bind_python! { self.iterations => pub fn iterations(&self, py: Python) -> Result<usize> }
    bind_python! { self.iterations = pub fn set_iterations(&mut self, py: Python, value: usize) }
    bind_python! { self.ratio => pub fn ratio(&self, py: Python) -> Result<f64> }
    bind_python! { self.ratio = pub fn set_ratio(&mut self, py: Python, value: f64) }
    bind_python! { self.symmetry_axis => pub fn symmetry_axis(&self, py: Python) -> Result<String> }
    bind_python! { self.symmetry_axis = pub fn set_symmetry_axis(&mut self, py: Python, value: &str) }
    bind_python! { self.use_collapse_triangulate => pub fn use_collapse_triangulate(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_collapse_triangulate = pub fn set_use_collapse_triangulate(&mut self, py: Python, value: bool) }
    bind_python! { self.use_dissolve_boundaries => pub fn use_dissolve_boundaries(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_dissolve_boundaries = pub fn set_use_dissolve_boundaries(&mut self, py: Python, value: bool) }
    bind_python! { self.use_symmetry => pub fn use_symmetry(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_symmetry = pub fn set_use_symmetry(&mut self, py: Python, value: bool) }
    bind_python! { self.vertex_group => pub fn vertex_group(&self, py: Python) -> Result<String> }
    bind_python! { self.vertex_group = pub fn set_vertex_group(&mut self, py: Python, value: &str) }
    bind_python! { self.vergex_group_factor => pub fn vergex_group_factor(&self, py: Python) -> Result<f64> }
    bind_python! { self.vergex_group_factor = pub fn set_vergex_group_factor(&mut self, py: Python, value: f64) }
}

impl From<pyo3::PyObject> for DecimateModifier {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for DecimateModifier {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for DecimateModifier {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for DecimateModifier {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
