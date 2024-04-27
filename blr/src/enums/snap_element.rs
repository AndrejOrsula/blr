use crate::{result::Result, BlError};
use std::{collections::HashSet, fmt};

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/snap_element_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapElement {
    Edge,
    EdgeMidpoint,
    EdgePerpendicular,
    Face,
    FaceNearest,
    Increment,
    Vertex,
    Volume,
}

impl TryFrom<&str> for SnapElement {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "EDGE_MIDPOINT" => Self::EdgeMidpoint,
            "EDGE_PERPENDICULAR" => Self::EdgePerpendicular,
            "EDGE" => Self::Edge,
            "FACE_NEAREST" => Self::FaceNearest,
            "FACE" => Self::Face,
            "INCREMENT" => Self::Increment,
            "VERTEX" => Self::Vertex,
            "VOLUME" => Self::Volume,
            _ => Err(BlError::ValueError(format!("Unknown snap element: {s}")))?,
        })
    }
}

impl fmt::Display for SnapElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Edge => write!(f, "EDGE"),
            Self::EdgeMidpoint => write!(f, "EDGE_MIDPOINT"),
            Self::EdgePerpendicular => write!(f, "EDGE_PERPENDICULAR"),
            Self::Face => write!(f, "FACE"),
            Self::FaceNearest => write!(f, "FACE_NEAREST"),
            Self::Increment => write!(f, "INCREMENT"),
            Self::Vertex => write!(f, "VERTEX"),
            Self::Volume => write!(f, "VOLUME"),
        }
    }
}

impl pyo3::FromPyObject<'_> for SnapElement {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        let hashset: HashSet<String> = value.extract()?;
        let value = hashset.into_iter().next().ok_or(BlError::ValueError(
            "Snap element cannot be determined due to an empty set".to_string(),
        ))?;
        Ok(value.as_str().try_into()?)
    }
}

impl pyo3::ToPyObject for SnapElement {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        let mut hashset = HashSet::new();
        hashset.insert(self.to_string());
        hashset.to_object(py)
    }
}
