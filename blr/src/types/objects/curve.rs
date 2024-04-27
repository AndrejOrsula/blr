use crate::types::{BpyID, Collection, Object};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyAny, PyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Curve.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Curve(PyObject);

impl BpyID for Curve {}
impl CurveImpl for Curve {}

pub trait CurveImpl:
    std::ops::Deref<Target = PyObject> + std::ops::DerefMut<Target = PyObject>
{
    bind_python! { self.animation_data => fn animation_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.bevel_depth => fn bevel_depth(&self, py: Python) -> Result<f32> }
    bind_python! { self.bevel_depth = fn set_bevel_depth(&mut self, py: Python, value: f32) }
    bind_python! { self.bevel_factor_end => fn bevel_factor_end(&self, py: Python) -> Result<f32> }
    bind_python! { self.bevel_factor_end = fn set_bevel_factor_end(&mut self, py: Python, value: f32) }
    bind_python! { self.bevel_factor_mapping_end => fn bevel_factor_mapping_end(&self, py: Python) -> Result<String> }
    bind_python! { self.bevel_factor_mapping_end = fn set_bevel_factor_mapping_end(&mut self, py: Python, value: &str) }
    bind_python! { self.bevel_factor_mapping_start => fn bevel_factor_mapping_start(&self, py: Python) -> Result<String> }
    bind_python! { self.bevel_factor_mapping_start = fn set_bevel_factor_mapping_start(&mut self, py: Python, value: &str) }
    bind_python! { self.bevel_factor_start => fn bevel_factor_start(&self, py: Python) -> Result<f32> }
    bind_python! { self.bevel_factor_start = fn set_bevel_factor_start(&mut self, py: Python, value: f32) }
    bind_python! { self.bevel_mode => fn bevel_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.bevel_mode = fn set_bevel_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.bevel_object => fn bevel_object(&self, py: Python) -> Result<Object> }
    bind_python! { self.bevel_object = fn set_bevel_object(&mut self, py: Python, value: Object) }
    bind_python! { self.bevel_profile => fn bevel_profile(&self, py: Python) -> Result<String> }
    bind_python! { self.bevel_resolution => fn bevel_resolution(&self, py: Python) -> Result<u8> }
    bind_python! { self.bevel_resolution = fn set_bevel_resolution(&mut self, py: Python, value: u8) }
    bind_python! { self.cycles => fn cycles<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.dimensions => fn dimensions(&self, py: Python) -> Result<String> }
    bind_python! { self.dimensions = fn set_dimensions(&mut self, py: Python, value: &str) }
    bind_python! { self.eval_time => fn eval_time(&self, py: Python) -> Result<f32> }
    bind_python! { self.eval_time = fn set_eval_time(&mut self, py: Python, value: f32) }
    bind_python! { self.extrude => fn extrude(&self, py: Python) -> Result<f32> }
    bind_python! { self.extrude = fn set_extrude(&mut self, py: Python, value: f32) }
    bind_python! { self.fill_mode => fn fill_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.fill_mode = fn set_fill_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.is_editmode => fn is_editmode(&self, py: Python) -> Result<bool> }
    bind_python! { self.materials => fn materials(&self, py: Python) -> Result<Collection> }
    bind_python! { self.offset => fn offset(&self, py: Python) -> Result<f32> }
    bind_python! { self.offset = fn set_offset(&mut self, py: Python, value: f32) }
    bind_python! { self.path_duration => fn path_duration(&self, py: Python) -> Result<u32> }
    bind_python! { self.path_duration = fn set_path_duration(&mut self, py: Python, value: u32) }
    bind_python! { self.render_resolution_u => fn render_resolution_u(&self, py: Python) -> Result<u32> }
    bind_python! { self.render_resolution_u = fn set_render_resolution_u(&mut self, py: Python, value: u32) }
    bind_python! { self.render_resolution_v => fn render_resolution_v(&self, py: Python) -> Result<u32> }
    bind_python! { self.render_resolution_v = fn set_render_resolution_v(&mut self, py: Python, value: u32) }
    bind_python! { self.resolution_u => fn resolution_u(&self, py: Python) -> Result<u16> }
    bind_python! { self.resolution_u = fn set_resolution_u(&mut self, py: Python, value: u16) }
    bind_python! { self.resolution_v => fn resolution_v(&self, py: Python) -> Result<u16> }
    bind_python! { self.resolution_v = fn set_resolution_v(&mut self, py: Python, value: u16) }
    bind_python! { self.shape_keys => fn shape_keys<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.splines => fn splines(&self, py: Python) -> Result<Collection> }
    bind_python! { self.taper_object => fn taper_object(&self, py: Python) -> Result<Object> }
    bind_python! { self.taper_object = fn set_taper_object(&mut self, py: Python, value: Object) }
    bind_python! { self.taper_radius_mode => fn taper_radius_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.taper_radius_mode = fn set_taper_radius_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.texspace_location => fn texspace_location(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.texspace_location = fn set_texspace_location(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.texspace_size => fn texspace_size(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.texspace_size = fn set_texspace_size(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.twist_mode => fn twist_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.twist_mode = fn set_twist_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.twist_smooth => fn twist_smooth(&self, py: Python) -> Result<f32> }
    bind_python! { self.twist_smooth = fn set_twist_smooth(&mut self, py: Python, value: f32) }
    bind_python! { self.use_auto_texspace => fn use_auto_texspace(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_auto_texspace = fn set_use_auto_texspace(&mut self, py: Python, value: bool) }
    bind_python! { self.use_deform_bounds => fn use_deform_bounds(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_deform_bounds = fn set_use_deform_bounds(&mut self, py: Python, value: bool) }
    bind_python! { self.use_fill_caps => fn use_fill_caps(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_fill_caps = fn set_use_fill_caps(&mut self, py: Python, value: bool) }
    bind_python! { self.use_map_taper => fn use_map_taper(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_map_taper = fn set_use_map_taper(&mut self, py: Python, value: bool) }
    bind_python! { self.use_path => fn use_path(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_path = fn set_use_path(&mut self, py: Python, value: bool) }
    bind_python! { self.use_path_clamp => fn use_path_clamp(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_path_clamp = fn set_use_path_clamp(&mut self, py: Python, value: bool) }
    bind_python! { self.use_path_follow => fn use_path_follow(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_path_follow = fn set_use_path_follow(&mut self, py: Python, value: bool) }
    bind_python! { self.use_radius => fn use_radius(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_radius = fn set_use_radius(&mut self, py: Python, value: bool) }
    bind_python! { self.use_stretch => fn use_stretch(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_stretch = fn set_use_stretch(&mut self, py: Python, value: bool) }
    bind_python! { self.transform() => fn transform(&self, py: Python, matrix: [[f32; 4]; 4], shape_keys: bool) }
    bind_python! { self.validate_material_indices() => fn validate_material_indices(&self, py: Python) -> Result<bool> }
    bind_python! { self.update_gpu_tag() => fn update_gpu_tag(&self, py: Python) }
}

impl From<pyo3::PyObject> for Curve {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Curve {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Curve {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Curve {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
