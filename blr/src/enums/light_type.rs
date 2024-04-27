use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/light_type_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightType {
    Area,
    Point,
    Spot,
    Sun,
}

impl TryFrom<&str> for LightType {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "AREA" => Self::Area,
            "POINT" => Self::Point,
            "SPOT" => Self::Spot,
            "SUN" => Self::Sun,
            _ => Err(BlError::ValueError(format!("Invalid light type: {s}")))?,
        })
    }
}

impl fmt::Display for LightType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Area => write!(f, "AREA"),
            Self::Point => write!(f, "POINT"),
            Self::Spot => write!(f, "SPOT"),
            Self::Sun => write!(f, "SUN"),
        }
    }
}

impl pyo3::FromPyObject<'_> for LightType {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for LightType {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
