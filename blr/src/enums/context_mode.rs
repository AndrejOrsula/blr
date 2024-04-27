use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/context_mode_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContextMode {
    EditArmature,
    EditCurve,
    EditCurves,
    EditGpencil,
    EditLattice,
    EditMesh,
    EditMetaball,
    EditSurface,
    EditText,
    Object,
    PaintGpencil,
    PaintTexture,
    PaintVertex,
    PaintWeight,
    Particle,
    Pose,
    Sculpt,
    SculptCurves,
    SculptGpencil,
    VertexGpencil,
    WeightGpencil,
}

impl TryFrom<&str> for ContextMode {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "EDIT_ARMATURE" => Self::EditArmature,
            "EDIT_CURVE" => Self::EditCurve,
            "EDIT_CURVES" => Self::EditCurves,
            "EDIT_GPENCIL" => Self::EditGpencil,
            "EDIT_LATTICE" => Self::EditLattice,
            "EDIT_MESH" => Self::EditMesh,
            "EDIT_METABALL" => Self::EditMetaball,
            "EDIT_SURFACE" => Self::EditSurface,
            "EDIT_TEXT" => Self::EditText,
            "OBJECT" => Self::Object,
            "PAINT_GPENCIL" => Self::PaintGpencil,
            "PAINT_TEXTURE" => Self::PaintTexture,
            "PAINT_VERTEX" => Self::PaintVertex,
            "PAINT_WEIGHT" => Self::PaintWeight,
            "PARTICLE" => Self::Particle,
            "POSE" => Self::Pose,
            "SCULPT_CURVES" => Self::SculptCurves,
            "SCULPT_GPENCIL" => Self::SculptGpencil,
            "SCULPT" => Self::Sculpt,
            "VERTEX_GPENCIL" => Self::VertexGpencil,
            "WEIGHT_GPENCIL" => Self::WeightGpencil,
            _ => Err(BlError::ValueError(format!("Unknown context mode: {s}")))?,
        })
    }
}

impl fmt::Display for ContextMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EditArmature => write!(f, "EDIT_ARMATURE"),
            Self::EditCurve => write!(f, "EDIT_CURVE"),
            Self::EditCurves => write!(f, "EDIT_CURVES"),
            Self::EditGpencil => write!(f, "EDIT_GPENCIL"),
            Self::EditLattice => write!(f, "EDIT_LATTICE"),
            Self::EditMesh => write!(f, "EDIT_MESH"),
            Self::EditMetaball => write!(f, "EDIT_METABALL"),
            Self::EditSurface => write!(f, "EDIT_SURFACE"),
            Self::EditText => write!(f, "EDIT_TEXT"),
            Self::Object => write!(f, "OBJECT"),
            Self::PaintGpencil => write!(f, "PAINT_GPENCIL"),
            Self::PaintTexture => write!(f, "PAINT_TEXTURE"),
            Self::PaintVertex => write!(f, "PAINT_VERTEX"),
            Self::PaintWeight => write!(f, "PAINT_WEIGHT"),
            Self::Particle => write!(f, "PARTICLE"),
            Self::Pose => write!(f, "POSE"),
            Self::Sculpt => write!(f, "SCULPT"),
            Self::SculptCurves => write!(f, "SCULPT_CURVES"),
            Self::SculptGpencil => write!(f, "SCULPT_GPENCIL"),
            Self::VertexGpencil => write!(f, "VERTEX_GPENCIL"),
            Self::WeightGpencil => write!(f, "WEIGHT_GPENCIL"),
        }
    }
}

impl pyo3::FromPyObject<'_> for ContextMode {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for ContextMode {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
