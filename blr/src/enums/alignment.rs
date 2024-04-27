use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alignment {
    Cursor,
    View,
    World,
}

impl TryFrom<&str> for Alignment {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "CURSOR" => Self::Cursor,
            "VIEW" => Self::View,
            "WORLD" => Self::World,
            _ => Err(BlError::ValueError(format!("Unknown alignment: {s}")))?,
        })
    }
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cursor => write!(f, "CURSOR"),
            Self::View => write!(f, "VIEW"),
            Self::World => write!(f, "WORLD"),
        }
    }
}

impl pyo3::FromPyObject<'_> for Alignment {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for Alignment {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
