//! Utilities for importing models and scenes.

use crate::{bpy, enums::DirectionXYZ, error::BlError, macros::enum_wrap_inner_fn, result::Result};
use builder_derive_more::IntoBuilder;
use derive_builder::Builder;
use pyo3::{
    intern,
    types::{IntoPyDict, PyAny},
    Python,
};
use std::{convert::From, fmt::Debug, path::Path};

pub trait BlendImport: IntoPyDict + Sized {
    const EXTENSIONS: &'static [&'static str];

    fn fn_import(py: Python) -> Result<&PyAny>;

    fn import(self, filepath: impl AsRef<Path>) -> Result<()> {
        let filepath = filepath.as_ref();
        self.check_import_filepath(filepath)?;

        Python::with_gil(|py| -> Result<()> {
            let kwargs = self.into_py_dict(py);
            kwargs.set_item(intern!(py, "filepath"), filepath)?;

            Self::fn_import(py)?.call((), Some(kwargs))?;
            Ok(())
        })?;

        Ok(())
    }

    fn check_import_filepath(&self, filepath: impl AsRef<Path>) -> Result<()> {
        let filepath = filepath.as_ref();
        if !filepath.is_file() {
            return Err(BlError::ValueError(format!(
                "Filepath '{}' does not point to a valid file.",
                filepath.display()
            )));
        }

        Ok(())
    }
}

pub enum BlendImporter {
    Abc(AbcImporter),
    Dae(DaeImporter),
    Fbx(FbxImporter),
    Gltf(GltfImporter),
    Obj(ObjImporter),
    Ply(PlyImporter),
    Stl(StlImporter),
    Svg(SvgImporter),
    Usd(UsdImporter),
    X3d(X3dImporter),
}

