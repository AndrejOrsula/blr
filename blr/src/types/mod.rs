//! Idiomatic wrappers around Blender [types](https://docs.blender.org/api/latest/bpy.types.html).

pub mod bpy_id;
pub mod bpy_struct;
pub mod collections;
pub mod materials;
pub mod modifiers;
pub mod nodes;
pub mod objects;
pub mod scene;
pub mod temp_override;

pub use bpy_id::BpyID;
pub use bpy_struct::BpyStruct;
pub use collections::{
    Collection, CollectionImpl, Materials, NodeLinks, NodeTreeInputs, NodeTreeOutputs, NodeTrees,
    Nodes, ObjectCollection, ObjectModifiers,
};
pub use materials::{IdMaterials, Material};
pub use modifiers::{Modifier, ModifierImpl};
pub use nodes::{Node, NodeLink, NodeSocket, NodeSocketInterface, NodeTree};
pub use objects::{
    Armature, Camera, Curve, CurveImpl, Curves, Empty, GreasePencil, Lattice, Light, LightProbe,
    Mesh, MetaBall, Object, PointCloud, Speaker, Surface, TextCurve, Volume,
};
pub use scene::{Scene, Screen, SpaceView3D, ViewLayer, Window, World};
pub use temp_override::TempOverride;

// pub type Addon<'py> = &'py pyo3::PyAny;
// pub type AddonPreferences<'py> = &'py pyo3::PyAny;
// pub type Addons<'py> = &'py pyo3::PyAny;
