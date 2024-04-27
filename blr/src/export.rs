//! Utilities for exporting models and scenes.

use crate::{bpy, enums::DirectionXYZ, error::BlError, macros::enum_wrap_inner_fn, result::Result};
use builder_derive_more::IntoBuilder;
use derive_builder::Builder;
use pyo3::{
    intern,
    types::{IntoPyDict, PyAny},
    Python,
};
use std::{
    convert::From,
    fmt::Debug,
    path::{Path, PathBuf},
};

pub trait BlendExport: IntoPyDict + Sized {
    const EXTENSIONS: &'static [&'static str];

    fn extension(&self) -> &str;

    fn fn_export(py: Python) -> Result<&PyAny>;

    fn export(self, filepath: impl AsRef<Path>) -> Result<PathBuf> {
        let filepath = self.check_export_filepath(filepath)?;

        Python::with_gil(|py| -> Result<()> {
            let kwargs = self.into_py_dict(py);
            kwargs.set_item(intern!(py, "filepath"), &filepath)?;

            Self::fn_export(py)?.call((), Some(kwargs))?;
            Ok(())
        })?;

        Ok(filepath)
    }

    fn check_export_filepath(&self, filepath: impl AsRef<Path>) -> Result<PathBuf> {
        let filepath = filepath.as_ref();

        if filepath.as_os_str().is_empty() {
            return Err(BlError::ValueError("Filepath cannot be empty".to_string()));
        }

        if filepath.is_dir() {
            return Err(BlError::ValueError(format!(
                "Filepath cannot be a directory: '{}'",
                filepath.display()
            )));
        }

        match filepath.extension() {
            Some(valid_extr)
                if Self::EXTENSIONS
                    .iter()
                    .any(|ext| ext.eq_ignore_ascii_case(valid_extr.to_str().unwrap())) =>
            {
                Ok(filepath.into())
            }
            Some(invalid_ext) => Err(BlError::ValueError(format!(
                "Invalid file extension (expected: '{expected_ext}', actual: '{invalid_ext}')",
                expected_ext = self.extension(),
                invalid_ext = invalid_ext.to_str().unwrap()
            ))),
            _ => Ok(filepath.with_extension(self.extension())),
        }
    }
}

pub enum BlendExporter {
    Abc(AbcExporter),
    Dae(DaeExporter),
    Fbx(FbxExporter),
    Gltf(GltfExporter),
    Obj(ObjExporter),
    Ply(PlyExporter),
    Stl(StlExporter),
    Usd(UsdExporter),
    X3d(X3dExporter),
}

