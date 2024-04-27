use super::{
    ArmatureModifier, ArrayModifier, BevelModifier, BooleanModifier, BuildModifier, CastModifier,
    ClothModifier, CollisionModifier, CorrectiveSmoothModifier, CurveModifier,
    DataTransferModifier, DecimateModifier, DisplaceModifier, DynamicPaintModifier,
    EdgeSplitModifier, ExplodeModifier, FluidModifier, HookModifier, LaplaciandeformModifier,
    LaplaciansmoothModifier, LatticeModifier, MaskModifier, MeshCacheModifier, MeshDeformModifier,
    MeshSequenceCacheModifier, MeshToVolumeModifier, MirrorModifier, ModifierImpl,
    MultiresModifier, NodesModifier, NormalEditModifier, OceanModifier, ParticleInstanceModifier,
    ParticleSystemModifier, RemeshModifier, ScrewModifier, ShrinkwrapModifier,
    SimpleDeformModifier, SkinModifier, SmoothModifier, SoftBodyModifier, SolidifyModifier,
    SubsurfModifier, SurfaceDeformModifier, SurfaceModifier, TriangulateModifier,
    UvProjectModifier, UvWarpModifier, VertexWeightEditModifier, VertexWeightMixModifier,
    VertexWeightProximityModifier, VolumeDisplaceModifier, VolumeToMeshModifier, WarpModifier,
    WaveModifier, WeightedNormalModifier, WeldModifier, WireframeModifier,
};
use crate::{
    enums::ModifierType, error::BlError, macros::enum_wrap_inner_fn, result::Result, types::Object,
};
use derive_more::Display;
use pyo3::{intern, PyResult, Python};

/// Wrapper around all supported object modifiers
#[derive(Clone, Debug, Display)]
pub enum Modifier {
    Armature(ArmatureModifier),
    Array(ArrayModifier),
    Bevel(BevelModifier),
    Boolean(BooleanModifier),
    Build(BuildModifier),
    Cast(CastModifier),
    Cloth(ClothModifier),
    Collision(CollisionModifier),
    CorrectiveSmooth(CorrectiveSmoothModifier),
    Curve(CurveModifier),
    DataTransfer(DataTransferModifier),
    Decimate(DecimateModifier),
    Displace(DisplaceModifier),
    DynamicPaint(DynamicPaintModifier),
    EdgeSplit(EdgeSplitModifier),
    Explode(ExplodeModifier),
    Fluid(FluidModifier),
    Hook(HookModifier),
    Laplaciandeform(LaplaciandeformModifier),
    Laplaciansmooth(LaplaciansmoothModifier),
    Lattice(LatticeModifier),
    Mask(MaskModifier),
    MeshCache(MeshCacheModifier),
    MeshDeform(MeshDeformModifier),
    MeshSequenceCache(MeshSequenceCacheModifier),
    MeshToVolume(MeshToVolumeModifier),
    Mirror(MirrorModifier),
    Multires(MultiresModifier),
    Nodes(NodesModifier),
    NormalEdit(NormalEditModifier),
    Ocean(OceanModifier),
    ParticleInstance(ParticleInstanceModifier),
    ParticleSystem(ParticleSystemModifier),
    Remesh(RemeshModifier),
    Screw(ScrewModifier),
    Shrinkwrap(ShrinkwrapModifier),
    SimpleDeform(SimpleDeformModifier),
    Skin(SkinModifier),
    Smooth(SmoothModifier),
    SoftBody(SoftBodyModifier),
    Solidify(SolidifyModifier),
    Subsurf(SubsurfModifier),
    Surface(SurfaceModifier),
    SurfaceDeform(SurfaceDeformModifier),
    Triangulate(TriangulateModifier),
    UvProject(UvProjectModifier),
    UvWarp(UvWarpModifier),
    VertexWeightEdit(VertexWeightEditModifier),
    VertexWeightMix(VertexWeightMixModifier),
    VertexWeightProximity(VertexWeightProximityModifier),
    VolumeDisplace(VolumeDisplaceModifier),
    VolumeToMesh(VolumeToMeshModifier),
    Warp(WarpModifier),
    Wave(WaveModifier),
    WeightedNormal(WeightedNormalModifier),
    Weld(WeldModifier),
    Wireframe(WireframeModifier),
}

