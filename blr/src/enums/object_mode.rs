use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/object_mode_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectMode {
    Edit,
    EditGpencil,
    Object,
    PaintGpencil,
    ParticleEdit,
    Pose,
    Sculpt,
    SculptCurves,
    SculptGpencil,
    TexturePaint,
    VertexGpencil,
    VertexPaint,
    WeightGpencil,
    WeightPaint,
}

impl TryFrom<&str> for ObjectMode {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "EDIT_GPENCIL" => Self::EditGpencil,
            "EDIT" => Self::Edit,
            "OBJECT" => Self::Object,
            "PAINT_GPENCIL" => Self::PaintGpencil,
            "PARTICLE_EDIT" => Self::ParticleEdit,
            "POSE" => Self::Pose,
            "SCULPT_CURVES" => Self::SculptCurves,
            "SCULPT_GPENCIL" => Self::SculptGpencil,
            "SCULPT" => Self::Sculpt,
            "TEXTURE_PAINT" => Self::TexturePaint,
            "VERTEX_GPENCIL" => Self::VertexGpencil,
            "VERTEX_PAINT" => Self::VertexPaint,
            "WEIGHT_GPENCIL" => Self::WeightGpencil,
            "WEIGHT_PAINT" => Self::WeightPaint,
            _ => Err(BlError::ValueError(format!("Unknown object mode: {s}")))?,
        })
    }
}

impl fmt::Display for ObjectMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Edit => write!(f, "EDIT"),
            Self::EditGpencil => write!(f, "EDIT_GPENCIL"),
            Self::Object => write!(f, "OBJECT"),
            Self::PaintGpencil => write!(f, "PAINT_GPENCIL"),
            Self::ParticleEdit => write!(f, "PARTICLE_EDIT"),
            Self::Pose => write!(f, "POSE"),
            Self::Sculpt => write!(f, "SCULPT"),
            Self::SculptCurves => write!(f, "SCULPT_CURVES"),
            Self::SculptGpencil => write!(f, "SCULPT_GPENCIL"),
            Self::TexturePaint => write!(f, "TEXTURE_PAINT"),
            Self::VertexGpencil => write!(f, "VERTEX_GPENCIL"),
            Self::VertexPaint => write!(f, "VERTEX_PAINT"),
            Self::WeightGpencil => write!(f, "WEIGHT_GPENCIL"),
            Self::WeightPaint => write!(f, "WEIGHT_PAINT"),
        }
    }
}

impl pyo3::FromPyObject<'_> for ObjectMode {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for ObjectMode {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