impl BlendExporter {
    pub fn from_filepath_extension(filepath: impl AsRef<Path>) -> Result<Self> {
        let filepath = filepath.as_ref();
        match filepath.extension() {
            Some(extension) => {
                let ext = extension.to_ascii_lowercase();
                let ext = ext.to_str().unwrap();
                match ext {
                    ext if AbcExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Abc(AbcExporter::default()))
                    }
                    ext if DaeExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Dae(DaeExporter::default()))
                    }
                    ext if FbxExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Fbx(FbxExporter::default()))
                    }
                    ext if GltfExporter::EXTENSIONS.contains(&ext) => Ok(Self::Gltf(match ext {
                        "gltf" => GltfExporter::builder()
                            .export_format("GLTF_EMBEDDED")
                            .build(),
                        "glb" => GltfExporter::builder().export_format("GLB").build(),
                        _ => unreachable!(),
                    })),
                    ext if ObjExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Obj(ObjExporter::default()))
                    }
                    ext if PlyExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Ply(PlyExporter::default()))
                    }
                    ext if StlExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Stl(StlExporter::default()))
                    }
                    ext if UsdExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::Usd(UsdExporter::default()))
                    }
                    ext if X3dExporter::EXTENSIONS.contains(&ext) => {
                        Ok(Self::X3d(X3dExporter::default()))
                    }
                    _ => Err(BlError::ValueError(format!(
                        "Invalid file extension for exporting from Blender: '{ext}'"
                    ))),
                }
            }
            None => Err(BlError::ValueError(format!(
                "Export format cannot be determined from filepath: '{}'",
                filepath.display()
            ))),
        }
    }

    enum_wrap_inner_fn! {
        { pub fn extension(&self) -> &str }
        for [
            Self::Abc, Self::Dae, Self::Fbx, Self::Gltf, Self::Obj,
            Self::Ply, Self::Stl, Self::Usd, Self::X3d,
        ]
    }

    enum_wrap_inner_fn! {
        { pub fn export(self, filepath: impl AsRef<Path>) -> Result<PathBuf> }
        for [
            Self::Abc, Self::Dae, Self::Fbx, Self::Gltf, Self::Obj,
            Self::Ply, Self::Stl, Self::Usd, Self::X3d,
        ]
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct AbcExporter {
    pub start: i32,
    pub end: i32,
    pub xsamples: u32,
    pub gsamples: u32,
    pub sh_open: f32,
    pub sh_close: f32,
    pub selected: bool,
    pub visible_objects_only: bool,
    pub flatten: bool,
    pub uvs: bool,
    pub packuv: bool,
    pub normals: bool,
    pub vcolors: bool,
    pub orcos: bool,
    pub face_sets: bool,
    pub subdiv_schema: bool,
    pub apply_subdiv: bool,
    pub curves_as_mesh: bool,
    pub use_instancing: bool,
    pub global_scale: f32,
    pub triangulate: bool,
    #[builder(setter(into))]
    pub quad_method: String,
    #[builder(setter(into))]
    pub ngon_method: String,
    pub export_hair: bool,
    pub export_particles: bool,
    pub export_custom_properties: bool,
    pub as_background_job: bool,
    #[builder(setter(into))]
    pub evaluation_mode: String,
    pub init_scene_frame_range: bool,
}

impl AbcExporterBuilder {
    #[must_use]
    pub fn build(&self) -> AbcExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for AbcExporter {
    fn default() -> Self {
        Self {
            start: -2_147_483_648,
            end: -2_147_483_648,
            xsamples: 1,
            gsamples: 1,
            sh_open: 0.0,
            sh_close: 1.0,
            selected: false,
            visible_objects_only: false,
            flatten: false,
            uvs: true,
            packuv: true,
            normals: true,
            vcolors: false,
            orcos: true,
            face_sets: false,
            subdiv_schema: false,
            apply_subdiv: false,
            curves_as_mesh: false,
            use_instancing: true,
            global_scale: 1.0,
            triangulate: false,
            quad_method: "SHORTEST_DIAGONAL".to_string(),
            ngon_method: "BEAUTY".to_string(),
            export_hair: true,
            export_particles: true,
            export_custom_properties: true,
            as_background_job: false,
            evaluation_mode: "RENDER".to_string(),
            init_scene_frame_range: true,
        }
    }
}

impl BlendExport for AbcExporter {
    const EXTENSIONS: &'static [&'static str] = &["abc"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_alembic_export(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct DaeExporter {
    #[builder(setter(into))]
    pub prop_bc_export_ui_section: String,
    pub apply_modifiers: bool,
    pub export_mesh_type: u32,
    #[builder(setter(into))]
    pub export_mesh_type_selection: String,
    pub export_global_forward_selection: DirectionXYZ,
    pub export_global_up_selection: DirectionXYZ,
    pub apply_global_orientation: bool,
    pub selected: bool,
    pub include_children: bool,
    pub include_armatures: bool,
    pub include_shapekeys: bool,
    pub deform_bones_only: bool,
    pub include_animations: bool,
    pub include_all_actions: bool,
    #[builder(setter(into))]
    pub export_animation_type_selection: String,
    pub sampling_rate: u32,
    pub keep_smooth_curves: bool,
    pub keep_keyframes: bool,
    pub keep_flat_curves: bool,
    pub active_uv_only: bool,
    pub use_texture_copies: bool,
    pub triangulate: bool,
    pub use_object_instantiation: bool,
    pub use_blender_profile: bool,
    pub sort_by_name: bool,
    pub export_object_transformation_type: u32,
    #[builder(setter(into))]
    pub export_object_transformation_type_selection: String,
    pub export_animation_transformation_type: u32,
    #[builder(setter(into))]
    pub export_animation_transformation_type_selection: String,
    pub open_sim: bool,
    pub limit_precision: bool,
    pub keep_bind_info: bool,
}

impl DaeExporterBuilder {
    #[must_use]
    pub fn build(&self) -> DaeExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for DaeExporter {
    fn default() -> Self {
        Self {
            prop_bc_export_ui_section: "main".to_string(),
            apply_modifiers: true,
            export_mesh_type: 0,
            export_mesh_type_selection: "view".to_string(),
            export_global_forward_selection: DirectionXYZ::PosY,
            export_global_up_selection: DirectionXYZ::PosZ,
            apply_global_orientation: false,
            selected: false,
            include_children: false,
            include_armatures: false,
            include_shapekeys: false,
            deform_bones_only: false,
            include_animations: true,
            include_all_actions: true,
            export_animation_type_selection: "sample".to_string(),
            sampling_rate: 1,
            keep_smooth_curves: false,
            keep_keyframes: false,
            keep_flat_curves: false,
            active_uv_only: false,
            use_texture_copies: true,
            triangulate: true,
            use_object_instantiation: true,
            use_blender_profile: true,
            sort_by_name: false,
            export_object_transformation_type: 0,
            export_object_transformation_type_selection: "matrix".to_string(),
            export_animation_transformation_type: 0,
            export_animation_transformation_type_selection: "matrix".to_string(),
            open_sim: false,
            limit_precision: false,
            keep_bind_info: false,
        }
    }
}

impl BlendExport for DaeExporter {
    const EXTENSIONS: &'static [&'static str] = &["dae"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_collada_export(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct FbxExporter {
    pub use_selection: bool,
    pub use_visible: bool,
    pub use_active_collection: bool,
    pub global_scale: f32,
    pub apply_unit_scale: bool,
    #[builder(setter(into))]
    pub apply_scale_options: String,
    pub use_space_transform: bool,
    pub bake_space_transform: bool,
    #[builder(setter(into))]
    pub object_types: std::collections::HashSet<String>,
    pub use_mesh_modifiers: bool,
    pub use_mesh_modifiers_render: bool,
    #[builder(setter(into))]
    pub mesh_smooth_type: String,
    #[builder(setter(into))]
    pub colors_type: String,
    pub prioritize_active_color: bool,
    pub use_subsurf: bool,
    pub use_mesh_edges: bool,
    pub use_tspace: bool,
    pub use_triangles: bool,
    pub use_custom_props: bool,
    pub add_leaf_bones: bool,
    pub primary_bone_axis: DirectionXYZ,
    pub secondary_bone_axis: DirectionXYZ,
    pub use_armature_deform_only: bool,
    #[builder(setter(into))]
    pub armature_nodetype: String,
    pub bake_anim: bool,
    pub bake_anim_use_all_bones: bool,
    pub bake_anim_use_nla_strips: bool,
    pub bake_anim_use_all_actions: bool,
    pub bake_anim_force_startend_keying: bool,
    pub bake_anim_step: f32,
    pub bake_anim_simplify_factor: f32,
    #[builder(setter(into))]
    pub path_mode: String,
    pub embed_textures: bool,
    #[builder(setter(into))]
    pub batch_mode: String,
    pub use_batch_own_dir: bool,
    pub use_metadata: bool,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl FbxExporterBuilder {
    #[must_use]
    pub fn build(&self) -> FbxExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for FbxExporter {
    fn default() -> Self {
        Self {
            use_selection: false,
            use_visible: false,
            use_active_collection: false,
            global_scale: 1.0,
            apply_unit_scale: true,
            apply_scale_options: "FBX_SCALE_NONE".to_string(),
            use_space_transform: true,
            bake_space_transform: false,
            object_types: std::collections::HashSet::from_iter([
                "ARMATURE".to_string(),
                "CAMERA".to_string(),
                "EMPTY".to_string(),
                "LIGHT".to_string(),
                "MESH".to_string(),
                "OTHER".to_string(),
            ]),
            use_mesh_modifiers: true,
            use_mesh_modifiers_render: true,
            mesh_smooth_type: "OFF".to_string(),
            colors_type: "SRGB".to_string(),
            prioritize_active_color: false,
            use_subsurf: false,
            use_mesh_edges: false,
            use_tspace: false,
            use_triangles: false,
            use_custom_props: false,
            add_leaf_bones: true,
            primary_bone_axis: DirectionXYZ::PosY,
            secondary_bone_axis: DirectionXYZ::PosX,
            use_armature_deform_only: false,
            armature_nodetype: "NULL".to_string(),
            bake_anim: true,
            bake_anim_use_all_bones: true,
            bake_anim_use_nla_strips: true,
            bake_anim_use_all_actions: true,
            bake_anim_force_startend_keying: true,
            bake_anim_step: 1.0,
            bake_anim_simplify_factor: 1.0,
            path_mode: "AUTO".to_string(),
            embed_textures: false,
            batch_mode: "OFF".to_string(),
            use_batch_own_dir: true,
            use_metadata: true,
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendExport for FbxExporter {
    const EXTENSIONS: &'static [&'static str] = &["fbx"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_scene::fn_fbx(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct GltfExporter {
    #[builder(setter(into))]
    pub export_import_convert_lighting_mode: String,
    #[builder(setter(into))]
    pub gltf_export_id: String,
    #[builder(setter(into))]
    pub export_format: String,
    #[builder(setter(into))]
    pub ui_tab: String,
    #[builder(setter(into))]
    pub export_copyright: String,
    #[builder(setter(into))]
    pub export_image_format: String,
    #[builder(setter(into))]
    pub export_texture_dir: String,
    pub export_jpeg_quality: u32,
    pub export_keep_originals: bool,
    pub export_texcoords: bool,
    pub export_normals: bool,
    pub export_draco_mesh_compression_enable: bool,
    pub export_draco_mesh_compression_level: u32,
    pub export_draco_position_quantization: u32,
    pub export_draco_normal_quantization: u32,
    pub export_draco_texcoord_quantization: u32,
    pub export_draco_color_quantization: u32,
    pub export_draco_generic_quantization: u32,
    pub export_tangents: bool,
    #[builder(setter(into))]
    pub export_materials: String,
    pub export_original_specular: bool,
    pub export_colors: bool,
    pub export_attributes: bool,
    pub use_mesh_edges: bool,
    pub use_mesh_vertices: bool,
    pub export_cameras: bool,
    pub use_selection: bool,
    pub use_visible: bool,
    pub use_renderable: bool,
    pub use_active_collection_with_nested: bool,
    pub use_active_collection: bool,
    pub use_active_scene: bool,
    pub export_extras: bool,
    pub export_yup: bool,
    pub export_apply: bool,
    pub export_animations: bool,
    pub export_frame_range: bool,
    pub export_frame_step: u32,
    pub export_force_sampling: bool,
    #[builder(setter(into))]
    pub export_animation_mode: String,
    #[builder(setter(into))]
    pub export_nla_strips_merged_animation_name: String,
    pub export_def_bones: bool,
    pub export_hierarchy_flatten_bones: bool,
    pub export_optimize_animation_size: bool,
    pub export_optimize_animation_keep_anim_armature: bool,
    pub export_optimize_animation_keep_anim_object: bool,
    #[builder(setter(into))]
    pub export_negative_frame: String,
    pub export_anim_slide_to_zero: bool,
    pub export_bake_animation: bool,
    pub export_anim_single_armature: bool,
    pub export_reset_pose_bones: bool,
    pub export_current_frame: bool,
    pub export_rest_position_armature: bool,
    pub export_anim_scene_split_object: bool,
    pub export_skins: bool,
    pub export_all_influences: bool,
    pub export_morph: bool,
    pub export_morph_normal: bool,
    pub export_morph_tangent: bool,
    pub export_morph_animation: bool,
    pub export_morph_reset_sk_data: bool,
    pub export_lights: bool,
    pub export_nla_strips: bool,
    pub will_save_settings: bool,
}

impl GltfExporterBuilder {
    #[must_use]
    pub fn build(&self) -> GltfExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for GltfExporter {
    fn default() -> Self {
        Self {
            export_import_convert_lighting_mode: "SPEC".to_string(),
            gltf_export_id: String::new(),
            export_format: "GLB".to_string(),
            ui_tab: "GENERAL".to_string(),
            export_copyright: String::new(),
            export_image_format: "AUTO".to_string(),
            export_texture_dir: String::new(),
            export_jpeg_quality: 75,
            export_keep_originals: false,
            export_texcoords: true,
            export_normals: true,
            export_draco_mesh_compression_enable: false,
            export_draco_mesh_compression_level: 6,
            export_draco_position_quantization: 14,
            export_draco_normal_quantization: 10,
            export_draco_texcoord_quantization: 12,
            export_draco_color_quantization: 10,
            export_draco_generic_quantization: 12,
            export_tangents: false,
            export_materials: "EXPORT".to_string(),
            export_original_specular: false,
            export_colors: true,
            export_attributes: false,
            use_mesh_edges: false,
            use_mesh_vertices: false,
            export_cameras: false,
            use_selection: false,
            use_visible: false,
            use_renderable: false,
            use_active_collection_with_nested: true,
            use_active_collection: false,
            use_active_scene: false,
            export_extras: false,
            export_yup: true,
            export_apply: true,
            export_animations: true,
            export_frame_range: false,
            export_frame_step: 1,
            export_force_sampling: true,
            export_animation_mode: "ACTIONS".to_string(),
            export_nla_strips_merged_animation_name: "Animation".to_string(),
            export_def_bones: false,
            export_hierarchy_flatten_bones: false,
            export_optimize_animation_size: true,
            export_optimize_animation_keep_anim_armature: true,
            export_optimize_animation_keep_anim_object: false,
            export_negative_frame: "SLIDE".to_string(),
            export_anim_slide_to_zero: false,
            export_bake_animation: false,
            export_anim_single_armature: true,
            export_reset_pose_bones: true,
            export_current_frame: false,
            export_rest_position_armature: true,
            export_anim_scene_split_object: true,
            export_skins: true,
            export_all_influences: false,
            export_morph: true,
            export_morph_normal: true,
            export_morph_tangent: false,
            export_morph_animation: true,
            export_morph_reset_sk_data: true,
            export_lights: false,
            export_nla_strips: true,
            will_save_settings: false,
        }
    }
}

impl BlendExport for GltfExporter {
    const EXTENSIONS: &'static [&'static str] = &["gltf", "glb"];

    fn extension(&self) -> &str {
        match self.export_format.to_uppercase().as_str() {
            "GLB" => "glb",
            _ => "gltf",
        }
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_scene::fn_gltf(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct ObjExporter {
    pub use_selection: bool,
    pub use_animation: bool,
    pub use_mesh_modifiers: bool,
    pub use_edges: bool,
    pub use_smooth_groups: bool,
    pub use_smooth_groups_bitflags: bool,
    pub use_normals: bool,
    pub use_uvs: bool,
    pub use_materials: bool,
    pub use_triangles: bool,
    pub use_nurbs: bool,
    pub use_vertex_groups: bool,
    pub use_blen_objects: bool,
    pub group_by_object: bool,
    pub group_by_material: bool,
    pub keep_vertex_order: bool,
    pub global_scale: f32,
    #[builder(setter(into))]
    pub path_mode: String,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl ObjExporterBuilder {
    #[must_use]
    pub fn build(&self) -> ObjExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for ObjExporter {
    fn default() -> Self {
        Self {
            use_selection: false,
            use_animation: false,
            use_mesh_modifiers: true,
            use_edges: true,
            use_smooth_groups: false,
            use_smooth_groups_bitflags: false,
            use_normals: true,
            use_uvs: true,
            use_materials: true,
            use_triangles: false,
            use_nurbs: false,
            use_vertex_groups: false,
            use_blen_objects: true,
            group_by_object: false,
            group_by_material: false,
            keep_vertex_order: false,
            global_scale: 1.0,
            path_mode: "AUTO".to_string(),
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendExport for ObjExporter {
    const EXTENSIONS: &'static [&'static str] = &["obj"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_scene::fn_obj(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_mesh.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct PlyExporter {
    pub use_ascii: bool,
    pub use_selection: bool,
    pub use_mesh_modifiers: bool,
    pub use_normals: bool,
    pub use_uv_coords: bool,
    pub use_colors: bool,
    pub global_scale: f32,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl PlyExporterBuilder {
    #[must_use]
    pub fn build(&self) -> PlyExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for PlyExporter {
    fn default() -> Self {
        Self {
            use_ascii: false,
            use_selection: false,
            use_mesh_modifiers: true,
            use_normals: true,
            use_uv_coords: true,
            use_colors: true,
            global_scale: 1.0,
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendExport for PlyExporter {
    const EXTENSIONS: &'static [&'static str] = &["ply"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_mesh::fn_ply(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_mesh.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct StlExporter {
    pub use_selection: bool,
    pub global_scale: f32,
    pub use_scene_unit: bool,
    pub ascii: bool,
    pub use_mesh_modifiers: bool,
    #[builder(setter(into))]
    pub batch_mode: String,
    pub global_space: [[f32; 4]; 4],
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl StlExporterBuilder {
    #[must_use]
    pub fn build(&self) -> StlExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for StlExporter {
    fn default() -> Self {
        Self {
            use_selection: false,
            global_scale: 1.0,
            use_scene_unit: false,
            ascii: false,
            use_mesh_modifiers: true,
            batch_mode: "OFF".to_string(),
            global_space: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendExport for StlExporter {
    const EXTENSIONS: &'static [&'static str] = &["stl"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_mesh::fn_stl(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct UsdExporter {
    pub selected_objects_only: bool,
    pub visible_objects_only: bool,
    pub export_animation: bool,
    pub export_hair: bool,
    pub export_uvmaps: bool,
    pub export_normals: bool,
    pub export_materials: bool,
    pub use_instancing: bool,
    #[builder(setter(into))]
    pub evaluation_mode: String,
    pub generate_preview_surface: bool,
    pub export_textures: bool,
    pub overwrite_textures: bool,
    #[builder(setter(into))]
    pub root_prim_path: String,
}

impl UsdExporterBuilder {
    #[must_use]
    pub fn build(&self) -> UsdExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for UsdExporter {
    fn default() -> Self {
        Self {
            selected_objects_only: false,
            visible_objects_only: true,
            export_animation: false,
            export_hair: false,
            export_uvmaps: true,
            export_normals: true,
            export_materials: true,
            use_instancing: false,
            evaluation_mode: "RENDER".to_string(),
            generate_preview_surface: true,
            export_textures: true,
            overwrite_textures: false,
            root_prim_path: String::new(),
        }
    }
}

impl BlendExport for UsdExporter {
    const EXTENSIONS: &'static [&'static str] = &["usd", "usda", "usdc", "usdz"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::wm::fn_usd_export(py)?)
    }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_scene.html>
#[derive(Debug, Clone, Builder, IntoBuilder, pyo3_derive_more::IntoPyDict)]
#[builder(default, build_fn(private, name = "fallible_build", error = "BlError"))]
pub struct X3dExporter {
    pub use_selection: bool,
    pub use_mesh_modifiers: bool,
    pub use_triangulate: bool,
    pub use_normals: bool,
    pub use_compress: bool,
    pub use_hierarchy: bool,
    pub name_decorations: bool,
    pub use_h3d: bool,
    pub global_scale: f32,
    #[builder(setter(into))]
    pub path_mode: String,
    pub axis_forward: DirectionXYZ,
    pub axis_up: DirectionXYZ,
}

impl X3dExporterBuilder {
    #[must_use]
    pub fn build(&self) -> X3dExporter {
        self.fallible_build().unwrap_or_else(|_| unreachable!())
    }
}

impl Default for X3dExporter {
    fn default() -> Self {
        Self {
            use_selection: false,
            use_mesh_modifiers: true,
            use_triangulate: false,
            use_normals: false,
            use_compress: false,
            use_hierarchy: true,
            name_decorations: true,
            use_h3d: false,
            global_scale: 1.0,
            path_mode: "AUTO".to_string(),
            axis_forward: DirectionXYZ::PosY,
            axis_up: DirectionXYZ::PosZ,
        }
    }
}

impl BlendExport for X3dExporter {
    const EXTENSIONS: &'static [&'static str] = &["x3d"];

    fn extension(&self) -> &str {
        Self::EXTENSIONS[0]
    }

    fn fn_export(py: Python) -> Result<&PyAny> {
        Ok(bpy::ops::export_scene::fn_x3d(py)?)
    }
}
