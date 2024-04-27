use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtype {
    Boolean,
    Collection,
    Custom,
    Geometry,
    Image,
    Int,
    Material,
    Object,
    Rgba,
    Shader,
    String,
    Texture,
    Value,
    Vector,
}

impl TryFrom<&str> for Dtype {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "COLLECTION" => Self::Collection,
            "CUSTOM" => Self::Custom,
            "GEOMETRY" => Self::Geometry,
            "IMAGE" => Self::Image,
            "INT" => Self::Int,
            "MATERIAL" => Self::Material,
            "OBJECT" => Self::Object,
            "RGBA" => Self::Rgba,
            "SHADER" => Self::Shader,
            "STRING" => Self::String,
            "TEXTURE" => Self::Texture,
            "VALUE" => Self::Value,
            "VECTOR" => Self::Vector,
            _ => Err(BlError::ValueError(format!("Invalid data type: {s}")))?,
        })
    }
}

impl fmt::Display for Dtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "BOOLEAN"),
            Self::Collection => write!(f, "COLLECTION"),
            Self::Custom => write!(f, "CUSTOM"),
            Self::Geometry => write!(f, "GEOMETRY"),
            Self::Image => write!(f, "IMAGE"),
            Self::Int => write!(f, "INT"),
            Self::Material => write!(f, "MATERIAL"),
            Self::Object => write!(f, "OBJECT"),
            Self::Rgba => write!(f, "RGBA"),
            Self::Shader => write!(f, "SHADER"),
            Self::String => write!(f, "STRING"),
            Self::Texture => write!(f, "TEXTURE"),
            Self::Value => write!(f, "VALUE"),
            Self::Vector => write!(f, "VECTOR"),
        }
    }
}

impl pyo3::FromPyObject<'_> for Dtype {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for Dtype {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
