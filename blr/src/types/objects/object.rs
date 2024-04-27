use crate::{
    bpy,
    enums::{
        Alignment, ContextMode, LightType, ObjectMode, ObjectType, OriginCenter, OriginType,
        RenderVariant,
    },
    result::Result,
    scene::{Scene, SpaceView3D, ViewLayer},
    types::{
        collections::ObjectModifiers, BpyID, Collection, CollectionImpl, Curve, Empty, Material,
        Mesh,
    },
};
use derive_more::{Deref, DerefMut, Display};
// TODO: Remove nalgebra dependency
use nalgebra::Quaternion;
use pyo3::{intern, PyAny, PyObject, Python};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Object.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Object(PyObject);

impl BpyID for Object {}

impl Object {
    pub fn new_camera(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::camera_add(
            py,
            false,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn new_empty(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Empty> {
        Self::force_object_mode(py)?;
        bpy::ops::object::empty_add(
            py,
            "PLAIN_AXES",
            1.0,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(Empty(bpy::context::active_object(py)?))
    }

    pub fn new_light(
        py: Python,
        r#type: LightType,
        location: [f32; 3],
        rotation: [f32; 3],
    ) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::light_add(
            py,
            r#type,
            1.0,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn new_mesh(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::add(
            py,
            1.0,
            ObjectType::Mesh,
            false,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn new_pointcloud(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::pointcloud_add(
            py,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn new_text(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::text_add(
            py,
            1.0,
            false,
            Alignment::World,
            location,
            rotation,
            [1.0, 1.0, 1.0],
        )?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn new_volume(py: Python, location: [f32; 3], rotation: [f32; 3]) -> Result<Self> {
        Self::force_object_mode(py)?;
        bpy::ops::object::volume_add(py, Alignment::World, location, rotation, [1.0, 1.0, 1.0])?;
        Ok(bpy::context::active_object(py)?)
    }

    pub fn from_name(name: &str) -> Result<Self> {
        Ok(Python::with_gil(|py| {
            bpy::data::objects(py)?.get(py, name)
        })?)
    }

    pub fn from_active(py: Python) -> Result<Self> {
        Ok(bpy::context::active_object(py)?)
    }

    pub fn set_active(&self, py: Python) -> Result<()> {
        bpy::context::view_layer(py)?.objects(py)?.setattr(
            py,
            intern!(py, "active"),
            self.as_ref(py),
        )?;
        Ok(())
    }

    pub fn delete(self, py: Python) -> Result<()> {
        let objects = bpy::data::objects(py)?;
        Ok(objects.remove(py, &self, true, true, true)?)
    }

    pub fn object_type(&self, py: Python) -> Result<ObjectType> {
        Ok(self.getattr(py, intern!(py, "type"))?.extract(py)?)
    }

    pub fn translate(&mut self, py: Python, translation: [f32; 3]) -> Result<()> {
        let mut location = self.location(py)?;
        location
            .iter_mut()
            .zip(translation)
            .for_each(|(location, translation)| *location += translation);
        Ok(self.set_location(py, location)?)
    }

    pub fn rotate_euler(&mut self, py: Python, rotation: [f32; 3]) -> Result<()> {
        let mut orientation = self.rotation_euler(py)?;
        orientation
            .iter_mut()
            .zip(rotation)
            .for_each(|(orientation, rotation)| *orientation += rotation);
        Ok(self.set_rotation_euler(py, orientation)?)
    }

    pub fn rotate_quaternion(&mut self, py: Python, rotation: Quaternion<f32>) -> Result<()> {
        let orientation = self.rotation_quaternion(py)?;
        let mut orientation = Quaternion::new(
            orientation[0],
            orientation[1],
            orientation[2],
            orientation[3],
        );
        orientation *= rotation;
        Ok(self.set_rotation_quaternion(
            py,
            [orientation.w, orientation.i, orientation.j, orientation.k],
        )?)
    }

    pub fn resize(&mut self, py: Python, size: [f32; 3]) -> Result<()> {
        let mut scale = self.scale(py)?;
        scale
            .iter_mut()
            .zip(size)
            .for_each(|(scale, size)| *scale *= size);
        Ok(self.set_scale(py, scale)?)
    }

    pub fn set_mode(&self, py: Python, mode: ObjectMode) -> Result<()> {
        self.set_active(py)?;
        Ok(bpy::ops::object::mode_set(py, mode, false)?)
    }

    pub fn set_origin(&self, py: Python, r#type: OriginType, center: OriginCenter) -> Result<()> {
        self.set_active(py)?;
        Ok(bpy::ops::object::origin_set(py, r#type, center)?)
    }

    pub(crate) fn force_object_mode(py: Python) -> Result<()> {
        let obj = Self::from_active(py)?;
        if !obj.is_none(py) {
            obj.set_mode(py, ObjectMode::Object)?;
        }
        Ok(())
    }

    bind_python! { self.active_material => pub fn active_material(&self, py: Python) -> Result<Material> }
    bind_python! { self.active_material = pub fn set_active_material(&mut self, py: Python, value: &Material) }
    bind_python! { self.active_material_index => pub fn active_material_index(&self, py: Python) -> Result<usize> }
    bind_python! { self.active_material_index = pub fn set_active_material_index(&mut self, py: Python, value: usize) }
    bind_python! { self.active_shape_key => pub fn active_shape_key<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.active_shape_key_index => pub fn active_shape_key_index(&self, py: Python) -> Result<i16> }
    bind_python! { self.active_shape_key_index = pub fn set_active_shape_key_index(&mut self, py: Python, value: i16) }
    bind_python! { self.add_rest_position_attribute => pub fn add_rest_position_attribute(&self, py: Python) -> Result<bool> }
    bind_python! { self.add_rest_position_attribute = pub fn set_add_rest_position_attribute(&mut self, py: Python, value: bool) }
    bind_python! { self.animation_data => pub fn animation_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.animation_visualization => pub fn animation_visualization<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.bound_box => pub fn bound_box<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.collision => pub fn collision<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.color => pub fn color(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.color = pub fn set_color(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.constraints => pub fn constraints<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.cycles => pub fn cycles<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.data => pub fn data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.data = pub fn set_data(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.delta_location => pub fn delta_location(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.delta_location = pub fn set_delta_location(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.delta_rotation_euler => pub fn delta_rotation_euler(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.delta_rotation_euler = pub fn set_delta_rotation_euler(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.delta_rotation_quaternion => pub fn delta_rotation_quaternion(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.delta_rotation_quaternion = pub fn set_delta_rotation_quaternion(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.delta_scale => pub fn delta_scale(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.delta_scale = pub fn set_delta_scale(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.dimensions => pub fn dimensions(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.dimensions = pub fn set_dimensions(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.display => pub fn display<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.display_bounds_type => pub fn display_bounds_type(&self, py: Python) -> Result<String> }
    bind_python! { self.display_bounds_type = pub fn set_display_bounds_type(&mut self, py: Python, value: &str) }
    bind_python! { self.display_type => pub fn display_type(&self, py: Python) -> Result<String> }
    bind_python! { self.display_type = pub fn set_display_type(&mut self, py: Python, value: &str) }
    bind_python! { self.empty_display_size => pub fn empty_display_size(&self, py: Python) -> Result<f32> }
    bind_python! { self.empty_display_size = pub fn set_empty_display_size(&mut self, py: Python, value: f32) }
    bind_python! { self.empty_display_type => pub fn empty_display_type<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.empty_display_type = pub fn set_empty_display_type(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.empty_image_depth => pub fn empty_image_depth(&self, py: Python) -> Result<String> }
    bind_python! { self.empty_image_depth = pub fn set_empty_image_depth(&mut self, py: Python, value: &str) }
    bind_python! { self.empty_image_offset => pub fn empty_image_offset<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.empty_image_offset = pub fn set_empty_image_offset(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.empty_image_side => pub fn empty_image_side(&self, py: Python) -> Result<String> }
    bind_python! { self.empty_image_side = pub fn set_empty_image_side(&mut self, py: Python, value: &str) }
    bind_python! { self.face_maps => pub fn face_maps<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.field => pub fn field<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.grease_pencil_modifiers => pub fn grease_pencil_modifiers<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.hide_render => pub fn hide_render(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_render = pub fn set_hide_render(&mut self, py: Python, value: bool) }
    bind_python! { self.hide_select => pub fn hide_select(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_select = pub fn set_hide_select(&mut self, py: Python, value: bool) }
    bind_python! { self.hide_viewport => pub fn hide_viewport(&self, py: Python) -> Result<bool> }
    bind_python! { self.hide_viewport = pub fn set_hide_viewport(&mut self, py: Python, value: bool) }
    bind_python! { self.image_user => pub fn image_user<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.instance_collection => pub fn instance_collection<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.instance_collection = pub fn set_instance_collection(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.instance_faces_scale => pub fn instance_faces_scale(&self, py: Python) -> Result<f32> }
    bind_python! { self.instance_faces_scale = pub fn set_instance_faces_scale(&mut self, py: Python, value: f32) }
    bind_python! { self.instance_type => pub fn instance_type(&self, py: Python) -> Result<String> }
    bind_python! { self.instance_type = pub fn set_instance_type(&mut self, py: Python, value: &str) }
    bind_python! { self.is_from_instancer => pub fn is_from_instancer(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_from_set => pub fn is_from_set(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_holdout => pub fn is_holdout(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_holdout = pub fn set_is_holdout(&mut self, py: Python, value: bool) }
    bind_python! { self.is_instancer => pub fn is_instancer(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_shadow_catcher => pub fn is_shadow_catcher(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_shadow_catcher = pub fn set_is_shadow_catcher(&mut self, py: Python, value: bool) }
    bind_python! { self.lightgroup => pub fn lightgroup(&self, py: Python) -> Result<String> }
    bind_python! { self.lightgroup = pub fn set_lightgroup(&mut self, py: Python, value: &str) }
    bind_python! { self.lineart => pub fn lineart<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.location => pub fn location(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.location = pub fn set_location(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.lock_location => pub fn lock_location<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lock_location = pub fn set_lock_location(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.lock_rotation => pub fn lock_rotation<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lock_rotation = pub fn set_lock_rotation(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.lock_rotation_w => pub fn lock_rotation_w<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lock_rotation_w = pub fn set_lock_rotation_w(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.lock_rotations_4d => pub fn lock_rotations_4d<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lock_rotations_4d = pub fn set_lock_rotations_4d(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.lock_scale => pub fn lock_scale<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.lock_scale = pub fn set_lock_scale(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.material_slots => pub fn material_slots<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.matrix_basis => pub fn matrix_basis<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.matrix_basis = pub fn set_matrix_basis(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.matrix_local => pub fn matrix_local<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.matrix_local = pub fn set_matrix_local(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.matrix_parent_inverse => pub fn matrix_parent_inverse<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.matrix_parent_inverse = pub fn set_matrix_parent_inverse(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.matrix_world => pub fn matrix_world<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.matrix_world = pub fn set_matrix_world(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.mode => pub fn mode(&self, py: Python) -> Result<ContextMode> }
    bind_python! { self.modifiers => pub fn modifiers(&self, py: Python) -> Result<ObjectModifiers> }
    bind_python! { self.motion_path => pub fn motion_path<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.parent => pub fn parent(&self, py: Python) -> Result<Self> }
    bind_python! { self.parent = pub fn set_parent(&mut self, py: Python, value: &Self) }
    bind_python! { self.parent_bone => pub fn parent_bone(&self, py: Python) -> Result<String> }
    bind_python! { self.parent_bone = pub fn set_parent_bone(&mut self, py: Python, value: &str) }
    bind_python! { self.parent_type => pub fn parent_type(&self, py: Python) -> Result<ObjectType> }
    // bind_python! { self.parent_type = pub fn set_parent_type(&mut self, py: Python, value: ObjectType) }
    bind_python! { self.parent_vertices => pub fn parent_vertices<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.parent_vertices = pub fn set_parent_vertices(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.particle_systems => pub fn particle_systems<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.pass_index => pub fn pass_index(&self, py: Python) -> Result<i16> }
    bind_python! { self.pass_index = pub fn set_pass_index(&mut self, py: Python, value: i16) }
    bind_python! { self.pose => pub fn pose<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.rigid_body => pub fn rigid_body<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.rigid_body_constraint => pub fn rigid_body_constraint<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.rotation_axis_angle => pub fn rotation_axis_angle(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.rotation_axis_angle = pub fn set_rotation_axis_angle(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.rotation_euler => pub fn rotation_euler(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.rotation_euler = pub fn set_rotation_euler(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.rotation_mode => pub fn rotation_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.rotation_mode = pub fn set_rotation_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.rotation_quaternion => pub fn rotation_quaternion(&self, py: Python) -> Result<[f32; 4]> }
    bind_python! { self.rotation_quaternion = pub fn set_rotation_quaternion(&mut self, py: Python, value: [f32; 4]) }
    bind_python! { self.scale => pub fn scale(&self, py: Python) -> Result<[f32; 3]> }
    bind_python! { self.scale = pub fn set_scale(&mut self, py: Python, value: [f32; 3]) }
    bind_python! { self.shader_effects => pub fn shader_effects<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.shader_effects = pub fn set_shader_effects(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.show_all_edges => pub fn show_all_edges(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_all_edges = pub fn set_show_all_edges(&mut self, py: Python, value: bool) }
    bind_python! { self.show_axis => pub fn show_axis(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_axis = pub fn set_show_axis(&mut self, py: Python, value: bool) }
    bind_python! { self.show_bounds => pub fn show_bounds(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_bounds = pub fn set_show_bounds(&mut self, py: Python, value: bool) }
    bind_python! { self.show_empty_image_only_axis_aligned => pub fn show_empty_image_only_axis_aligned(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_empty_image_only_axis_aligned = pub fn set_show_empty_image_only_axis_aligned(&mut self, py: Python, value: bool) }
    bind_python! { self.show_empty_image_orthographic => pub fn show_empty_image_orthographic(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_empty_image_orthographic = pub fn set_show_empty_image_orthographic(&mut self, py: Python, value: bool) }
    bind_python! { self.show_empty_image_perspective => pub fn show_empty_image_perspective(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_empty_image_perspective = pub fn set_show_empty_image_perspective(&mut self, py: Python, value: bool) }
    bind_python! { self.show_in_front => pub fn show_in_front(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_in_front = pub fn set_show_in_front(&mut self, py: Python, value: bool) }
    bind_python! { self.show_instancer_for_render => pub fn show_instancer_for_render(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_instancer_for_render = pub fn set_show_instancer_for_render(&mut self, py: Python, value: bool) }
    bind_python! { self.show_instancer_for_viewport => pub fn show_instancer_for_viewport(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_instancer_for_viewport = pub fn set_show_instancer_for_viewport(&mut self, py: Python, value: bool) }
    bind_python! { self.show_name => pub fn show_name(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_name = pub fn set_show_name(&mut self, py: Python, value: bool) }
    bind_python! { self.show_only_shape_key => pub fn show_only_shape_key(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_only_shape_key = pub fn set_show_only_shape_key(&mut self, py: Python, value: bool) }
    bind_python! { self.show_texture_space => pub fn show_texture_space(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_texture_space = pub fn set_show_texture_space(&mut self, py: Python, value: bool) }
    bind_python! { self.show_transparent => pub fn show_transparent(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_transparent = pub fn set_show_transparent(&mut self, py: Python, value: bool) }
    bind_python! { self.show_wire => pub fn show_wire(&self, py: Python) -> Result<bool> }
    bind_python! { self.show_wire = pub fn set_show_wire(&mut self, py: Python, value: bool) }
    bind_python! { self.soft_body => pub fn soft_body<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.track_axis => pub fn track_axis<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.up_axis => pub fn up_axis<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.use_camera_lock_parent => pub fn use_camera_lock_parent(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_camera_lock_parent = pub fn set_use_camera_lock_parent(&mut self, py: Python, value: bool) }
    bind_python! { self.use_dynamic_topology_sculpting => pub fn use_dynamic_topology_sculpting(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_empty_image_alpha => pub fn use_empty_image_alpha(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_empty_image_alpha = pub fn set_use_empty_image_alpha(&mut self, py: Python, value: bool) }
    bind_python! { self.use_grease_pencil_lights => pub fn use_grease_pencil_lights(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_grease_pencil_lights = pub fn set_use_grease_pencil_lights(&mut self, py: Python, value: bool) }
    bind_python! { self.use_instance_faces_scale => pub fn use_instance_faces_scale(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_instance_faces_scale = pub fn set_use_instance_faces_scale(&mut self, py: Python, value: bool) }
    bind_python! { self.use_instance_vertices_rotation => pub fn use_instance_vertices_rotation(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_instance_vertices_rotation = pub fn set_use_instance_vertices_rotation(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mesh_mirror_x => pub fn use_mesh_mirror_x(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mesh_mirror_x = pub fn set_use_mesh_mirror_x(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mesh_mirror_y => pub fn use_mesh_mirror_y(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mesh_mirror_y = pub fn set_use_mesh_mirror_y(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mesh_mirror_z => pub fn use_mesh_mirror_z(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mesh_mirror_z = pub fn set_use_mesh_mirror_z(&mut self, py: Python, value: bool) }
    bind_python! { self.use_shape_key_edit_mode => pub fn use_shape_key_edit_mode(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_shape_key_edit_mode = pub fn set_use_shape_key_edit_mode(&mut self, py: Python, value: bool) }
    bind_python! { self.use_simulation_cache => pub fn use_simulation_cache(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_simulation_cache = pub fn set_use_simulation_cache(&mut self, py: Python, value: bool) }
    bind_python! { self.vertex_groups => pub fn vertex_groups<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.visible_camera => pub fn visible_camera(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_camera = pub fn set_visible_camera(&mut self, py: Python, value: bool) }
    bind_python! { self.visible_diffuse => pub fn visible_diffuse(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_diffuse = pub fn set_visible_diffuse(&mut self, py: Python, value: bool) }
    bind_python! { self.visible_glossy => pub fn visible_glossy(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_glossy = pub fn set_visible_glossy(&mut self, py: Python, value: bool) }
    bind_python! { self.visible_shadow => pub fn visible_shadow(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_shadow = pub fn set_visible_shadow(&mut self, py: Python, value: bool) }
    bind_python! { self.visible_transmission => pub fn visible_transmission(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_transmission = pub fn set_visible_transmission(&mut self, py: Python, value: bool) }
    bind_python! { self.visible_volume_scatter => pub fn visible_volume_scatter(&self, py: Python) -> Result<bool> }
    bind_python! { self.visible_volume_scatter = pub fn set_visible_volume_scatter(&mut self, py: Python, value: bool) }
    bind_python! { self.children => pub fn children(&self, py: Python) -> Result<Vec<Self>> }
    bind_python! { self.children_recursive => pub fn children_recursive(&self, py: Python) -> Result<Vec<Self>> }
    bind_python! { self.users_collection => pub fn users_collection(&self, py: Python) -> Result<Vec<Collection>> }
    bind_python! { self.users_scene => pub fn users_scene<'py>(&'py self, py: Python<'py>) -> Result<Vec<Scene<'py>>> }
    bind_python! { self.select_get() => pub fn select_get(&self, py: Python, view_layer: Option<ViewLayer>) -> Result<bool> }
    bind_python! { self.select_set() => pub fn select_set(&self, py: Python, state: bool, view_layer: Option<ViewLayer>) }
    bind_python! { self.hide_get() => pub fn hide_get(&self, py: Python, view_layer: Option<ViewLayer>) -> Result<bool> }
    bind_python! { self.hide_set() => pub fn hide_set(&self, py: Python, state: bool, view_layer: Option<ViewLayer>) }
    bind_python! { self.visible_get() => pub fn visible_get(&self, py: Python, view_layer: Option<ViewLayer>, viewport: Option<SpaceView3D>) -> Result<bool> }
    bind_python! { self.holdout_get() => pub fn holdout_get(&self, py: Python, view_layer: Option<ViewLayer>) -> Result<bool> }
    bind_python! { self.indirect_only_get() => pub fn indirect_only_get(&self, py: Python, view_layer: Option<ViewLayer>) -> Result<bool> }
    bind_python! { self.local_view_get() => pub fn local_view_get(&self, py: Python, viewport: SpaceView3D) -> Result<bool> }
    bind_python! { self.local_view_set() => pub fn local_view_set(&self, py: Python, viewport: SpaceView3D, state: bool) }
    bind_python! { self.visible_in_viewport_get() => pub fn visible_in_viewport_get(&self, py: Python, viewport: SpaceView3D) -> Result<bool> }
    // bind_python! { self.convert_space() => pub fn convert_space(&self, py: Python, pose_bone=None, matrix=((0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0), (0.0, 0.0, 0.0, 0.0)), from_space='WORLD', to_space='WORLD') -> Result<&PyAny> }
    // bind_python! { self.calc_matrix_camera() => pub fn calc_matrix_camera(&self, py: Python, x=1, y=1, scale_x=1.0, scale_y=1.0) -> Result<&PyAny> }
    // bind_python! { self.camera_fit_coords() => pub fn camera_fit_coords(&self, py: Python, coordinates) -> Result<&PyAny> }
    // bind_python! { self.crazyspace_eval() => pub fn crazyspace_eval(&self, py: Python, scene) -> Result<&PyAny> }
    // bind_python! { self.crazyspace_displacement_to_deformed() => pub fn crazyspace_displacement_to_deformed(&self, py: Python, vertex_index=0, displacement=(0.0, 0.0, 0.0)) -> Result<&PyAny> }
    // bind_python! { self.crazyspace_displacement_to_original() => pub fn crazyspace_displacement_to_original(&self, py: Python, vertex_index=0, displacement=(0.0, 0.0, 0.0)) -> Result<&PyAny> }
    // bind_python! { self.crazyspace_eval_clear() => pub fn crazyspace_eval_clear(&self, py: Python) -> Result<&PyAny> }
    bind_python! { self.to_mesh() => pub fn to_mesh(&self, py: Python) -> Result<Mesh> }
    bind_python! { self.to_mesh_clear() => pub fn to_mesh_clear(&self, py: Python) }
    bind_python! { self.to_curve() => pub fn to_curve(&self, py: Python, apply_modifiers: bool) -> Result<Curve> }
    bind_python! { self.to_curve_clear() => pub fn to_curve_clear(&self, py: Python) }
    bind_python! { self.find_armature() => pub fn find_armature(&self, py: Python) -> Result<Self> }
    // bind_python! { self.shape_key_add() => pub fn shape_key_add(&self, py: Python, name='Key', from_mix: bool) -> Result<&PyAny> }
    // bind_python! { self.shape_key_remove() => pub fn shape_key_remove(&self, py: Python, key) }
    // bind_python! { self.shape_key_clear() => pub fn shape_key_clear(&self, py: Python) }
    bind_python! { self.ray_cast() => pub fn ray_cast(&self, py: Python, origin: [f32; 3], direction: [f32; 3], distance: f32) -> Result<(bool, [f32; 3], [f32; 3], i32)> }
    bind_python! { self.closest_point_on_mesh() => pub fn closest_point_on_mesh(&self, py: Python, origin: [f32; 3], distance: f32) -> Result<(bool, [f32; 3], [f32; 3], i32)> }
    bind_python! { self.is_modified() => pub fn is_modified(&self, py: Python, scene: Scene, settings: RenderVariant) -> Result<bool> }
    bind_python! { self.is_deform_modified() => pub fn is_deform_modified(&self, py: Python, scene: Scene, settings: RenderVariant) -> Result<bool> }
    bind_python! { self.update_from_editmode() => pub fn update_from_editmode(&self, py: Python) -> Result<bool> }
    bind_python! { self.cache_release() => pub fn cache_release(&self, py: Python) }
    bind_python! { self.generate_gpencil_strokes() => pub fn generate_gpencil_strokes(&self, py: Python, grease_pencil_object: &Self, use_collections: bool, scale_thickness: f32, sample: f32) -> Result<bool> }
}

impl From<pyo3::PyObject> for Object {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Object {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Object {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Object {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
