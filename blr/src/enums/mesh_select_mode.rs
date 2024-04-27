use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/mesh_select_mode_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MeshSelectMode {
    Edge,
    Face,
    Vert,
}

impl TryFrom<&str> for MeshSelectMode {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "EDGE" => Self::Edge,
            "FACE" => Self::Face,
            "VERT" => Self::Vert,
            _ => Err(BlError::ValueError(format!(
                "Invalid mesh selection mode: {s}"
            )))?,
        })
    }
}

impl fmt::Display for MeshSelectMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Edge => write!(f, "EDGE"),
            Self::Face => write!(f, "FACE"),
            Self::Vert => write!(f, "VERT"),
        }
    }
}

impl pyo3::FromPyObject<'_> for MeshSelectMode {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for MeshSelectMode {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
