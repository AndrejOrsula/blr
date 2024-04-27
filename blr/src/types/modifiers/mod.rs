pub mod armature;
pub mod array;
pub mod bevel;
pub mod boolean;
pub mod build;
pub mod cast;
pub mod cloth;
pub mod collision;
pub mod corrective_smooth;
pub mod curve;
pub mod data_transfer;
pub mod decimate;
pub mod displace;
pub mod dynamic_paint;
pub mod edge_split;
pub mod explode;
pub mod fluid;
pub mod hook;
pub mod laplaciandeform;
pub mod laplaciansmooth;
pub mod lattice;
pub mod mask;
pub mod mesh_cache;
pub mod mesh_deform;
pub mod mesh_sequence_cache;
pub mod mesh_to_volume;
pub mod mirror;
pub mod modifier;
pub mod multires;
pub mod nodes;
pub mod normal_edit;
pub mod ocean;
pub mod particle_instance;
pub mod particle_system;
pub mod remesh;
pub mod screw;
pub mod shrinkwrap;
pub mod simple_deform;
pub mod skin;
pub mod smooth;
pub mod soft_body;
pub mod solidify;
pub mod subsurf;
pub mod surface;
pub mod surface_deform;
pub mod triangulate;
pub mod uv_project;
pub mod uv_warp;
pub mod vertex_weight_edit;
pub mod vertex_weight_mix;
pub mod vertex_weight_proximity;
pub mod volume_displace;
pub mod volume_to_mesh;
pub mod warp;
pub mod wave;
pub mod weighted_normal;
pub mod weld;
pub mod wireframe;

pub use armature::ArmatureModifier;
pub use array::ArrayModifier;
pub use bevel::BevelModifier;
pub use boolean::BooleanModifier;
pub use build::BuildModifier;
pub use cast::CastModifier;
pub use cloth::ClothModifier;
pub use collision::CollisionModifier;
pub use corrective_smooth::CorrectiveSmoothModifier;
pub use curve::CurveModifier;
pub use data_transfer::DataTransferModifier;
pub use decimate::DecimateModifier;
pub use displace::DisplaceModifier;
pub use dynamic_paint::DynamicPaintModifier;
pub use edge_split::EdgeSplitModifier;
pub use explode::ExplodeModifier;
pub use fluid::FluidModifier;
pub use hook::HookModifier;
pub use laplaciandeform::LaplaciandeformModifier;
pub use laplaciansmooth::LaplaciansmoothModifier;
pub use lattice::LatticeModifier;
pub use mask::MaskModifier;
pub use mesh_cache::MeshCacheModifier;
pub use mesh_deform::MeshDeformModifier;
pub use mesh_sequence_cache::MeshSequenceCacheModifier;
pub use mesh_to_volume::MeshToVolumeModifier;
pub use mirror::MirrorModifier;
pub use modifier::Modifier;
pub use multires::MultiresModifier;
pub use nodes::NodesModifier;
pub use normal_edit::NormalEditModifier;
pub use ocean::OceanModifier;
pub use particle_instance::ParticleInstanceModifier;
pub use particle_system::ParticleSystemModifier;
pub use remesh::RemeshModifier;
pub use screw::ScrewModifier;
pub use shrinkwrap::ShrinkwrapModifier;
pub use simple_deform::SimpleDeformModifier;
pub use skin::SkinModifier;
pub use smooth::SmoothModifier;
pub use soft_body::SoftBodyModifier;
pub use solidify::SolidifyModifier;
pub use subsurf::SubsurfModifier;
pub use surface::SurfaceModifier;
pub use surface_deform::SurfaceDeformModifier;
pub use triangulate::TriangulateModifier;
pub use uv_project::UvProjectModifier;
pub use uv_warp::UvWarpModifier;
pub use vertex_weight_edit::VertexWeightEditModifier;
pub use vertex_weight_mix::VertexWeightMixModifier;
pub use vertex_weight_proximity::VertexWeightProximityModifier;
pub use volume_displace::VolumeDisplaceModifier;
pub use volume_to_mesh::VolumeToMeshModifier;
pub use warp::WarpModifier;
pub use wave::WaveModifier;
pub use weighted_normal::WeightedNormalModifier;
pub use weld::WeldModifier;
pub use wireframe::WireframeModifier;

use crate::{enums::ModifierType, error::BlError, result::Result, types::Object};
use pyo3::{intern, PyObject, Python};
use pyo3_macros_more::bind_python;
use std::ops::{Deref, DerefMut};

/// Wrapper around <https://docs.blender.org/api/latest/bpy.types.Modifier.html>
pub trait ModifierImpl:
    Deref<Target = PyObject> + DerefMut<Target = PyObject> + pyo3::ToPyObject + Into<Modifier> + Sized
where
    Modifier: TryInto<Self, Error = BlError>,
{
    const MODIFIER_TYPE: ModifierType;

    fn new(py: Python, object: &Object, name: &str) -> Result<Self> {
        object
            .modifiers(py)?
            .new(py, name, Self::MODIFIER_TYPE)?
            .try_into()
    }

    fn remove(self, py: Python, object: &Object) -> Result<()> {
        Ok(object.modifiers(py)?.remove(py, self.into())?)
    }

    fn modifier_type(&self, py: Python) -> Result<ModifierType> {
        Ok(self.getattr(py, intern!(py, "type"))?.extract(py)?)
    }

    bind_python! { self.execution_time => fn execution_time(&self, py: Python) -> Result<f32> }
    bind_python! { self.execution_time = fn set_execution_time(&mut self, py: Python, value: f32) }
    bind_python! { self.is_active => fn is_active(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_active = fn set_is_active(&mut self, py: Python, value: bool) }
    bind_python! { self.is_override_data => fn is_override_data(&self, py: Python) -> Result<bool> }
    bind_python! { self.name => fn name(&self, py: Python) -> Result<String> }
    bind_python! { self.name = fn set_name(&mut self, py: Python, value: bool) }
    bind_python! { self.show_expanded => fn show_expanded(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_expanded = fn set_show_expanded(&mut self, py: Python, value: &str) }
    bind_python! { self.show_in_editmode => fn show_in_editmode(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_in_editmode = fn set_show_in_editmode(&mut self, py: Python, value: bool) }
    bind_python! { self.show_on_cage => fn show_on_cage(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_on_cage = fn set_show_on_cage(&mut self, py: Python, value: bool) }
    bind_python! { self.show_render => fn show_render(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_render = fn set_show_render(&mut self, py: Python, value: bool) }
    bind_python! { self.show_viewport => fn show_viewport(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_viewport = fn set_show_viewport(&mut self, py: Python, value: bool) }
    bind_python! { self.use_apply_on_spline => fn use_apply_on_spline(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_apply_on_spline = fn set_use_apply_on_spline(&mut self, py: Python, value: bool) }
}
