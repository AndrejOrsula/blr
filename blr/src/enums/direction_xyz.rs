use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectionXYZ {
    PosX,
    PosY,
    PosZ,
    NegX,
    NegY,
    NegZ,
}

impl TryFrom<&str> for DirectionXYZ {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "X" => Self::PosX,
            "Y" => Self::PosY,
            "Z" => Self::PosZ,
            "-X" => Self::NegX,
            "-Y" => Self::NegY,
            "-Z" => Self::NegZ,
            _ => Err(BlError::ValueError(format!("Invalid axis: {s}")))?,
        })
    }
}

impl fmt::Display for DirectionXYZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PosX => write!(f, "X"),
            Self::PosY => write!(f, "Y"),
            Self::PosZ => write!(f, "Z"),
            Self::NegX => write!(f, "-X"),
            Self::NegY => write!(f, "-Y"),
            Self::NegZ => write!(f, "-Z"),
        }
    }
}

impl pyo3::FromPyObject<'_> for DirectionXYZ {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for DirectionXYZ {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
