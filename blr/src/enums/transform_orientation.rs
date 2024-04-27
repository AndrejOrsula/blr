use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/transform_orientation_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TransformOrientation {
    Cursor,
    Gimbal,
    Global,
    Local,
    Normal,
    Parent,
    View,
}

impl TryFrom<&str> for TransformOrientation {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "CURSOR" => Self::Cursor,
            "GIMBAL" => Self::Gimbal,
            "GLOBAL" => Self::Global,
            "LOCAL" => Self::Local,
            "NORMAL" => Self::Normal,
            "PARENT" => Self::Parent,
            "VIEW" => Self::View,
            _ => Err(BlError::ValueError(format!(
                "Unknown transform orientation: {s}"
            )))?,
        })
    }
}

impl fmt::Display for TransformOrientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cursor => write!(f, "CURSOR"),
            Self::Gimbal => write!(f, "GIMBAL"),
            Self::Global => write!(f, "GLOBAL"),
            Self::Local => write!(f, "LOCAL"),
            Self::Normal => write!(f, "NORMAL"),
            Self::Parent => write!(f, "PARENT"),
            Self::View => write!(f, "VIEW"),
        }
    }
}

impl pyo3::FromPyObject<'_> for TransformOrientation {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for TransformOrientation {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
