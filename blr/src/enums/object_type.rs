use crate::{result::Result, BlError};
use std::fmt;

/// Wrapper around <https://docs.blender.org/api/latest/bpy_types_enum_items/object_type_items.html>
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObjectType {
    Armature,
    Camera,
    Curve,
    Curves,
    Empty,
    Font,
    GreasePencil,
    Lattice,
    Light,
    LightProbe,
    Mesh,
    Meta,
    PointCloud,
    Speaker,
    Surface,
    Volume,
}

impl TryFrom<&str> for ObjectType {
    type Error = BlError;

    fn try_from(s: &str) -> Result<Self> {
        Ok(match s {
            "ARMATURE" => Self::Armature,
            "CAMERA" => Self::Camera,
            "CURVE" => Self::Curve,
            "CURVES" => Self::Curves,
            "EMPTY" => Self::Empty,
            "FONT" => Self::Font,
            "GPENCIL" => Self::GreasePencil,
            "LATTICE" => Self::Lattice,
            "LIGHT_PROBE" => Self::LightProbe,
            "LIGHT" => Self::Light,
            "MESH" => Self::Mesh,
            "META" => Self::Meta,
            "POINTCLOUD" => Self::PointCloud,
            "SPEAKER" => Self::Speaker,
            "SURFACE" => Self::Surface,
            "VOLUME" => Self::Volume,
            _ => Err(BlError::ValueError(format!("Unknown object type: {s}")))?,
        })
    }
}

impl fmt::Display for ObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Armature => write!(f, "ARMATURE"),
            Self::Camera => write!(f, "CAMERA"),
            Self::Curve => write!(f, "CURVE"),
            Self::Curves => write!(f, "CURVES"),
            Self::Empty => write!(f, "EMPTY"),
            Self::Font => write!(f, "FONT"),
            Self::GreasePencil => write!(f, "GPENCIL"),
            Self::Lattice => write!(f, "LATTICE"),
            Self::Light => write!(f, "LIGHT"),
            Self::LightProbe => write!(f, "LIGHT_PROBE"),
            Self::Mesh => write!(f, "MESH"),
            Self::Meta => write!(f, "META"),
            Self::PointCloud => write!(f, "POINTCLOUD"),
            Self::Speaker => write!(f, "SPEAKER"),
            Self::Surface => write!(f, "SURFACE"),
            Self::Volume => write!(f, "VOLUME"),
        }
    }
}

impl pyo3::FromPyObject<'_> for ObjectType {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(value.str()?.to_str()?.try_into()?)
    }
}

impl pyo3::ToPyObject for ObjectType {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.to_string().to_object(py)
    }
}
