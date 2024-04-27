use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/axis_xyz_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AxisXYZ {
    X,
    Y,
    Z,
}

impl TryFrom<&str> for AxisXYZ {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "X" => Self::X,
            "Y" => Self::Y,
            "Z" => Self::Z,
            _ => Err(BlError::ValueError(format!("Invalid axis: {s}")))?,
        })
    }
}

impl fmt::Display for AxisXYZ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::X => write!(f, "X"),
            Self::Y => write!(f, "Y"),
            Self::Z => write!(f, "Z"),
        }
    }
}

impl pyo3::FromPyObject<'_> for AxisXYZ {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for AxisXYZ {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
