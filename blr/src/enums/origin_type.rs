use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginType {
    GeometryOrigin,
    OriginCenterOfMass,
    OriginCenterOfVolume,
    OriginCursor,
    OriginGeometry,
}

impl TryFrom<&str> for OriginType {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "GEOMETRY_ORIGIN" => Self::GeometryOrigin,
            "ORIGIN_CENTER_OF_MASS" => Self::OriginCenterOfMass,
            "ORIGIN_CENTER_OF_VOLUME" => Self::OriginCenterOfVolume,
            "ORIGIN_CURSOR" => Self::OriginCursor,
            "ORIGIN_GEOMETRY" => Self::OriginGeometry,
            _ => Err(BlError::ValueError(format!("Unknown origin type: {s}")))?,
        })
    }
}

impl fmt::Display for OriginType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GeometryOrigin => write!(f, "GEOMETRY_ORIGIN"),
            Self::OriginCenterOfMass => write!(f, "ORIGIN_CENTER_OF_MASS"),
            Self::OriginCenterOfVolume => write!(f, "ORIGIN_CENTER_OF_VOLUME"),
            Self::OriginCursor => write!(f, "ORIGIN_CURSOR"),
            Self::OriginGeometry => write!(f, "ORIGIN_GEOMETRY"),
        }
    }
}

impl pyo3::FromPyObject<'_> for OriginType {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for OriginType {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
