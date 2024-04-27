use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignmentHorizontal {
    Center,
    Flush,
    Justify,
    Left,
    Right,
}

impl TryFrom<&str> for TextAlignmentHorizontal {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "CENTER" => Self::Center,
            "FLUSH" => Self::Flush,
            "JUSTIFY" => Self::Justify,
            "LEFT" => Self::Left,
            "RIGHT" => Self::Right,
            _ => Err(BlError::ValueError(format!(
                "Unknown horizontal text alignment: {s}"
            )))?,
        })
    }
}

impl fmt::Display for TextAlignmentHorizontal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Center => write!(f, "CENTER"),
            Self::Flush => write!(f, "FLUSH"),
            Self::Justify => write!(f, "JUSTIFY"),
            Self::Left => write!(f, "LEFT"),
            Self::Right => write!(f, "RIGHT"),
        }
    }
}

impl pyo3::FromPyObject<'_> for TextAlignmentHorizontal {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for TextAlignmentHorizontal {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
