use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OriginCenter {
    Bounds,
    Median,
}

impl TryFrom<&str> for OriginCenter {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "BOUNDS" => Self::Bounds,
            "MEDIAN" => Self::Median,
            _ => Err(BlError::ValueError(format!("Unknown origin center: {s}")))?,
        })
    }
}

impl fmt::Display for OriginCenter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bounds => write!(f, "BOUNDS"),
            Self::Median => write!(f, "MEDIAN"),
        }
    }
}

impl pyo3::FromPyObject<'_> for OriginCenter {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for OriginCenter {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
