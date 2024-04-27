use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around node tree type (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeTreeType {
    Compositing,
    Geometry,
    Shader,
    Texture,
    Undefined,
}

impl TryFrom<&str> for NodeTreeType {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "COMPOSITING" => Self::Compositing,
            "GEOMETRY" => Self::Geometry,
            "SHADER" => Self::Shader,
            "TEXTURE" => Self::Texture,
            "UNDEFINED" => Self::Undefined,
            _ => Err(BlError::ValueError(format!("Unknown Node tree type: {s}")))?,
        })
    }
}

impl fmt::Display for NodeTreeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Compositing => write!(f, "COMPOSITING"),
            Self::Geometry => write!(f, "GEOMETRY"),
            Self::Shader => write!(f, "SHADER"),
            Self::Texture => write!(f, "TEXTURE"),
            Self::Undefined => write!(f, "UNDEFINED"),
        }
    }
}

impl pyo3::FromPyObject<'_> for NodeTreeType {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for NodeTreeType {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