impl Modifier {
    pub fn new(py: Python, object: &Object, name: &str, r#type: ModifierType) -> Result<Self> {
        Ok(object.modifiers(py)?.new(py, name, r#type)?)
    }

    pub fn remove(self, py: Python, object: &Object) -> Result<()> {
        Ok(object.modifiers(py)?.remove(py, self)?)
    }

    enum_wrap_inner_fn! {
        { pub fn execution_time(&self, py: Python) -> PyResult<f32> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_execution_time(&mut self, py: Python, value: f32) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn is_active(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_is_active(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn is_override_data(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn name(&self, py: Python) -> PyResult<String> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_name(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn show_expanded(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_show_expanded(&mut self, py: Python, value: &str) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn show_in_editmode(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_show_in_editmode(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn show_on_cage(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_show_on_cage(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn show_render(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_show_render(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn show_viewport(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_show_viewport(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn use_apply_on_spline(&self, py: Python) -> PyResult<bool> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn set_use_apply_on_spline(&mut self, py: Python, value: bool) -> PyResult<()> }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }
}

impl pyo3::ToPyObject for Modifier {
    enum_wrap_inner_fn! {
        { fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject  }
        for [
            Self::Armature, Self::Array, Self::Bevel, Self::Boolean, Self::Build,
            Self::Cast, Self::Cloth, Self::Collision, Self::CorrectiveSmooth,
            Self::Curve, Self::DataTransfer, Self::Decimate, Self::Displace,
            Self::DynamicPaint, Self::EdgeSplit, Self::Explode, Self::Fluid,
            Self::Hook, Self::Laplaciandeform, Self::Laplaciansmooth, Self::Lattice,
            Self::Mask, Self::MeshCache, Self::MeshDeform, Self::MeshSequenceCache,
            Self::MeshToVolume, Self::Mirror, Self::Multires, Self::Nodes,
            Self::NormalEdit, Self::Ocean, Self::ParticleInstance, Self::ParticleSystem,
            Self::Remesh, Self::Screw, Self::Shrinkwrap, Self::SimpleDeform, Self::Skin,
            Self::Smooth, Self::SoftBody, Self::Solidify, Self::Subsurf, Self::Surface,
            Self::SurfaceDeform, Self::Triangulate, Self::UvProject, Self::UvWarp,
            Self::VertexWeightEdit, Self::VertexWeightMix, Self::VertexWeightProximity,
            Self::VolumeDisplace, Self::VolumeToMesh, Self::Warp, Self::Wave,
            Self::WeightedNormal, Self::Weld, Self::Wireframe,
        ]
    }
}

impl pyo3::FromPyObject<'_> for Modifier {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        let modifier_type: ModifierType = value.getattr(intern!(value.py(), "type"))?.extract()?;
        Ok(match modifier_type {
            ModifierType::Armature => Self::Armature(value.into()),
            ModifierType::Array => Self::Array(value.into()),
            ModifierType::Bevel => Self::Bevel(value.into()),
            ModifierType::Boolean => Self::Boolean(value.into()),
            ModifierType::Build => Self::Build(value.into()),
            ModifierType::Cast => Self::Cast(value.into()),
            ModifierType::Cloth => Self::Cloth(value.into()),
            ModifierType::Collision => Self::Collision(value.into()),
            ModifierType::CorrectiveSmooth => Self::CorrectiveSmooth(value.into()),
            ModifierType::Curve => Self::Curve(value.into()),
            ModifierType::DataTransfer => Self::DataTransfer(value.into()),
            ModifierType::Decimate => Self::Decimate(value.into()),
            ModifierType::Displace => Self::Displace(value.into()),
            ModifierType::DynamicPaint => Self::DynamicPaint(value.into()),
            ModifierType::EdgeSplit => Self::EdgeSplit(value.into()),
            ModifierType::Explode => Self::Explode(value.into()),
            ModifierType::Fluid => Self::Fluid(value.into()),
            ModifierType::Hook => Self::Hook(value.into()),
            ModifierType::Laplaciandeform => Self::Laplaciandeform(value.into()),
            ModifierType::Laplaciansmooth => Self::Laplaciansmooth(value.into()),
            ModifierType::Lattice => Self::Lattice(value.into()),
            ModifierType::Mask => Self::Mask(value.into()),
            ModifierType::MeshCache => Self::MeshCache(value.into()),
            ModifierType::MeshDeform => Self::MeshDeform(value.into()),
            ModifierType::MeshSequenceCache => Self::MeshSequenceCache(value.into()),
            ModifierType::MeshToVolume => Self::MeshToVolume(value.into()),
            ModifierType::Mirror => Self::Mirror(value.into()),
            ModifierType::Multires => Self::Multires(value.into()),
            ModifierType::Nodes => Self::Nodes(value.into()),
            ModifierType::NormalEdit => Self::NormalEdit(value.into()),
            ModifierType::Ocean => Self::Ocean(value.into()),
            ModifierType::ParticleInstance => Self::ParticleInstance(value.into()),
            ModifierType::ParticleSystem => Self::ParticleSystem(value.into()),
            ModifierType::Remesh => Self::Remesh(value.into()),
            ModifierType::Screw => Self::Screw(value.into()),
            ModifierType::Shrinkwrap => Self::Shrinkwrap(value.into()),
            ModifierType::SimpleDeform => Self::SimpleDeform(value.into()),
            ModifierType::Skin => Self::Skin(value.into()),
            ModifierType::Smooth => Self::Smooth(value.into()),
            ModifierType::SoftBody => Self::SoftBody(value.into()),
            ModifierType::Solidify => Self::Solidify(value.into()),
            ModifierType::Subsurf => Self::Subsurf(value.into()),
            ModifierType::Surface => Self::Surface(value.into()),
            ModifierType::SurfaceDeform => Self::SurfaceDeform(value.into()),
            ModifierType::Triangulate => Self::Triangulate(value.into()),
            ModifierType::UvProject => Self::UvProject(value.into()),
            ModifierType::UvWarp => Self::UvWarp(value.into()),
            ModifierType::VertexWeightEdit => Self::VertexWeightEdit(value.into()),
            ModifierType::VertexWeightMix => Self::VertexWeightMix(value.into()),
            ModifierType::VertexWeightProximity => Self::VertexWeightProximity(value.into()),
            ModifierType::VolumeDisplace => Self::VolumeDisplace(value.into()),
            ModifierType::VolumeToMesh => Self::VolumeToMesh(value.into()),
            ModifierType::Warp => Self::Warp(value.into()),
            ModifierType::Wave => Self::Wave(value.into()),
            ModifierType::WeightedNormal => Self::WeightedNormal(value.into()),
            ModifierType::Weld => Self::Weld(value.into()),
            ModifierType::Wireframe => Self::Wireframe(value.into()),
        })
    }
}

impl From<Modifier> for ModifierType {
    fn from(value: Modifier) -> Self {
        match value {
            Modifier::Armature(_) => Self::Armature,
            Modifier::Array(_) => Self::Array,
            Modifier::Bevel(_) => Self::Bevel,
            Modifier::Boolean(_) => Self::Boolean,
            Modifier::Build(_) => Self::Build,
            Modifier::Cast(_) => Self::Cast,
            Modifier::Cloth(_) => Self::Cloth,
            Modifier::Collision(_) => Self::Collision,
            Modifier::CorrectiveSmooth(_) => Self::CorrectiveSmooth,
            Modifier::Curve(_) => Self::Curve,
            Modifier::DataTransfer(_) => Self::DataTransfer,
            Modifier::Decimate(_) => Self::Decimate,
            Modifier::Displace(_) => Self::Displace,
            Modifier::DynamicPaint(_) => Self::DynamicPaint,
            Modifier::EdgeSplit(_) => Self::EdgeSplit,
            Modifier::Explode(_) => Self::Explode,
            Modifier::Fluid(_) => Self::Fluid,
            Modifier::Hook(_) => Self::Hook,
            Modifier::Laplaciandeform(_) => Self::Laplaciandeform,
            Modifier::Laplaciansmooth(_) => Self::Laplaciansmooth,
            Modifier::Lattice(_) => Self::Lattice,
            Modifier::Mask(_) => Self::Mask,
            Modifier::MeshCache(_) => Self::MeshCache,
            Modifier::MeshDeform(_) => Self::MeshDeform,
            Modifier::MeshSequenceCache(_) => Self::MeshSequenceCache,
            Modifier::MeshToVolume(_) => Self::MeshToVolume,
            Modifier::Mirror(_) => Self::Mirror,
            Modifier::Multires(_) => Self::Multires,
            Modifier::Nodes(_) => Self::Nodes,
            Modifier::NormalEdit(_) => Self::NormalEdit,
            Modifier::Ocean(_) => Self::Ocean,
            Modifier::ParticleInstance(_) => Self::ParticleInstance,
            Modifier::ParticleSystem(_) => Self::ParticleSystem,
            Modifier::Remesh(_) => Self::Remesh,
            Modifier::Screw(_) => Self::Screw,
            Modifier::Shrinkwrap(_) => Self::Shrinkwrap,
            Modifier::SimpleDeform(_) => Self::SimpleDeform,
            Modifier::Skin(_) => Self::Skin,
            Modifier::Smooth(_) => Self::Smooth,
            Modifier::SoftBody(_) => Self::SoftBody,
            Modifier::Solidify(_) => Self::Solidify,
            Modifier::Subsurf(_) => Self::Subsurf,
            Modifier::Surface(_) => Self::Surface,
            Modifier::SurfaceDeform(_) => Self::SurfaceDeform,
            Modifier::Triangulate(_) => Self::Triangulate,
            Modifier::UvProject(_) => Self::UvProject,
            Modifier::UvWarp(_) => Self::UvWarp,
            Modifier::VertexWeightEdit(_) => Self::VertexWeightEdit,
            Modifier::VertexWeightMix(_) => Self::VertexWeightMix,
            Modifier::VertexWeightProximity(_) => Self::VertexWeightProximity,
            Modifier::VolumeDisplace(_) => Self::VolumeDisplace,
            Modifier::VolumeToMesh(_) => Self::VolumeToMesh,
            Modifier::Warp(_) => Self::Warp,
            Modifier::Wave(_) => Self::Wave,
            Modifier::WeightedNormal(_) => Self::WeightedNormal,
            Modifier::Weld(_) => Self::Weld,
            Modifier::Wireframe(_) => Self::Wireframe,
        }
    }
}

impl From<&Modifier> for ModifierType {
    fn from(value: &Modifier) -> Self {
        match value {
            Modifier::Armature(_) => Self::Armature,
            Modifier::Array(_) => Self::Array,
            Modifier::Bevel(_) => Self::Bevel,
            Modifier::Boolean(_) => Self::Boolean,
            Modifier::Build(_) => Self::Build,
            Modifier::Cast(_) => Self::Cast,
            Modifier::Cloth(_) => Self::Cloth,
            Modifier::Collision(_) => Self::Collision,
            Modifier::CorrectiveSmooth(_) => Self::CorrectiveSmooth,
            Modifier::Curve(_) => Self::Curve,
            Modifier::DataTransfer(_) => Self::DataTransfer,
            Modifier::Decimate(_) => Self::Decimate,
            Modifier::Displace(_) => Self::Displace,
            Modifier::DynamicPaint(_) => Self::DynamicPaint,
            Modifier::EdgeSplit(_) => Self::EdgeSplit,
            Modifier::Explode(_) => Self::Explode,
            Modifier::Fluid(_) => Self::Fluid,
            Modifier::Hook(_) => Self::Hook,
            Modifier::Laplaciandeform(_) => Self::Laplaciandeform,
            Modifier::Laplaciansmooth(_) => Self::Laplaciansmooth,
            Modifier::Lattice(_) => Self::Lattice,
            Modifier::Mask(_) => Self::Mask,
            Modifier::MeshCache(_) => Self::MeshCache,
            Modifier::MeshDeform(_) => Self::MeshDeform,
            Modifier::MeshSequenceCache(_) => Self::MeshSequenceCache,
            Modifier::MeshToVolume(_) => Self::MeshToVolume,
            Modifier::Mirror(_) => Self::Mirror,
            Modifier::Multires(_) => Self::Multires,
            Modifier::Nodes(_) => Self::Nodes,
            Modifier::NormalEdit(_) => Self::NormalEdit,
            Modifier::Ocean(_) => Self::Ocean,
            Modifier::ParticleInstance(_) => Self::ParticleInstance,
            Modifier::ParticleSystem(_) => Self::ParticleSystem,
            Modifier::Remesh(_) => Self::Remesh,
            Modifier::Screw(_) => Self::Screw,
            Modifier::Shrinkwrap(_) => Self::Shrinkwrap,
            Modifier::SimpleDeform(_) => Self::SimpleDeform,
            Modifier::Skin(_) => Self::Skin,
            Modifier::Smooth(_) => Self::Smooth,
            Modifier::SoftBody(_) => Self::SoftBody,
            Modifier::Solidify(_) => Self::Solidify,
            Modifier::Subsurf(_) => Self::Subsurf,
            Modifier::Surface(_) => Self::Surface,
            Modifier::SurfaceDeform(_) => Self::SurfaceDeform,
            Modifier::Triangulate(_) => Self::Triangulate,
            Modifier::UvProject(_) => Self::UvProject,
            Modifier::UvWarp(_) => Self::UvWarp,
            Modifier::VertexWeightEdit(_) => Self::VertexWeightEdit,
            Modifier::VertexWeightMix(_) => Self::VertexWeightMix,
            Modifier::VertexWeightProximity(_) => Self::VertexWeightProximity,
            Modifier::VolumeDisplace(_) => Self::VolumeDisplace,
            Modifier::VolumeToMesh(_) => Self::VolumeToMesh,
            Modifier::Warp(_) => Self::Warp,
            Modifier::Wave(_) => Self::Wave,
            Modifier::WeightedNormal(_) => Self::WeightedNormal,
            Modifier::Weld(_) => Self::Weld,
            Modifier::Wireframe(_) => Self::Wireframe,
        }
    }
}

macro_rules! impl_modifier_conversions_for {
    ($modifier_type:path, $modifier_variant:path, $modifier_struct:ty) => {
        impl From<$modifier_struct> for Modifier {
            fn from(value: $modifier_struct) -> Self {
                $modifier_variant(value)
            }
        }

        impl TryFrom<Modifier> for $modifier_struct {
            type Error = BlError;

            fn try_from(value: Modifier) -> Result<Self> {
                match value {
                    $modifier_variant(value) => Ok(value),
                    incorrect_type => Err(BlError::TypeError(format!(
                        "Blender modifier {name} is of type {incorrect_type}, but {correct_type} is expected",
                        name = Python::with_gil(|py| incorrect_type.name(py))?,
                        incorrect_type = Into::<ModifierType>::into(&incorrect_type),
                        correct_type = $modifier_type,
                    ))),
                }
            }
        }
    };
}
impl_modifier_conversions_for!(ModifierType::Armature, Modifier::Armature, ArmatureModifier);
impl_modifier_conversions_for!(ModifierType::Array, Modifier::Array, ArrayModifier);
impl_modifier_conversions_for!(ModifierType::Bevel, Modifier::Bevel, BevelModifier);
impl_modifier_conversions_for!(ModifierType::Boolean, Modifier::Boolean, BooleanModifier);
impl_modifier_conversions_for!(ModifierType::Build, Modifier::Build, BuildModifier);
impl_modifier_conversions_for!(ModifierType::Cast, Modifier::Cast, CastModifier);
impl_modifier_conversions_for!(ModifierType::Cloth, Modifier::Cloth, ClothModifier);
impl_modifier_conversions_for!(
    ModifierType::Collision,
    Modifier::Collision,
    CollisionModifier
);
impl_modifier_conversions_for!(
    ModifierType::CorrectiveSmooth,
    Modifier::CorrectiveSmooth,
    CorrectiveSmoothModifier
);
impl_modifier_conversions_for!(ModifierType::Curve, Modifier::Curve, CurveModifier);
impl_modifier_conversions_for!(
    ModifierType::DataTransfer,
    Modifier::DataTransfer,
    DataTransferModifier
);
impl_modifier_conversions_for!(ModifierType::Decimate, Modifier::Decimate, DecimateModifier);
impl_modifier_conversions_for!(ModifierType::Displace, Modifier::Displace, DisplaceModifier);
impl_modifier_conversions_for!(
    ModifierType::DynamicPaint,
    Modifier::DynamicPaint,
    DynamicPaintModifier
);
impl_modifier_conversions_for!(
    ModifierType::EdgeSplit,
    Modifier::EdgeSplit,
    EdgeSplitModifier
);
impl_modifier_conversions_for!(ModifierType::Explode, Modifier::Explode, ExplodeModifier);
impl_modifier_conversions_for!(ModifierType::Fluid, Modifier::Fluid, FluidModifier);
impl_modifier_conversions_for!(ModifierType::Hook, Modifier::Hook, HookModifier);
impl_modifier_conversions_for!(
    ModifierType::Laplaciandeform,
    Modifier::Laplaciandeform,
    LaplaciandeformModifier
);
impl_modifier_conversions_for!(
    ModifierType::Laplaciansmooth,
    Modifier::Laplaciansmooth,
    LaplaciansmoothModifier
);
impl_modifier_conversions_for!(ModifierType::Lattice, Modifier::Lattice, LatticeModifier);
impl_modifier_conversions_for!(ModifierType::Mask, Modifier::Mask, MaskModifier);
impl_modifier_conversions_for!(
    ModifierType::MeshCache,
    Modifier::MeshCache,
    MeshCacheModifier
);
impl_modifier_conversions_for!(
    ModifierType::MeshDeform,
    Modifier::MeshDeform,
    MeshDeformModifier
);
impl_modifier_conversions_for!(
    ModifierType::MeshSequenceCache,
    Modifier::MeshSequenceCache,
    MeshSequenceCacheModifier
);
impl_modifier_conversions_for!(
    ModifierType::MeshToVolume,
    Modifier::MeshToVolume,
    MeshToVolumeModifier
);
impl_modifier_conversions_for!(ModifierType::Mirror, Modifier::Mirror, MirrorModifier);
impl_modifier_conversions_for!(ModifierType::Multires, Modifier::Multires, MultiresModifier);
impl_modifier_conversions_for!(ModifierType::Nodes, Modifier::Nodes, NodesModifier);
impl_modifier_conversions_for!(
    ModifierType::NormalEdit,
    Modifier::NormalEdit,
    NormalEditModifier
);
impl_modifier_conversions_for!(ModifierType::Ocean, Modifier::Ocean, OceanModifier);
impl_modifier_conversions_for!(
    ModifierType::ParticleInstance,
    Modifier::ParticleInstance,
    ParticleInstanceModifier
);
impl_modifier_conversions_for!(
    ModifierType::ParticleSystem,
    Modifier::ParticleSystem,
    ParticleSystemModifier
);
impl_modifier_conversions_for!(ModifierType::Remesh, Modifier::Remesh, RemeshModifier);
impl_modifier_conversions_for!(ModifierType::Screw, Modifier::Screw, ScrewModifier);
impl_modifier_conversions_for!(
    ModifierType::Shrinkwrap,
    Modifier::Shrinkwrap,
    ShrinkwrapModifier
);
impl_modifier_conversions_for!(
    ModifierType::SimpleDeform,
    Modifier::SimpleDeform,
    SimpleDeformModifier
);
impl_modifier_conversions_for!(ModifierType::Skin, Modifier::Skin, SkinModifier);
impl_modifier_conversions_for!(ModifierType::Smooth, Modifier::Smooth, SmoothModifier);
impl_modifier_conversions_for!(ModifierType::SoftBody, Modifier::SoftBody, SoftBodyModifier);
impl_modifier_conversions_for!(ModifierType::Solidify, Modifier::Solidify, SolidifyModifier);
impl_modifier_conversions_for!(ModifierType::Subsurf, Modifier::Subsurf, SubsurfModifier);
impl_modifier_conversions_for!(ModifierType::Surface, Modifier::Surface, SurfaceModifier);
impl_modifier_conversions_for!(
    ModifierType::SurfaceDeform,
    Modifier::SurfaceDeform,
    SurfaceDeformModifier
);
impl_modifier_conversions_for!(
    ModifierType::Triangulate,
    Modifier::Triangulate,
    TriangulateModifier
);
impl_modifier_conversions_for!(
    ModifierType::UvProject,
    Modifier::UvProject,
    UvProjectModifier
);
impl_modifier_conversions_for!(ModifierType::UvWarp, Modifier::UvWarp, UvWarpModifier);
impl_modifier_conversions_for!(
    ModifierType::VertexWeightEdit,
    Modifier::VertexWeightEdit,
    VertexWeightEditModifier
);
impl_modifier_conversions_for!(
    ModifierType::VertexWeightMix,
    Modifier::VertexWeightMix,
    VertexWeightMixModifier
);
impl_modifier_conversions_for!(
    ModifierType::VertexWeightProximity,
    Modifier::VertexWeightProximity,
    VertexWeightProximityModifier
);
impl_modifier_conversions_for!(
    ModifierType::VolumeDisplace,
    Modifier::VolumeDisplace,
    VolumeDisplaceModifier
);
impl_modifier_conversions_for!(
    ModifierType::VolumeToMesh,
    Modifier::VolumeToMesh,
    VolumeToMeshModifier
);
impl_modifier_conversions_for!(ModifierType::Warp, Modifier::Warp, WarpModifier);
impl_modifier_conversions_for!(ModifierType::Wave, Modifier::Wave, WaveModifier);
impl_modifier_conversions_for!(
    ModifierType::WeightedNormal,
    Modifier::WeightedNormal,
    WeightedNormalModifier
);
impl_modifier_conversions_for!(ModifierType::Weld, Modifier::Weld, WeldModifier);
impl_modifier_conversions_for!(
    ModifierType::Wireframe,
    Modifier::Wireframe,
    WireframeModifier
);
