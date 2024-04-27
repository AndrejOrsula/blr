use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderVariant {
    Preview,
    Render,
}

impl TryFrom<&str> for RenderVariant {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "PREVIEW" => Self::Preview,
            "RENDER" => Self::Render,
            _ => Err(BlError::ValueError(format!("Unknown render variant: {s}")))?,
        })
    }
}

impl fmt::Display for RenderVariant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Preview => write!(f, "PREVIEW"),
            Self::Render => write!(f, "RENDER"),
        }
    }
}

impl pyo3::FromPyObject<'_> for RenderVariant {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for RenderVariant {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
