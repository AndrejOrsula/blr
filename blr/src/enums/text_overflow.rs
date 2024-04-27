use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextOverflow {
    None,
    Scale,
    Truncate,
}

impl TryFrom<&str> for TextOverflow {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "NONE" => Self::None,
            "SCALE" => Self::Scale,
            "TRUNCATE" => Self::Truncate,
            _ => Err(BlError::ValueError(format!("Unknown text overflow: {s}")))?,
        })
    }
}

impl fmt::Display for TextOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Scale => write!(f, "SCALE"),
            Self::Truncate => write!(f, "TRUNCATE"),
        }
    }
}

impl pyo3::FromPyObject<'_> for TextOverflow {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for TextOverflow {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
