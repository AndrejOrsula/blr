//! Bindings for [`bpy.context`](https://docs.blender.org/api/latest/bpy.context.html).
use super::{bind_python, Collection, ContextMode, Object, PyAny, ViewLayer};

bind_python! { bpy.context.temp_override => pub fn fn_temp_override(py: Python) -> Result<&PyAny> }

// Global context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.area => pub fn area(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.asset_file_handle => pub fn asset_file_handle(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.blend_data => pub fn blend_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.collection => pub fn collection(py: Python) -> Result<Collection> }
bind_python! { bpy.context.engine => pub fn engine(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.gizmo_group => pub fn gizmo_group(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.layer_collection => pub fn layer_collection(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.mode => pub fn mode(py: Python) -> Result<ContextMode> }
bind_python! { bpy.context.preferences => pub fn preferences(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.region => pub fn region(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.region_data => pub fn region_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.scene => pub fn scene(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.screen => pub fn screen(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.space_data => pub fn space_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.tool_settings => pub fn tool_settings(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.view_layer => pub fn view_layer(py: Python) -> Result<ViewLayer> }
bind_python! { bpy.context.window => pub fn window(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.window_manager => pub fn window_manager(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.workspace => pub fn workspace(py: Python) -> Result<&PyAny> }

// Buttons context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.texture_slot => pub fn texture_slot(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.mesh => pub fn mesh(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.armature => pub fn armature(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.lattice => pub fn lattice(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.curve => pub fn curve(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.meta_ball => pub fn meta_ball(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.speaker => pub fn speaker(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.lightprobe => pub fn lightprobe(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.camera => pub fn camera(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.material_slot => pub fn material_slot(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.texture => pub fn texture(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.texture_user => pub fn texture_user(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.texture_user_property => pub fn texture_user_property(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.bone => pub fn bone(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.edit_bone => pub fn edit_bone(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.pose_bone => pub fn pose_bone(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.particle_system => pub fn particle_system(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.particle_system_editable => pub fn particle_system_editable(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.particle_settings => pub fn particle_settings(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.cloth => pub fn cloth(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.soft_body => pub fn soft_body(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.fluid => pub fn fluid(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.collision => pub fn collision(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.brush => pub fn brush(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.dynamic_paint => pub fn dynamic_paint(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.line_style => pub fn line_style(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.gpencil => pub fn gpencil(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.curves => pub fn curves(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.volume => pub fn volume(py: Python) -> Result<&PyAny> }

// Clip context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.edit_movieclip => pub fn edit_movieclip(py: Python) -> Result<&PyAny> }

// File context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.active_file => pub fn active_file(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_files => pub fn selected_files(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.asset_library_ref => pub fn asset_library_ref(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_asset_files => pub fn selected_asset_files(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.id => pub fn id(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_ids => pub fn selected_ids(py: Python) -> Result<Vec<&PyAny>> }

// Image context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.edit_image => pub fn edit_image(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.edit_mask => pub fn edit_mask(py: Python) -> Result<&PyAny> }

// Node context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.selected_nodes => pub fn selected_nodes(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_node => pub fn active_node(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.light => pub fn light(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.material => pub fn material(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.world => pub fn world(py: Python) -> Result<&PyAny> }

// Screen context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.visible_objects => pub fn visible_objects(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.selectable_objects => pub fn selectable_objects(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.selected_objects => pub fn selected_objects(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.editable_objects => pub fn editable_objects(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.selected_editable_objects => pub fn selected_editable_objects(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.objects_in_mode => pub fn objects_in_mode(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.objects_in_mode_unique_data => pub fn objects_in_mode_unique_data(py: Python) -> Result<Vec<Object>> }
bind_python! { bpy.context.visible_bones => pub fn visible_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.editable_bones => pub fn editable_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_bones => pub fn selected_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_editable_bones => pub fn selected_editable_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.visible_pose_bones => pub fn visible_pose_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_pose_bones => pub fn selected_pose_bones(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_pose_bones_from_active_object => pub fn selected_pose_bones_from_active_object(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_bone => pub fn active_bone(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.active_pose_bone => pub fn active_pose_bone(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.active_object => pub fn active_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.object => pub fn object(py: Python) -> Result<Object> }
bind_python! { bpy.context.edit_object => pub fn edit_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.sculpt_object => pub fn sculpt_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.vertex_paint_object => pub fn vertex_paint_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.weight_paint_object => pub fn weight_paint_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.image_paint_object => pub fn image_paint_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.particle_edit_object => pub fn particle_edit_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.pose_object => pub fn pose_object(py: Python) -> Result<Object> }
bind_python! { bpy.context.active_sequence_strip => pub fn active_sequence_strip(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.sequences => pub fn sequences(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_sequences => pub fn selected_sequences(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_editable_sequences => pub fn selected_editable_sequences(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_nla_track => pub fn active_nla_track(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.active_nla_strip => pub fn active_nla_strip(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_nla_strips => pub fn selected_nla_strips(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_movieclip_tracks => pub fn selected_movieclip_tracks(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.gpencil_data => pub fn gpencil_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.gpencil_data_owner => pub fn gpencil_data_owner(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.annotation_data => pub fn annotation_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.annotation_data_owner => pub fn annotation_data_owner(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.visible_gpencil_layers => pub fn visible_gpencil_layers(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.editable_gpencil_layers => pub fn editable_gpencil_layers(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.editable_gpencil_strokes => pub fn editable_gpencil_strokes(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_gpencil_layer => pub fn active_gpencil_layer(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_gpencil_frame => pub fn active_gpencil_frame(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_annotation_layer => pub fn active_annotation_layer(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.active_operator => pub fn active_operator(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.active_action => pub fn active_action(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_visible_actions => pub fn selected_visible_actions(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_editable_actions => pub fn selected_editable_actions(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.visible_fcurves => pub fn visible_fcurves(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.editable_fcurves => pub fn editable_fcurves(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_visible_fcurves => pub fn selected_visible_fcurves(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.selected_editable_fcurves => pub fn selected_editable_fcurves(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.active_editable_fcurve => pub fn active_editable_fcurve(py: Python) -> Result<&PyAny> }
bind_python! { bpy.context.selected_editable_keyframes => pub fn selected_editable_keyframes(py: Python) -> Result<Vec<&PyAny>> }
bind_python! { bpy.context.ui_list => pub fn ui_list(py: Python) -> Result<&PyAny> }

// Text context <https://docs.blender.org/api/latest/bpy.context.html>
bind_python! { bpy.context.edit_text => pub fn edit_text(py: Python) -> Result<&PyAny> }

bind_python! { bpy.context.evaluated_depsgraph_get => pub fn evaluated_depsgraph_get(py: Python) -> Result<&PyAny> }
