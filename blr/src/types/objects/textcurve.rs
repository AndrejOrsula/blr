use crate::{
    enums::{TextAlignmentHorizontal, TextAlignmentVertical, TextOverflow},
    types::{BpyID, Collection, CurveImpl, Object},
};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyAny, PyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.TextCurve.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct TextCurve(PyObject);

impl BpyID for TextCurve {}
impl CurveImpl for TextCurve {}

impl TextCurve {
    bind_python! { self.active_textbox => pub fn active_textbox(&self, py: Python) -> Result<i32> }
    bind_python! { self.active_textbox = pub fn set_active_textbox(&mut self, py: Python, value: i32) }
    bind_python! { self.align_x => pub fn align_x(&self, py: Python) -> Result<TextAlignmentHorizontal> }
    bind_python! { self.align_x = pub fn set_align_x(&mut self, py: Python, value: TextAlignmentHorizontal) }
    bind_python! { self.align_y => pub fn align_y(&self, py: Python) -> Result<TextAlignmentVertical> }
    bind_python! { self.align_y = pub fn set_align_y(&mut self, py: Python, value: TextAlignmentVertical) }
    bind_python! { self.body => pub fn body(&self, py: Python) -> Result<String> }
    bind_python! { self.body = pub fn set_body(&mut self, py: Python, value: &str) }
    bind_python! { self.body_format => pub fn body_format(&self, py: Python) -> Result<Collection> }
    bind_python! { self.edit_format => pub fn edit_format<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.family => pub fn family(&self, py: Python) -> Result<String> }
    bind_python! { self.family = pub fn set_family(&mut self, py: Python, value: &str) }
    bind_python! { self.follow_curve => pub fn follow_curve(&self, py: Python) -> Result<Object> }
    bind_python! { self.follow_curve = pub fn set_follow_curve(&mut self, py: Python, value: Object) }
    bind_python! { self.font => pub fn font<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.font = pub fn set_font(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.font_bold => pub fn font_bold<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.font_bold = pub fn set_font_bold(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.font_bold_italic => pub fn font_bold_italic<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.font_bold_italic = pub fn set_font_bold_italic(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.font_italic => pub fn font_italic<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.font_italic = pub fn set_font_italic(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.has_selection => pub fn has_selection(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_select_bold => pub fn is_select_bold(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_select_italic => pub fn is_select_italic(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_select_smallcaps => pub fn is_select_smallcaps(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_select_underline => pub fn is_select_underline(&self, py: Python) -> Result<bool> }
    bind_python! { self.offset_x => pub fn offset_x(&self, py: Python) -> Result<f32> }
    bind_python! { self.offset_x = pub fn set_offset_x(&mut self, py: Python, value: f32) }
    bind_python! { self.offset_y => pub fn offset_y(&self, py: Python) -> Result<f32> }
    bind_python! { self.offset_y = pub fn set_offset_y(&mut self, py: Python, value: f32) }
    bind_python! { self.overflow => pub fn overflow(&self, py: Python) -> Result<TextOverflow> }
    bind_python! { self.overflow = pub fn set_overflow(&mut self, py: Python, value: TextOverflow) }
    bind_python! { self.shear => pub fn shear(&self, py: Python) -> Result<f32> }
    bind_python! { self.shear = pub fn set_shear(&mut self, py: Python, value: f32) }
    bind_python! { self.size => pub fn size(&self, py: Python) -> Result<f32> }
    bind_python! { self.size = pub fn set_size(&mut self, py: Python, value: f32) }
    bind_python! { self.small_caps_scale => pub fn small_caps_scale(&self, py: Python) -> Result<f32> }
    bind_python! { self.small_caps_scale = pub fn set_small_caps_scale(&mut self, py: Python, value: f32) }
    bind_python! { self.space_character => pub fn space_character(&self, py: Python) -> Result<f32> }
    bind_python! { self.space_character = pub fn set_space_character(&mut self, py: Python, value: f32) }
    bind_python! { self.space_line => pub fn space_line(&self, py: Python) -> Result<f32> }
    bind_python! { self.space_line = pub fn set_space_line(&mut self, py: Python, value: f32) }
    bind_python! { self.space_word => pub fn space_word(&self, py: Python) -> Result<f32> }
    bind_python! { self.space_word = pub fn set_space_word(&mut self, py: Python, value: f32) }
    bind_python! { self.text_boxes => pub fn text_boxes(&self, py: Python) -> Result<Collection> }
    bind_python! { self.underline_height => pub fn underline_height(&self, py: Python) -> Result<f32> }
    bind_python! { self.underline_height = pub fn set_underline_height(&mut self, py: Python, value: f32) }
    bind_python! { self.underline_position => pub fn underline_position(&self, py: Python) -> Result<f32> }
    bind_python! { self.underline_position = pub fn set_underline_position(&mut self, py: Python, value: f32) }
    bind_python! { self.use_fast_edit => pub fn use_fast_edit(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_fast_edit = pub fn set_use_fast_edit(&mut self, py: Python, value: bool) }
}

impl From<pyo3::PyObject> for TextCurve {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for TextCurve {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for TextCurve {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for TextCurve {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
