use crate::{bpy, result::Result, types::collections};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{intern, PyAny, PyObject, Python};
use pyo3_macros_more::bind_python;

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.ViewLayer.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct ViewLayer(PyObject);

impl ViewLayer {
    pub fn new(py: Python) -> Result<Self> {
        bpy::ops::scene::view_layer_add(py, "NEW")?;
        Ok(bpy::context::view_layer(py)?)
    }

    pub fn new_copy(py: Python) -> Result<Self> {
        bpy::ops::scene::view_layer_add(py, "COPY")?;
        Ok(bpy::context::view_layer(py)?)
    }

    pub fn new_empty(py: Python) -> Result<Self> {
        bpy::ops::scene::view_layer_add(py, "EMPTY")?;
        Ok(bpy::context::view_layer(py)?)
    }

    pub fn r#use(&self, py: Python) -> Result<bool> {
        Ok(self.getattr(py, intern!(py, "use"))?.extract(py)?)
    }

    pub fn set_use(&mut self, py: Python, value: bool) -> Result<()> {
        self.setattr(py, intern!(py, "use"), value)?;
        Ok(())
    }

    bind_python! { self.active_aov => pub fn active_aov<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_aov_index => pub fn active_aov_index<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_aov_index = pub fn set_active_aov_index(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.active_layer_collection => pub fn active_layer_collection<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_layer_collection = pub fn set_active_layer_collection(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.active_lightgroup => pub fn active_lightgroup<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_lightgroup_index => pub fn active_lightgroup_index<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_lightgroup_index = pub fn set_active_lightgroup_index(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.aovs => pub fn aovs<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.cycles => pub fn cycles<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.depsgraph => pub fn depsgraph<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.eevee => pub fn eevee<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.freestyle_settings => pub fn freestyle_settings<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.layer_collection => pub fn layer_collection<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lightgroups => pub fn lightgroups<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.material_override => pub fn material_override<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.material_override = pub fn set_material_override(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.name => pub fn name<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.name = pub fn set_name(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.objects => pub fn objects(&self, py: Python) -> Result<collections::ObjectCollection> }
    bind_python! { self.pass_alpha_threshold => pub fn pass_alpha_threshold<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.pass_alpha_threshold = pub fn set_pass_alpha_threshold(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.pass_cryptomatte_depth => pub fn pass_cryptomatte_depth<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.pass_cryptomatte_depth = pub fn set_pass_cryptomatte_depth(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.samples => pub fn samples<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.samples = pub fn set_samples(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_ao => pub fn use_ao<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_ao = pub fn set_use_ao(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_freestyle => pub fn use_freestyle<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_freestyle = pub fn set_use_freestyle(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_motion_blur => pub fn use_motion_blur<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_motion_blur = pub fn set_use_motion_blur(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_ambient_occlusion => pub fn use_pass_ambient_occlusion<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_ambient_occlusion = pub fn set_use_pass_ambient_occlusion(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_combined => pub fn use_pass_combined<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_combined = pub fn set_use_pass_combined(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_cryptomatte_accurate => pub fn use_pass_cryptomatte_accurate<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_cryptomatte_accurate = pub fn set_use_pass_cryptomatte_accurate(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_cryptomatte_asset => pub fn use_pass_cryptomatte_asset<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_cryptomatte_asset = pub fn set_use_pass_cryptomatte_asset(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_cryptomatte_material => pub fn use_pass_cryptomatte_material<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_cryptomatte_material = pub fn set_use_pass_cryptomatte_material(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_cryptomatte_object => pub fn use_pass_cryptomatte_object<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_cryptomatte_object = pub fn set_use_pass_cryptomatte_object(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_diffuse_color => pub fn use_pass_diffuse_color<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_diffuse_color = pub fn set_use_pass_diffuse_color(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_diffuse_direct => pub fn use_pass_diffuse_direct<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_diffuse_direct = pub fn set_use_pass_diffuse_direct(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_diffuse_indirect => pub fn use_pass_diffuse_indirect<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_diffuse_indirect = pub fn set_use_pass_diffuse_indirect(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_emit => pub fn use_pass_emit<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_emit = pub fn set_use_pass_emit(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_environment => pub fn use_pass_environment<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_environment = pub fn set_use_pass_environment(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_glossy_color => pub fn use_pass_glossy_color<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_glossy_color = pub fn set_use_pass_glossy_color(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_glossy_direct => pub fn use_pass_glossy_direct<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_glossy_direct = pub fn set_use_pass_glossy_direct(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_glossy_indirect => pub fn use_pass_glossy_indirect<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_glossy_indirect = pub fn set_use_pass_glossy_indirect(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_material_index => pub fn use_pass_material_index<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_material_index = pub fn set_use_pass_material_index(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_mist => pub fn use_pass_mist<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_mist = pub fn set_use_pass_mist(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_normal => pub fn use_pass_normal<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_normal = pub fn set_use_pass_normal(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_object_index => pub fn use_pass_object_index<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_object_index = pub fn set_use_pass_object_index(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_position => pub fn use_pass_position<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_position = pub fn set_use_pass_position(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_shadow => pub fn use_pass_shadow<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_shadow = pub fn set_use_pass_shadow(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_subsurface_color => pub fn use_pass_subsurface_color<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_subsurface_color = pub fn set_use_pass_subsurface_color(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_subsurface_direct => pub fn use_pass_subsurface_direct<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_subsurface_direct = pub fn set_use_pass_subsurface_direct(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_subsurface_indirect => pub fn use_pass_subsurface_indirect<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_subsurface_indirect = pub fn set_use_pass_subsurface_indirect(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_transmission_color => pub fn use_pass_transmission_color<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_transmission_color = pub fn set_use_pass_transmission_color(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_transmission_direct => pub fn use_pass_transmission_direct<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_transmission_direct = pub fn set_use_pass_transmission_direct(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_transmission_indirect => pub fn use_pass_transmission_indirect<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_transmission_indirect = pub fn set_use_pass_transmission_indirect(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_uv => pub fn use_pass_uv<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_uv = pub fn set_use_pass_uv(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_vector => pub fn use_pass_vector<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_vector = pub fn set_use_pass_vector(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_pass_z => pub fn use_pass_z<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_pass_z = pub fn set_use_pass_z(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_sky => pub fn use_sky<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_sky = pub fn set_use_sky(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_solid => pub fn use_solid<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_solid = pub fn set_use_solid(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_strand => pub fn use_strand<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_strand = pub fn set_use_strand(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.use_volumes => pub fn use_volumes<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_volumes = pub fn set_use_volumes(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.update_render_passes() => pub fn update_render_passes(&self, py: Python) }
    bind_python! { self.update() => pub fn update(&self, py: Python) }
}

impl From<pyo3::PyObject> for ViewLayer {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for ViewLayer {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for ViewLayer {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for ViewLayer {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
