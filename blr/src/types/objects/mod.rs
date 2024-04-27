use crate::{enums::ObjectType, error::BlError, result::Result, types::bpy_id::BpyID};

pub mod armature;
pub mod camera;
pub mod curve;
pub mod curves;
pub mod empty;
pub mod grease_pencil;
pub mod lattice;
pub mod light;
pub mod light_probe;
pub mod mesh;
pub mod meta;
pub mod object;
pub mod pointcloud;
pub mod primitives;
pub mod speaker;
pub mod surface;
pub mod textcurve;
pub mod volume;

pub use armature::Armature;
pub use camera::Camera;
pub use curve::{Curve, CurveImpl};
pub use curves::Curves;
pub use empty::Empty;
pub use grease_pencil::GreasePencil;
pub use lattice::Lattice;
pub use light::Light;
pub use light_probe::LightProbe;
pub use mesh::Mesh;
pub use meta::MetaBall;
pub use object::Object;
pub use pointcloud::PointCloud;
pub use speaker::Speaker;
pub use surface::Surface;
pub use textcurve::TextCurve;
pub use volume::Volume;

macro_rules! impl_try_from_object_for {
    ($enum_variant:path, $object_type:ty) => {
        impl TryFrom<Object> for $object_type {
            type Error = BlError;

            fn try_from(value: Object) -> Result<Self> {
                pyo3::Python::with_gil(|py| match value.object_type(py)? {
                    $enum_variant => Ok(value.data(py)?.into()),
                    incorrect_type => Err(BlError::TypeError(format!(
                        "Blender object {name} is of type {incorrect_type}, but {correct_type} is expected",
                        name = value.name(py)?,
                        correct_type = $enum_variant,
                    ))),
                })
            }
        }

        impl TryFrom<&Object> for $object_type {
            type Error = BlError;

            fn try_from(value: &Object) -> Result<Self> {
                pyo3::Python::with_gil(|py| match value.object_type(py)? {
                    $enum_variant => Ok(value.data(py)?.into()),
                    incorrect_type => Err(BlError::TypeError(format!(
                        "Blender object {name} is of type {incorrect_type}, but {correct_type} is expected",
                        name = value.name(py)?,
                        correct_type = $enum_variant,
                    ))),
                })
            }
        }
    };
}
impl_try_from_object_for!(ObjectType::Armature, Armature);
impl_try_from_object_for!(ObjectType::Camera, Camera);
impl_try_from_object_for!(ObjectType::Curve, Curve);
impl_try_from_object_for!(ObjectType::Curves, Curves);
impl_try_from_object_for!(ObjectType::GreasePencil, GreasePencil);
impl_try_from_object_for!(ObjectType::Lattice, Lattice);
impl_try_from_object_for!(ObjectType::Light, Light);
impl_try_from_object_for!(ObjectType::LightProbe, LightProbe);
impl_try_from_object_for!(ObjectType::Mesh, Mesh);
impl_try_from_object_for!(ObjectType::Meta, MetaBall);
impl_try_from_object_for!(ObjectType::PointCloud, PointCloud);
impl_try_from_object_for!(ObjectType::Speaker, Speaker);
impl_try_from_object_for!(ObjectType::Surface, Surface);
impl_try_from_object_for!(ObjectType::Font, TextCurve);
impl_try_from_object_for!(ObjectType::Volume, Volume);

impl TryFrom<Object> for Empty {
    type Error = BlError;

    /// Converts an Object into an Empty, if the Object is an Empty. Note the underlying data structure of
    /// Empty is identical to the Object data structure (not contained in `data` as the rest of the objects).
    fn try_from(value: Object) -> Result<Self> {
        pyo3::Python::with_gil(|py| match value.object_type(py)? {
            ObjectType::Empty => Ok(Self(value)),
            incorrect_type => Err(BlError::TypeError(format!(
                "Blender object {name} is of type {incorrect_type}, but {correct_type} is expected",
                name = value.name(py)?,
                correct_type = ObjectType::Empty,
            ))),
        })
    }
}
