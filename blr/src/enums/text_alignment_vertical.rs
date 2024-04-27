use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around option for certain functions (unofficial)
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextAlignmentVertical {
    Bottom,
    BottomBaseline,
    Center,
    Top,
    TopBaseline,
}

impl TryFrom<&str> for TextAlignmentVertical {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "BOTTOM" => Self::Bottom,
            "BOTTOM_BASELINE" => Self::BottomBaseline,
            "CENTER" => Self::Center,
            "TOP" => Self::Top,
            "TOP_BASELINE" => Self::TopBaseline,
            _ => Err(BlError::ValueError(format!(
                "Unknown vertical text alignment: {s}"
            )))?,
        })
    }
}

impl fmt::Display for TextAlignmentVertical {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bottom => write!(f, "BOTTOM"),
            Self::BottomBaseline => write!(f, "BOTTOM_BASELINE"),
            Self::Center => write!(f, "CENTER"),
            Self::Top => write!(f, "TOP"),
            Self::TopBaseline => write!(f, "TOP_BASELINE"),
        }
    }
}

impl pyo3::FromPyObject<'_> for TextAlignmentVertical {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for TextAlignmentVertical {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