impl BlendImporter {
    pub fn from_filepath_extension(filepath: impl AsRef<Path>) -> Result<Self> {
        let filepath = filepath.as_ref();
        match filepath.extension() {
            Some(extension) => {
                let ext = extension.to_ascii_lowercase();
                let ext = ext.to_str().unwrap();
                match ext {
                    ext if AbcImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Abc(AbcImporter::default()))
                    }
                    ext if DaeImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Dae(DaeImporter::default()))
                    }
                    ext if FbxImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Fbx(FbxImporter::default()))
                    }
                    ext if GltfImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Gltf(GltfImporter::default()))
                    }
                    ext if ObjImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Obj(ObjImporter::default()))
                    }
                    ext if PlyImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Ply(PlyImporter::default()))
                    }
                    ext if StlImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Stl(StlImporter::default()))
                    }
                    ext if SvgImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Svg(SvgImporter::default()))
                    }
                    ext if UsdImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Usd(UsdImporter::default()))
                    }
                    ext if X3dImporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::X3d(X3dImporter::default()))
                    }
                    _ => Err(BlError::ValueError(format!(
                        "Invalid file extension for importing from Blender: '{ext}'"
                    ))),
                }
            }
            None => Err(BlError::ValueError(format!(
                "Import format cannot be determined from filepath: '{}'",
                filepath.display()
            ))),
        }
    }

    enum_wrap_inner_fn! {
        { pub fn import(self, filepath: impl AsRef<Path>) -> Result<()> }
        for [
            Self::Abc, Self::Dae, Self::Fbx, Self::Gltf, Self::Obj,
            Self::Ply, Self::Stl, Self::Svg, Self::Usd, Self::X3d,
        ]
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct AbcImporter {
    pub scale: f32,
    pub set_frame_range: bool,
    pub validate_meshes: bool,
    pub always_add_cache_reader: bool,
    pub is_sequence: bool,
}

impl AbcImporterBuilder {
    #[must_use]
    pub fn build(&self) -> AbcImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for AbcImporter {
    fn default() -> Self {
        Self {
            scale: 1.0,
            set_frame_range: true,
            validate_meshes: false,
            always_add_cache_reader: false,
            is_sequence: false,
        }
    }
}

impl BlendImport for AbcImporter {
    const EXTENSIONS: &'static [&'static str] = &["abc"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_alembic_import(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct DaeImporter {
    pub import_units: bool,
    pub custom_normals: bool,
    pub fix_orientation: bool,
    pub find_chains: bool,
    pub auto_connect: bool,
    pub min_chain_length: u32,
    pub keep_bind_info: bool,
}

impl DaeImporterBuilder {
    #[must_use]
    pub fn build(&self) -> DaeImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for DaeImporter {
    fn default() -> Self {
        Self {
            import_units: false,
            custom_normals: true,
            fix_orientation: false,
            find_chains: false,
            auto_connect: false,
            min_chain_length: 0,
            keep_bind_info: false,
        }
    }
}

impl BlendImport for DaeImporter {
    const EXTENSIONS: &'static [&'static str] = &["dae"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_collada_import(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct FbxImporter {
    pub use_manual_orientation: bool,
    pub global_scale: f32,
    pub bake_space_transform: bool,
    pub use_custom_normals: bool,
    #[builder(setter(into))]
    pub colors_type: String,
    pub use_image_search: bool,
    pub use_alpha_decals: bool,
    pub decal_offset: f32,
    pub use_anim: bool,
    pub anim_offset: f32,
    pub use_subsurf: bool,
    pub use_custom_props: bool,
    pub use_custom_props_enum_as_string: bool,
    pub ignore_leaf_bones: bool,
    pub force_connect_children: bool,
    pub automatic_bone_orientation: bool,
    pub primary_bone_axis: DirectionXYZ,
    pub secondary_bone_axis: DirectionXYZ,
    pub use_prepost_rot: bool,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl FbxImporterBuilder {
    #[must_use]
    pub fn build(&self) -> FbxImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for FbxImporter {
    fn default() -> Self {
        Self {
            use_manual_orientation: false,
            global_scale: 1.0,
            bake_space_transform: false,
            use_custom_normals: true,
            colors_type: "SRGB".to_string(),
            use_image_search: true,
            use_alpha_decals: false,
            decal_offset: 0.0,
            use_anim: true,
            anim_offset: 1.0,
            use_subsurf: false,
            use_custom_props: true,
            use_custom_props_enum_as_string: true,
            ignore_leaf_bones: false,
            force_connect_children: false,
            automatic_bone_orientation: false,
            primary_bone_axis: DirectionXYZ::PosY,
            secondary_bone_axis: DirectionXYZ::PosX,
            use_prepost_rot: true,
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendImport for FbxImporter {
    const EXTENSIONS: &'static [&'static str] = &["fbx"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_scene::fn_fbx(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct GltfImporter {
    #[builder(setter(into))]
    pub export_import_convert_lighting_mode: String,
    pub import_pack_images: bool,
    pub merge_vertices: bool,
    #[builder(setter(into))]
    pub import_shading: String,
    #[builder(setter(into))]
    pub bone_heuristic: String,
    pub guess_original_bind_pose: bool,
}

impl GltfImporterBuilder {
    #[must_use]
    pub fn build(&self) -> GltfImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for GltfImporter {
    fn default() -> Self {
        Self {
            export_import_convert_lighting_mode: "SPEC".to_string(),
            import_pack_images: true,
            merge_vertices: false,
            import_shading: "NORMALS".to_string(),
            bone_heuristic: "TEMPERANCE".to_string(),
            guess_original_bind_pose: true,
        }
    }
}

impl BlendImport for GltfImporter {
    const EXTENSIONS: &'static [&'static str] = &["gltf", "glb"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_scene::fn_gltf(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct ObjImporter {
    pub global_scale: f32,
    pub clamp_size: f32,
    pub forward_axis: DirectionXYZ,
    pub up_axis: DirectionXYZ,
    pub use_split_objects: bool,
    pub use_split_groups: bool,
    pub import_vertex_groups: bool,
    pub validate_meshes: bool,
}

impl ObjImporterBuilder {
    #[must_use]
    pub fn build(&self) -> ObjImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for ObjImporter {
    fn default() -> Self {
        Self {
            global_scale: 1.0,
            clamp_size: 0.0,
            forward_axis: DirectionXYZ::PosY,
            up_axis: DirectionXYZ::PosZ,
            use_split_objects: true,
            use_split_groups: false,
            import_vertex_groups: false,
            validate_meshes: false,
        }
    }
}

impl BlendImport for ObjImporter {
    const EXTENSIONS: &'static [&'static str] = &["obj", "mtl"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_scene::fn_obj(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_mesh.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict, Default)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct PlyImporter {}

impl BlendImport for PlyImporter {
    const EXTENSIONS: &'static [&'static str] = &["ply"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_mesh::fn_ply(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_mesh.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct StlImporter {
    pub global_scale: f32,
    pub use_scene_unit: bool,
    pub use_facet_normal: bool,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl StlImporterBuilder {
    #[must_use]
    pub fn build(&self) -> StlImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for StlImporter {
    fn default() -> Self {
        Self {
            global_scale: 1.0,
            use_scene_unit: false,
            use_facet_normal: false,
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendImport for StlImporter {
    const EXTENSIONS: &'static [&'static str] = &["stl"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_mesh::fn_stl(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_curve.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict, Default)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct SvgImporter {}

impl BlendImport for SvgImporter {
    const EXTENSIONS: &'static [&'static str] = &["svg"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_curve::fn_svg(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct UsdImporter {
    pub scale: f32,
    pub set_frame_range: bool,
    pub import_cameras: bool,
    pub import_curves: bool,
    pub import_lights: bool,
    pub import_materials: bool,
    pub import_meshes: bool,
    pub import_volumes: bool,
    pub import_shapes: bool,
    pub import_subdiv: bool,
    pub import_instance_proxies: bool,
    pub import_visible_only: bool,
    pub create_collection: bool,
    pub read_mesh_uvs: bool,
    pub read_mesh_colors: bool,
    #[builder(setter(into))]
    pub prim_path_mask: String,
    pub import_guide: bool,
    pub import_proxy: bool,
    pub import_render: bool,
    pub import_all_materials: bool,
    pub import_usd_preview: bool,
    pub set_material_blend: bool,
    pub light_intensity_scale: f32,
    #[builder(setter(into))]
    pub mtl_name_collision_mode: String,
    #[builder(setter(into))]
    pub import_textures_mode: String,
    #[builder(setter(into))]
    pub import_textures_dir: String,
    #[builder(setter(into))]
    pub tex_name_collision_mode: String,
}

impl UsdImporterBuilder {
    #[must_use]
    pub fn build(&self) -> UsdImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for UsdImporter {
    fn default() -> Self {
        Self {
            scale: 1.0,
            set_frame_range: true,
            import_cameras: true,
            import_curves: true,
            import_lights: true,
            import_materials: true,
            import_meshes: true,
            import_volumes: true,
            import_shapes: true,
            import_subdiv: false,
            import_instance_proxies: true,
            import_visible_only: true,
            create_collection: false,
            read_mesh_uvs: true,
            read_mesh_colors: true,
            prim_path_mask: String::new(),
            import_guide: false,
            import_proxy: true,
            import_render: true,
            import_all_materials: false,
            import_usd_preview: true,
            set_material_blend: true,
            light_intensity_scale: 1.0,
            mtl_name_collision_mode: "MAKE_UNIQUE".to_string(),
            import_textures_mode: "IMPORT_PACK".to_string(),
            import_textures_dir: "//textures/".to_string(),
            tex_name_collision_mode: "USE_EXISTING".to_string(),
        }
    }
}

impl BlendImport for UsdImporter {
    const EXTENSIONS: &'static [&'static str] = &["usd", "usda", "usdc", "usdz"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_usd_import(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct X3dImporter {
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl X3dImporterBuilder {
    #[must_use]
    pub fn build(&self) -> X3dImporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for X3dImporter {
    fn default() -> Self {
        Self {
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendImport for X3dImporter {
    const EXTENSIONS: &'static [&'static str] = &["x3d", "wrl"];

    fn fn_import(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::import_scene::fn_x3d(py)?)
    }
}
