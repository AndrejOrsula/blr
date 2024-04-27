use crate::{types::BpyID, NodeTree};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyAny, PyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Material.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Material(PyObject);

impl BpyID for Material {}

impl Material {
    bind_python! { self.alpha_threshold => pub fn alpha_threshold(&self, py: Python) -> Result<f32> }
    bind_python! { self.alpha_threshold = pub fn set_alpha_threshold(&mut self, py: Python, value: &f32) }
    bind_python! { self.animation_data => pub fn animation_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.blend_method => pub fn blend_method(&self, py: Python) -> Result<String> }
    bind_python! { self.blend_method = pub fn set_blend_method(&mut self, py: Python, value: &str) }
    bind_python! { self.cycles => pub fn cycles<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.diffuse_color => pub fn diffuse_color(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.diffuse_color = pub fn set_diffuse_color(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.grease_pencil => pub fn grease_pencil<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.is_grease_pencil => pub fn is_grease_pencil<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.line_color => pub fn line_color(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.line_color = pub fn set_line_color(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.line_priority => pub fn line_priority(&self, py: Python) -> Result<u16> }
    bind_python! { self.line_priority = pub fn set_line_priority(&mut self, py: Python, value: &u16) }
    bind_python! { self.lineart => pub fn lineart<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.metallic => pub fn metallic(&self, py: Python) -> Result<f32> }
    bind_python! { self.metallic = pub fn set_metallic(&mut self, py: Python, value: &f32) }
    bind_python! { self.node_tree => pub fn node_tree(&self, py: Python) -> Result<NodeTree>  }
    bind_python! { self.paint_active_slot => pub fn paint_active_slot(&self, py: Python) -> Result<u16> }
    bind_python! { self.paint_active_slot = pub fn set_paint_active_slot(&mut self, py: Python, value: &u16) }
    bind_python! { self.paint_clone_slot => pub fn paint_clone_slot(&self, py: Python) -> Result<u16> }
    bind_python! { self.paint_clone_slot = pub fn set_paint_clone_slot(&mut self, py: Python, value: &u16) }
    bind_python! { self.pass_index => pub fn pass_index(&self, py: Python) -> Result<u16> }
    bind_python! { self.pass_index = pub fn set_pass_index(&mut self, py: Python, value: &u16) }
    bind_python! { self.preview_render_type => pub fn preview_render_type(&self, py: Python) -> Result<String> }
    bind_python! { self.preview_render_type = pub fn set_preview_render_type(&mut self, py: Python, value: &str) }
    bind_python! { self.refraction_depth => pub fn refraction_depth(&self, py: Python) -> Result<f32> }
    bind_python! { self.refraction_depth = pub fn set_refraction_depth(&mut self, py: Python, value: &f32) }
    bind_python! { self.roughness => pub fn roughness(&self, py: Python) -> Result<f32> }
    bind_python! { self.roughness = pub fn set_roughness(&mut self, py: Python, value: &f32) }
    bind_python! { self.shadow_method => pub fn shadow_method(&self, py: Python) -> Result<String> }
    bind_python! { self.shadow_method = pub fn set_shadow_method(&mut self, py: Python, value: &str) }
    bind_python! { self.show_transparent_back => pub fn show_transparent_back(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_transparent_back = pub fn set_show_transparent_back(&mut self, py: Python, value: &bool) }
    bind_python! { self.specular_color => pub fn specular_color(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.specular_color = pub fn set_specular_color(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.specular_intensity => pub fn specular_intensity(&self, py: Python) -> Result<f32> }
    bind_python! { self.specular_intensity = pub fn set_specular_intensity(&mut self, py: Python, value: &f32) }
    bind_python! { self.texture_paint_images => pub fn texture_paint_images<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.texture_paint_slots => pub fn texture_paint_slots<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny>  }
    bind_python! { self.use_backface_culling => pub fn use_backface_culling(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_backface_culling = pub fn set_use_backface_culling(&mut self, py: Python, value: &bool) }
    bind_python! { self.use_nodes => pub fn use_nodes(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_nodes = pub fn set_use_nodes(&mut self, py: Python, value: &bool) }
    bind_python! { self.use_preview_world => pub fn use_preview_world(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_preview_world = pub fn set_use_preview_world(&mut self, py: Python, value: &bool) }
    bind_python! { self.use_screen_refraction => pub fn use_screen_refraction(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_screen_refraction = pub fn set_use_screen_refraction(&mut self, py: Python, value: &bool) }
    bind_python! { self.use_sss_translucency => pub fn use_sss_translucency(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_sss_translucency = pub fn set_use_sss_translucency(&mut self, py: Python, value: &bool) }
}

impl From<pyo3::PyObject> for Material {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Material {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Material {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Material {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
