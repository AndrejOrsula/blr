//! Bindings for [`bpy.ops`](https://docs.blender.org/api/latest/bpy.ops.html).
use super::{
    bind_python, Alignment, AxisXYZ, HashSet, LightType, MeshSelectMode, ModifierType, ObjectMode,
    ObjectType, OriginCenter, OriginType, Path, PyAny, SnapElement, TransformOrientation,
};

// /// <https://docs.blender.org/api/latest/bpy.ops.action.html>
// pub mod action {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.anim.html>
// pub mod anim {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.armature.html>
// pub mod armature {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.asset.html>
pub mod asset {
    use super::{bind_python, Path};

    bind_python! { bpy.ops.asset.assign_action() => pub fn assign_action(py: Python) }
    bind_python! { bpy.ops.asset.bundle_install() => pub fn bundle_install(py: Python, asset_library_ref: &str, filepath: &Path) }
    bind_python! { bpy.ops.asset.catalog_delete() => pub fn catalog_delete(py: Python, catalog_id: &str) }
    bind_python! { bpy.ops.asset.catalog_new() => pub fn catalog_new(py: Python, parent_path: &str) }
    bind_python! { bpy.ops.asset.catalog_redo() => pub fn catalog_redo(py: Python) }
    bind_python! { bpy.ops.asset.catalog_undo() => pub fn catalog_undo(py: Python) }
    bind_python! { bpy.ops.asset.catalog_undo_push() => pub fn catalog_undo_push(py: Python) }
    bind_python! { bpy.ops.asset.catalogs_save() => pub fn catalogs_save(py: Python) }
    bind_python! { bpy.ops.asset.clear() => pub fn clear(py: Python, set_fake_user: bool) }
    bind_python! { bpy.ops.asset.library_refresh() => pub fn library_refresh(py: Python) }
    bind_python! { bpy.ops.asset.mark() => pub fn mark(py: Python) }
    bind_python! { bpy.ops.asset.open_containing_blend_file() => pub fn open_containing_blend_file(py: Python) }
    bind_python! { bpy.ops.asset.tag_add() => pub fn tag_add(py: Python) }
    bind_python! { bpy.ops.asset.tag_remove() => pub fn tag_remove(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.boid.html>
// pub mod boid {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.brush.html>
// pub mod brush {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.buttons.html>
// pub mod buttons {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.cachefile.html>
// pub mod cachefile {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.camera.html>
// pub mod camera {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.clip.html>
// pub mod clip {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.cloth.html>
// pub mod cloth {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.collection.html>
pub mod collection {
    use super::bind_python;

    bind_python! { bpy.ops.collection.create() => pub fn create(py: Python, name: &str) }
    bind_python! { bpy.ops.collection.objects_add_active() => pub fn objects_add_active(py: Python, collection: &str) }
    bind_python! { bpy.ops.collection.objects_remove() => pub fn objects_remove(py: Python, collection: &str) }
    bind_python! { bpy.ops.collection.objects_remove_active() => pub fn objects_remove_active(py: Python, collection: &str) }
    bind_python! { bpy.ops.collection.objects_remove_all() => pub fn objects_remove_all(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.console.html>
// pub mod console {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.constraint.html>
// pub mod constraint {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.curve.html>
pub mod curve {
    use super::{bind_python, Alignment};

    bind_python! { bpy.ops.curve.cyclic_toggle() => pub fn cyclic_toggle(py: Python, direction: &str) }
    bind_python! { bpy.ops.curve.de_select_first() => pub fn de_select_first(py: Python) }
    bind_python! { bpy.ops.curve.de_select_last() => pub fn de_select_last(py: Python) }
    bind_python! { bpy.ops.curve.decimate() => pub fn decimate(py: Python, ratio: f32) }
    bind_python! { bpy.ops.curve.delete() => pub fn delete(py: Python, r#type: &str) }
    bind_python! { bpy.ops.curve.dissolve_verts() => pub fn dissolve_verts(py: Python) }
    bind_python! { bpy.ops.curve.duplicate() => pub fn duplicate(py: Python) }
    bind_python! { bpy.ops.curve.extrude() => pub fn extrude(py: Python, mode: &str) }
    bind_python! { bpy.ops.curve.handle_type_set() => pub fn handle_type_set(py: Python, r#type: &str) }
    bind_python! { bpy.ops.curve.hide() => pub fn hide(py: Python, unselected: bool) }
    bind_python! { bpy.ops.curve.make_segment() => pub fn make_segment(py: Python) }
    bind_python! { bpy.ops.curve.match_texture_space() => pub fn match_texture_space(py: Python) }
    bind_python! { bpy.ops.curve.normals_make_consistent() => pub fn normals_make_consistent(py: Python, calc_length: bool) }
    bind_python! { bpy.ops.curve.pen() => pub fn pen(py: Python, extend: bool, deselect: bool, toggle: bool, deselect_all: bool, select_passthrough: bool, extrude_point: bool, extrude_handle: &str, delete_point: bool, insert_point: bool, move_segment: bool, select_point: bool, move_point: bool, close_spline: bool, close_spline_method: &str, toggle_vector: bool, cycle_handle_type: bool) }
    bind_python! { bpy.ops.curve.primitive_bezier_circle_add() => pub fn primitive_bezier_circle_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.curve.primitive_bezier_curve_add() => pub fn primitive_bezier_curve_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.curve.primitive_nurbs_circle_add() => pub fn primitive_nurbs_circle_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.curve.primitive_nurbs_curve_add() => pub fn primitive_nurbs_curve_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.curve.primitive_nurbs_path_add() => pub fn primitive_nurbs_path_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.curve.radius_set() => pub fn radius_set(py: Python, radius: f32) }
    bind_python! { bpy.ops.curve.reveal() => pub fn reveal(py: Python, select: bool) }
    bind_python! { bpy.ops.curve.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.curve.select_less() => pub fn select_less(py: Python) }
    bind_python! { bpy.ops.curve.select_linked() => pub fn select_linked(py: Python) }
    bind_python! { bpy.ops.curve.select_linked_pick() => pub fn select_linked_pick(py: Python, deselect: bool) }
    bind_python! { bpy.ops.curve.select_more() => pub fn select_more(py: Python) }
    bind_python! { bpy.ops.curve.select_next() => pub fn select_next(py: Python) }
    bind_python! { bpy.ops.curve.select_nth() => pub fn select_nth(py: Python, skip: u32, nth: u32, offset: i32) }
    bind_python! { bpy.ops.curve.select_previous() => pub fn select_previous(py: Python) }
    bind_python! { bpy.ops.curve.select_random() => pub fn select_random(py: Python, ratio: f32, seed: u32, action: &str) }
    bind_python! { bpy.ops.curve.select_row() => pub fn select_row(py: Python) }
    bind_python! { bpy.ops.curve.select_similar() => pub fn select_similar(py: Python, r#type: &str, compare: &str, threshold: f32) }
    bind_python! { bpy.ops.curve.separate() => pub fn separate(py: Python, confirm: bool) }
    bind_python! { bpy.ops.curve.shade_flat() => pub fn shade_flat(py: Python) }
    bind_python! { bpy.ops.curve.shade_smooth() => pub fn shade_smooth(py: Python) }
    bind_python! { bpy.ops.curve.shortest_path_pick() => pub fn shortest_path_pick(py: Python) }
    bind_python! { bpy.ops.curve.smooth() => pub fn smooth(py: Python) }
    bind_python! { bpy.ops.curve.smooth_radius() => pub fn smooth_radius(py: Python) }
    bind_python! { bpy.ops.curve.smooth_tilt() => pub fn smooth_tilt(py: Python) }
    bind_python! { bpy.ops.curve.smooth_weight() => pub fn smooth_weight(py: Python) }
    bind_python! { bpy.ops.curve.spin() => pub fn spin(py: Python, center: [f32; 3], axis: [f32; 3]) }
    bind_python! { bpy.ops.curve.spline_type_set() => pub fn spline_type_set(py: Python, r#type: &str, use_handles: bool) }
    bind_python! { bpy.ops.curve.spline_weight_set() => pub fn spline_weight_set(py: Python, weight: f32) }
    bind_python! { bpy.ops.curve.split() => pub fn split(py: Python) }
    bind_python! { bpy.ops.curve.subdivide() => pub fn subdivide(py: Python, number_cuts: u32) }
    bind_python! { bpy.ops.curve.switch_direction() => pub fn switch_direction(py: Python) }
    bind_python! { bpy.ops.curve.tilt_clear() => pub fn tilt_clear(py: Python) }
    bind_python! { bpy.ops.curve.vertex_add() => pub fn vertex_add(py: Python, location: [f32; 3]) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.curves.html>
// pub mod curves {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.cycles.html>
// pub mod cycles {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.dpaint.html>
// pub mod dpaint {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.ed.html>
// pub mod ed {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.export_anim.html>
pub mod export_anim {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.export_anim.bvh => pub fn fn_bvh(py: Python) -> Result<&PyAny> }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_mesh.html>
pub mod export_mesh {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.export_mesh.ply => pub fn fn_ply(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.export_mesh.stl => pub fn fn_stl(py: Python) -> Result<&PyAny> }
}

/// <https://docs.blender.org/api/latest/bpy.ops.export_scene.html>
pub mod export_scene {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.export_scene.fbx => pub fn fn_fbx(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.export_scene.gltf => pub fn fn_gltf(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.export_scene.obj => pub fn fn_obj(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.export_scene.x3d => pub fn fn_x3d(py: Python) -> Result<&PyAny> }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.file.html>
// pub mod file {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.fluid.html>
// pub mod fluid {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.font.html>
// pub mod font {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.geometry.html>
// pub mod geometry {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.gizmogroup.html>
// pub mod gizmogroup {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.gpencil.html>
// pub mod gpencil {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.graph.html>
// pub mod graph {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.image.html>
pub mod image {
    use super::{bind_python, Path};

    bind_python! { bpy.ops.image.add_render_slot() => pub fn add_render_slot(py: Python) }
    bind_python! { bpy.ops.image.change_frame() => pub fn change_frame(py: Python, frame: u32) }
    bind_python! { bpy.ops.image.clear_render_border() => pub fn clear_render_border(py: Python) }
    bind_python! { bpy.ops.image.clear_render_slot() => pub fn clear_render_slot(py: Python) }
    bind_python! { bpy.ops.image.clipboard_copy() => pub fn clipboard_copy(py: Python) }
    bind_python! { bpy.ops.image.clipboard_paste() => pub fn clipboard_paste(py: Python) }
    bind_python! { bpy.ops.image.curves_point_set() => pub fn curves_point_set(py: Python, point: &str, size: u32) }
    bind_python! { bpy.ops.image.cycle_render_slot() => pub fn cycle_render_slot(py: Python, reverse: bool) }
    bind_python! { bpy.ops.image.external_edit() => pub fn external_edit(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.image.file_browse() => pub fn file_browse(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.image.flip() => pub fn flip(py: Python, use_flip_x: bool, use_flip_y: bool) }
    bind_python! { bpy.ops.image.invert() => pub fn invert(py: Python, invert_r: bool, invert_g: bool, invert_b: bool, invert_a: bool) }
    bind_python! { bpy.ops.image.match_movie_length() => pub fn match_movie_length(py: Python) }
    bind_python! { bpy.ops.image.new() => pub fn new(py: Python, name: &str, width: u32, height: u32, color: [f32; 4], alpha: bool, generated_type: &str, float: bool, use_stereo_3d: bool, tiled: bool) }
    bind_python! { bpy.ops.image.pack() => pub fn pack(py: Python) }
    bind_python! { bpy.ops.image.project_apply() => pub fn project_apply(py: Python) }
    bind_python! { bpy.ops.image.project_edit() => pub fn project_edit(py: Python) }
    bind_python! { bpy.ops.image.read_viewlayers() => pub fn read_viewlayers(py: Python) }
    bind_python! { bpy.ops.image.reload() => pub fn reload(py: Python) }
    bind_python! { bpy.ops.image.remove_render_slot() => pub fn remove_render_slot(py: Python) }
    bind_python! { bpy.ops.image.render_border() => pub fn render_border(py: Python, xmin: i32, xmax: i32, ymin: i32, ymax: i32, wait_for_input: bool) }
    bind_python! { bpy.ops.image.replace() => pub fn replace(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.image.resize() => pub fn resize(py: Python, size: [u32; 2]) }
    bind_python! { bpy.ops.image.sample() => pub fn sample(py: Python, size: u32) }
    bind_python! { bpy.ops.image.sample_line() => pub fn sample_line(py: Python, xstart: i32, xend: i32, ystart: i32, yend: i32, flip: bool, cursor: u32) }
    bind_python! { bpy.ops.image.save() => pub fn save(py: Python) }
    bind_python! { bpy.ops.image.save_all_modified() => pub fn save_all_modified(py: Python) }
    bind_python! { bpy.ops.image.save_as() => pub fn save_as(py: Python, save_as_render: bool, copy: bool, allow_path_tokens: bool, filepath: &Path) }
    bind_python! { bpy.ops.image.save_sequence() => pub fn save_sequence(py: Python) }
    bind_python! { bpy.ops.image.tile_add() => pub fn tile_add(py: Python, number: u32, count: u32, label: &str, fill: bool, color: [f32; 4], generated_type: &str, width: u32, height: u32, float: bool, alpha: bool) }
    bind_python! { bpy.ops.image.tile_fill() => pub fn tile_fill(py: Python, color: [f32; 4], generated_type: &str, width: u32, height: u32, float: bool, alpha: bool) }
    bind_python! { bpy.ops.image.tile_remove() => pub fn tile_remove(py: Python) }
    bind_python! { bpy.ops.image.unpack() => pub fn unpack(py: Python, method: &str, id: &str) }
    bind_python! { bpy.ops.image.view_all() => pub fn view_all(py: Python, fit_view: bool) }
    bind_python! { bpy.ops.image.view_center_cursor() => pub fn view_center_cursor(py: Python) }
    bind_python! { bpy.ops.image.view_cursor_center() => pub fn view_cursor_center(py: Python, fit_view: bool) }
    bind_python! { bpy.ops.image.view_ndof() => pub fn view_ndof(py: Python) }
    bind_python! { bpy.ops.image.view_pan() => pub fn view_pan(py: Python, offset: [f32; 2]) }
    bind_python! { bpy.ops.image.view_selected() => pub fn view_selected(py: Python) }
    bind_python! { bpy.ops.image.view_zoom() => pub fn view_zoom(py: Python, factor: f32, use_cursor_init: bool) }
    bind_python! { bpy.ops.image.view_zoom_border() => pub fn view_zoom_border(py: Python, xmin: i32, xmax: i32, ymin: i32, ymax: i32, wait_for_input: bool, zoom_out: bool) }
    bind_python! { bpy.ops.image.view_zoom_in() => pub fn view_zoom_in(py: Python, location: [f32; 2]) }
    bind_python! { bpy.ops.image.view_zoom_out() => pub fn view_zoom_out(py: Python, location: [f32; 2]) }
    bind_python! { bpy.ops.image.view_zoom_ratio() => pub fn view_zoom_ratio(py: Python, ratio: f32) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_anim.html>
pub mod import_anim {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.import_anim.bvh => pub fn fn_bvh(py: Python) -> Result<&PyAny> }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_curve.html>
pub mod import_curve {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.import_curve.svg => pub fn fn_svg(py: Python) -> Result<&PyAny> }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_mesh.html>
pub mod import_mesh {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.import_mesh.ply => pub fn fn_ply(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.import_mesh.stl => pub fn fn_stl(py: Python) -> Result<&PyAny> }
}

/// <https://docs.blender.org/api/latest/bpy.ops.import_scene.html>
pub mod import_scene {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.import_scene.fbx => pub fn fn_fbx(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.import_scene.gltf => pub fn fn_gltf(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.import_scene.obj => pub fn fn_obj(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.import_scene.x3d => pub fn fn_x3d(py: Python) -> Result<&PyAny> }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.info.html>
// pub mod info {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.lattice.html>
// pub mod lattice {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.marker.html>
// pub mod marker {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.mask.html>
// pub mod mask {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.material.html>
pub mod material {
    use super::bind_python;

    bind_python! { bpy.ops.material.copy() => pub fn copy(py: Python) }
    bind_python! { bpy.ops.material.new() => pub fn new(py: Python) }
    bind_python! { bpy.ops.material.paste() => pub fn paste(py: Python) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.mball.html>
pub mod mball {
    use super::bind_python;

    bind_python! { bpy.ops.mball.delete_metaelems() => pub fn delete_metaelems(py: Python, confirm: bool) }
    bind_python! { bpy.ops.mball.duplicate_metaelems() => pub fn duplicate_metaelems(py: Python) }
    bind_python! { bpy.ops.mball.hide_metaelems() => pub fn hide_metaelems(py: Python, unselected: bool) }
    bind_python! { bpy.ops.mball.reveal_metaelems() => pub fn reveal_metaelems(py: Python, select: bool) }
    bind_python! { bpy.ops.mball.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.mball.select_random_metaelems() => pub fn select_random_metaelems(py: Python, ratio: f32, seed: u32, action: &str) }
    bind_python! { bpy.ops.mball.select_similar() => pub fn select_similar(py: Python, r#type: &str, threshold: f32) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.mesh.html>
pub mod mesh {
    use super::{bind_python, Alignment, HashSet, MeshSelectMode};

    bind_python! { bpy.ops.mesh.attribute_set() => pub fn attribute_set(py: Python, value_float: f32, value_float_vector_2d: [f32; 2], value_float_vector_3d: [f32; 3], value_int: i32, value_int_vector_2d: [u32; 2], value_color: [f32; 4], value_bool: bool) }
    bind_python! { bpy.ops.mesh.average_normals() => pub fn average_normals(py: Python, average_type: &str, weight: u32, threshold: f32) }
    bind_python! { bpy.ops.mesh.beautify_fill() => pub fn beautify_fill(py: Python, angle_limit: f32) }
    bind_python! { bpy.ops.mesh.bevel() => pub fn bevel(py: Python, offset_type: &str, offset: f32, profile_type: &str, offset_pct: f32, segments: u32, profile: f32, affect: &str, clamp_overlap: bool, loop_slide: bool, mark_seam: bool, mark_sharp: bool, material: i32, harden_normals: bool, face_strength_mode: &str, miter_outer: &str, miter_inner: &str, spread: f32, vmesh_method: &str, release_confirm: bool) }
    bind_python! { bpy.ops.mesh.bisect() => pub fn bisect(py: Python, plane_co: [f32; 3], plane_no: [f32; 3], use_fill: bool, clear_inner: bool, clear_outer: bool, threshold: f32, xstart: i32, xend: i32, ystart: i32, yend: i32, flip: bool, cursor: u32) }
    bind_python! { bpy.ops.mesh.blend_from_shape() => pub fn blend_from_shape(py: Python, shape: &str, blend: f32, add: bool) }
    bind_python! { bpy.ops.mesh.bridge_edge_loops() => pub fn bridge_edge_loops(py: Python, r#type: &str, use_merge: bool, merge_factor: f32, twist_offset: i32, number_cuts: i32, interpolation: &str, smoothness: f32, profile_shape_factor: f32, profile_shape: &str) }
    bind_python! { bpy.ops.mesh.colors_reverse() => pub fn colors_reverse(py: Python) }
    bind_python! { bpy.ops.mesh.colors_rotate() => pub fn colors_rotate(py: Python, use_ccw: bool) }
    bind_python! { bpy.ops.mesh.customdata_bevel_weight_edge_add() => pub fn customdata_bevel_weight_edge_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_bevel_weight_edge_clear() => pub fn customdata_bevel_weight_edge_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_bevel_weight_vertex_add() => pub fn customdata_bevel_weight_vertex_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_bevel_weight_vertex_clear() => pub fn customdata_bevel_weight_vertex_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_crease_edge_add() => pub fn customdata_crease_edge_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_crease_edge_clear() => pub fn customdata_crease_edge_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_crease_vertex_add() => pub fn customdata_crease_vertex_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_crease_vertex_clear() => pub fn customdata_crease_vertex_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_custom_splitnormals_add() => pub fn customdata_custom_splitnormals_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_custom_splitnormals_clear() => pub fn customdata_custom_splitnormals_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_mask_clear() => pub fn customdata_mask_clear(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_skin_add() => pub fn customdata_skin_add(py: Python) }
    bind_python! { bpy.ops.mesh.customdata_skin_clear() => pub fn customdata_skin_clear(py: Python) }
    bind_python! { bpy.ops.mesh.decimate() => pub fn decimate(py: Python, ratio: f32, use_vertex_group: bool, vertex_group_factor: f32, invert_vertex_group: bool, use_symmetry: bool, symmetry_axis: &str) }
    bind_python! { bpy.ops.mesh.delete() => pub fn delete(py: Python, r#type: &str) }
    bind_python! { bpy.ops.mesh.delete_edgeloop() => pub fn delete_edgeloop(py: Python, use_face_split: bool) }
    bind_python! { bpy.ops.mesh.delete_loose() => pub fn delete_loose(py: Python, use_verts: bool, use_edges: bool, use_faces: bool) }
    bind_python! { bpy.ops.mesh.dissolve_degenerate() => pub fn dissolve_degenerate(py: Python, threshold: f32) }
    bind_python! { bpy.ops.mesh.dissolve_edges() => pub fn dissolve_edges(py: Python, use_verts: bool, use_face_split: bool) }
    bind_python! { bpy.ops.mesh.dissolve_faces() => pub fn dissolve_faces(py: Python, use_verts: bool) }
    bind_python! { bpy.ops.mesh.dissolve_limited() => pub fn dissolve_limited(py: Python, angle_limit: f32, use_dissolve_boundaries: bool, delimit: HashSet<String>) }
    bind_python! { bpy.ops.mesh.dissolve_mode() => pub fn dissolve_mode(py: Python, use_verts: bool, use_face_split: bool, use_boundary_tear: bool) }
    bind_python! { bpy.ops.mesh.dissolve_verts() => pub fn dissolve_verts(py: Python, use_face_split: bool, use_boundary_tear: bool) }
    bind_python! { bpy.ops.mesh.dupli_extrude_cursor() => pub fn dupli_extrude_cursor(py: Python, rotate_source: bool) }
    bind_python! { bpy.ops.mesh.duplicate() => pub fn duplicate(py: Python, mode: u32) }
    bind_python! { bpy.ops.mesh.edge_collapse() => pub fn edge_collapse(py: Python) }
    bind_python! { bpy.ops.mesh.edge_face_add() => pub fn edge_face_add(py: Python) }
    bind_python! { bpy.ops.mesh.edge_rotate() => pub fn edge_rotate(py: Python, use_ccw: bool) }
    bind_python! { bpy.ops.mesh.edge_split() => pub fn edge_split(py: Python, r#type: &str) }
    bind_python! { bpy.ops.mesh.edgering_select() => pub fn edgering_select(py: Python, extend: bool, deselect: bool, toggle: bool, ring: bool) }
    bind_python! { bpy.ops.mesh.edges_select_sharp() => pub fn edges_select_sharp(py: Python, sharpness: f32) }
    bind_python! { bpy.ops.mesh.extrude_context() => pub fn extrude_context(py: Python, use_normal_flip: bool, use_dissolve_ortho_edges: bool, mirror: bool) }
    bind_python! { bpy.ops.mesh.extrude_edges_indiv() => pub fn extrude_edges_indiv(py: Python, use_normal_flip: bool, mirror: bool) }
    bind_python! { bpy.ops.mesh.extrude_faces_indiv() => pub fn extrude_faces_indiv(py: Python, mirror: bool) }
    bind_python! { bpy.ops.mesh.extrude_region() => pub fn extrude_region(py: Python, use_normal_flip: bool, use_dissolve_ortho_edges: bool, mirror: bool) }
    bind_python! { bpy.ops.mesh.extrude_repeat() => pub fn extrude_repeat(py: Python, steps: u32, offset: [f32; 3], scale_offset: f32) }
    bind_python! { bpy.ops.mesh.extrude_verts_indiv() => pub fn extrude_verts_indiv(py: Python, mirror: bool) }
    bind_python! { bpy.ops.mesh.face_make_planar() => pub fn face_make_planar(py: Python, factor: f32, repeat: u32) }
    bind_python! { bpy.ops.mesh.face_set_extract() => pub fn face_set_extract(py: Python, add_boundary_loop: bool, smooth_iterations: u32, apply_shrinkwrap: bool, add_solidify: bool) }
    bind_python! { bpy.ops.mesh.face_split_by_edges() => pub fn face_split_by_edges(py: Python) }
    bind_python! { bpy.ops.mesh.faces_mirror_uv() => pub fn faces_mirror_uv(py: Python, direction: &str, precision: u32) }
    bind_python! { bpy.ops.mesh.faces_select_linked_flat() => pub fn faces_select_linked_flat(py: Python, sharpness: f32) }
    bind_python! { bpy.ops.mesh.faces_shade_flat() => pub fn faces_shade_flat(py: Python) }
    bind_python! { bpy.ops.mesh.faces_shade_smooth() => pub fn faces_shade_smooth(py: Python) }
    bind_python! { bpy.ops.mesh.fill() => pub fn fill(py: Python, use_beauty: bool) }
    bind_python! { bpy.ops.mesh.fill_grid() => pub fn fill_grid(py: Python, span: u32, offset: i32, use_interp_simple: bool) }
    bind_python! { bpy.ops.mesh.fill_holes() => pub fn fill_holes(py: Python, sides: u32) }
    bind_python! { bpy.ops.mesh.flip_normals() => pub fn flip_normals(py: Python, only_clnors: bool) }
    bind_python! { bpy.ops.mesh.flip_quad_tessellation() => pub fn flip_quad_tessellation(py: Python) }
    bind_python! { bpy.ops.mesh.hide() => pub fn hide(py: Python, unselected: bool) }
    bind_python! { bpy.ops.mesh.inset() => pub fn inset(py: Python, use_boundary: bool, use_even_offset: bool, use_relative_offset: bool, use_edge_rail: bool, thickness: f32, depth: f32, use_outset: bool, use_select_inset: bool, use_individual: bool, use_interpolate: bool, release_confirm: bool) }
    bind_python! { bpy.ops.mesh.intersect() => pub fn intersect(py: Python, mode: &str, separate_mode: &str, threshold: f32, solver: &str) }
    bind_python! { bpy.ops.mesh.intersect_boolean() => pub fn intersect_boolean(py: Python, operation: &str, use_swap: bool, use_self: bool, threshold: f32, solver: &str) }
    bind_python! { bpy.ops.mesh.knife_project() => pub fn knife_project(py: Python, cut_through: bool) }
    bind_python! { bpy.ops.mesh.knife_tool() => pub fn knife_tool(py: Python, use_occlude_geometry: bool, only_selected: bool, xray: bool, visible_measurements: &str, angle_snapping: &str, angle_snapping_increment: f32, wait_for_input: bool) }
    bind_python! { bpy.ops.mesh.loop_multi_select() => pub fn loop_multi_select(py: Python, ring: bool) }
    bind_python! { bpy.ops.mesh.loop_select() => pub fn loop_select(py: Python, extend: bool, deselect: bool, toggle: bool, ring: bool) }
    bind_python! { bpy.ops.mesh.loop_to_region() => pub fn loop_to_region(py: Python, select_bigger: bool) }
    bind_python! { bpy.ops.mesh.loopcut() => pub fn loopcut(py: Python, number_cuts: u32, smoothness: f32, falloff: &str, object_index: i32, edge_index: i32, mesh_select_mode_init: [bool; 3]) }
    bind_python! { bpy.ops.mesh.mark_freestyle_edge() => pub fn mark_freestyle_edge(py: Python, clear: bool) }
    bind_python! { bpy.ops.mesh.mark_freestyle_face() => pub fn mark_freestyle_face(py: Python, clear: bool) }
    bind_python! { bpy.ops.mesh.mark_seam() => pub fn mark_seam(py: Python, clear: bool) }
    bind_python! { bpy.ops.mesh.mark_sharp() => pub fn mark_sharp(py: Python, clear: bool, use_verts: bool) }
    bind_python! { bpy.ops.mesh.merge() => pub fn merge(py: Python, r#type: &str, uvs: bool) }
    bind_python! { bpy.ops.mesh.merge_normals() => pub fn merge_normals(py: Python) }
    bind_python! { bpy.ops.mesh.mod_weighted_strength() => pub fn mod_weighted_strength(py: Python, set: bool, face_strength: &str) }
    bind_python! { bpy.ops.mesh.normals_make_consistent() => pub fn normals_make_consistent(py: Python, inside: bool) }
    bind_python! { bpy.ops.mesh.normals_tools() => pub fn normals_tools(py: Python, mode: &str, absolute: bool) }
    bind_python! { bpy.ops.mesh.offset_edge_loops() => pub fn offset_edge_loops(py: Python, use_cap_endpoint: bool) }
    bind_python! { bpy.ops.mesh.paint_mask_extract() => pub fn paint_mask_extract(py: Python, mask_threshold: f32, add_boundary_loop: bool, smooth_iterations: u32, apply_shrinkwrap: bool, add_solidify: bool) }
    bind_python! { bpy.ops.mesh.paint_mask_slice() => pub fn paint_mask_slice(py: Python, mask_threshold: f32, fill_holes: bool, new_object: bool) }
    bind_python! { bpy.ops.mesh.point_normals() => pub fn point_normals(py: Python, mode: &str, invert: bool, align: bool, target_location: [f32; 3], spherize: bool, spherize_strength: f32) }
    bind_python! { bpy.ops.mesh.poke() => pub fn poke(py: Python, offset: f32, use_relative_offset: bool, center_mode: &str) }
    bind_python! { bpy.ops.mesh.polybuild_delete_at_cursor() => pub fn polybuild_delete_at_cursor(py: Python, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.mesh.polybuild_dissolve_at_cursor() => pub fn polybuild_dissolve_at_cursor(py: Python) }
    bind_python! { bpy.ops.mesh.polybuild_face_at_cursor() => pub fn polybuild_face_at_cursor(py: Python, create_quads: bool, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.mesh.polybuild_split_at_cursor() => pub fn polybuild_split_at_cursor(py: Python, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.mesh.polybuild_transform_at_cursor() => pub fn polybuild_transform_at_cursor(py: Python, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.mesh.primitive_circle_add() => pub fn primitive_circle_add(py: Python, vertices: u32, radius: f32, fill_type: &str, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_cone_add() => pub fn primitive_cone_add(py: Python, vertices: u32, radius1: f32, radius2: f32, depth: f32, end_fill_type: &str, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_cube_add() => pub fn primitive_cube_add(py: Python, size: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_cube_add_gizmo() => pub fn primitive_cube_add_gizmo(py: Python, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3], matrix: ([f32; 4], [f32; 4], [f32; 4], [f32; 4])) }
    bind_python! { bpy.ops.mesh.primitive_cylinder_add() => pub fn primitive_cylinder_add(py: Python, vertices: u32, radius: f32, depth: f32, end_fill_type: &str, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_grid_add() => pub fn primitive_grid_add(py: Python, x_subdivisions: u32, y_subdivisions: u32, size: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_ico_sphere_add() => pub fn primitive_ico_sphere_add(py: Python, subdivisions: u32, radius: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_monkey_add() => pub fn primitive_monkey_add(py: Python, size: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_plane_add() => pub fn primitive_plane_add(py: Python, size: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.primitive_torus_add() => pub fn primitive_torus_add(py: Python, align: Alignment, location: [f32; 3], rotation: [f32; 3], major_segments: u32, minor_segments: u32, mode: &str, major_radius: f32, minor_radius: f32, abso_major_rad: f32, abso_minor_rad: f32, generate_uvs: bool) }
    bind_python! { bpy.ops.mesh.primitive_uv_sphere_add() => pub fn primitive_uv_sphere_add(py: Python, segments: u32, ring_count: u32, radius: f32, calc_uvs: bool, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.mesh.quads_convert_to_tris() => pub fn quads_convert_to_tris(py: Python, quad_method: &str, ngon_method: &str) }
    bind_python! { bpy.ops.mesh.region_to_loop() => pub fn region_to_loop(py: Python) }
    bind_python! { bpy.ops.mesh.remove_doubles() => pub fn remove_doubles(py: Python, threshold: f32, use_unselected: bool, use_sharp_edge_from_normals: bool) }
    bind_python! { bpy.ops.mesh.reveal() => pub fn reveal(py: Python, select: bool) }
    bind_python! { bpy.ops.mesh.rip() => pub fn rip(py: Python, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool, use_fill: bool) }
    bind_python! { bpy.ops.mesh.rip_edge() => pub fn rip_edge(py: Python, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.mesh.screw() => pub fn screw(py: Python, steps: u32, turns: u32, center: [f32; 3], axis: [f32; 3]) }
    bind_python! { bpy.ops.mesh.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.mesh.select_axis() => pub fn select_axis(py: Python, orientation: &str, sign: &str, axis: &str, threshold: f32) }
    bind_python! { bpy.ops.mesh.select_face_by_sides() => pub fn select_face_by_sides(py: Python, number: u32, r#type: &str, extend: bool) }
    bind_python! { bpy.ops.mesh.select_interior_faces() => pub fn select_interior_faces(py: Python) }
    bind_python! { bpy.ops.mesh.select_less() => pub fn select_less(py: Python, use_face_step: bool) }
    bind_python! { bpy.ops.mesh.select_linked() => pub fn select_linked(py: Python, delimit: HashSet<String>) }
    bind_python! { bpy.ops.mesh.select_linked_pick() => pub fn select_linked_pick(py: Python, deselect: bool, delimit: HashSet<String>, object_index: i32, index: i32) }
    bind_python! { bpy.ops.mesh.select_loose() => pub fn select_loose(py: Python, extend: bool) }
    bind_python! { bpy.ops.mesh.select_mirror() => pub fn select_mirror(py: Python, axis: HashSet<String>, extend: bool) }
    bind_python! { bpy.ops.mesh.select_mode() => pub fn select_mode(py: Python, use_extend: bool, use_expand: bool, r#type: MeshSelectMode, action: &str) }
    bind_python! { bpy.ops.mesh.select_more() => pub fn select_more(py: Python, use_face_step: bool) }
    bind_python! { bpy.ops.mesh.select_next_item() => pub fn select_next_item(py: Python) }
    bind_python! { bpy.ops.mesh.select_non_manifold() => pub fn select_non_manifold(py: Python, extend: bool, use_wire: bool, use_boundary: bool, use_multi_face: bool, use_non_contiguous: bool, use_verts: bool) }
    bind_python! { bpy.ops.mesh.select_nth() => pub fn select_nth(py: Python, skip: u32, nth: u32, offset: i32) }
    bind_python! { bpy.ops.mesh.select_prev_item() => pub fn select_prev_item(py: Python) }
    bind_python! { bpy.ops.mesh.select_random() => pub fn select_random(py: Python, ratio: f32, seed: u32, action: &str) }
    bind_python! { bpy.ops.mesh.select_similar() => pub fn select_similar(py: Python, r#type: &str, compare: &str, threshold: f32) }
    bind_python! { bpy.ops.mesh.select_similar_region() => pub fn select_similar_region(py: Python) }
    bind_python! { bpy.ops.mesh.select_ungrouped() => pub fn select_ungrouped(py: Python, extend: bool) }
    bind_python! { bpy.ops.mesh.separate() => pub fn separate(py: Python, r#type: &str) }
    bind_python! { bpy.ops.mesh.set_normals_from_faces() => pub fn set_normals_from_faces(py: Python, keep_sharp: bool) }
    bind_python! { bpy.ops.mesh.shape_propagate_to_all() => pub fn shape_propagate_to_all(py: Python) }
    bind_python! { bpy.ops.mesh.shortest_path_pick() => pub fn shortest_path_pick(py: Python, edge_mode: &str, use_face_step: bool, use_topology_distance: bool, use_fill: bool, skip: i32, nth: u32, offset: i32, index: i32) }
    bind_python! { bpy.ops.mesh.shortest_path_select() => pub fn shortest_path_select(py: Python, edge_mode: &str, use_face_step: bool, use_topology_distance: bool, use_fill: bool, skip: i32, nth: u32, offset: i32) }
    bind_python! { bpy.ops.mesh.smooth_normals() => pub fn smooth_normals(py: Python, factor: f32) }
    bind_python! { bpy.ops.mesh.solidify() => pub fn solidify(py: Python, thickness: f32) }
    bind_python! { bpy.ops.mesh.sort_elements() => pub fn sort_elements(py: Python, r#type: &str, elements: HashSet<String>, reverse: bool, seed: u32) }
    bind_python! { bpy.ops.mesh.spin() => pub fn spin(py: Python, steps: u32, dupli: bool, angle: f32, use_auto_merge: bool, use_normal_flip: bool, center: [f32; 3], axis: [f32; 3]) }
    bind_python! { bpy.ops.mesh.split() => pub fn split(py: Python) }
    bind_python! { bpy.ops.mesh.split_normals() => pub fn split_normals(py: Python) }
    bind_python! { bpy.ops.mesh.subdivide() => pub fn subdivide(py: Python, number_cuts: u32, smoothness: f32, ngon: bool, quadcorner: &str, fractal: f32, fractal_along_normal: f32, seed: u32) }
    bind_python! { bpy.ops.mesh.subdivide_edgering() => pub fn subdivide_edgering(py: Python, number_cuts: u32, interpolation: &str, smoothness: f32, profile_shape_factor: f32, profile_shape: &str) }
    bind_python! { bpy.ops.mesh.symmetrize() => pub fn symmetrize(py: Python, direction: &str, threshold: f32) }
    bind_python! { bpy.ops.mesh.symmetry_snap() => pub fn symmetry_snap(py: Python, direction: &str, threshold: f32, factor: f32, use_center: bool) }
    bind_python! { bpy.ops.mesh.tris_convert_to_quads() => pub fn tris_convert_to_quads(py: Python, face_threshold: f32, shape_threshold: f32, uvs: bool, vcols: bool, seam: bool, sharp: bool, materials: bool) }
    bind_python! { bpy.ops.mesh.unsubdivide() => pub fn unsubdivide(py: Python, iterations: u32) }
    bind_python! { bpy.ops.mesh.uv_texture_add() => pub fn uv_texture_add(py: Python) }
    bind_python! { bpy.ops.mesh.uv_texture_remove() => pub fn uv_texture_remove(py: Python) }
    bind_python! { bpy.ops.mesh.uvs_reverse() => pub fn uvs_reverse(py: Python) }
    bind_python! { bpy.ops.mesh.uvs_rotate() => pub fn uvs_rotate(py: Python, use_ccw: bool) }
    bind_python! { bpy.ops.mesh.vert_connect() => pub fn vert_connect(py: Python) }
    bind_python! { bpy.ops.mesh.vert_connect_concave() => pub fn vert_connect_concave(py: Python) }
    bind_python! { bpy.ops.mesh.vert_connect_nonplanar() => pub fn vert_connect_nonplanar(py: Python, angle_limit: f32) }
    bind_python! { bpy.ops.mesh.vert_connect_path() => pub fn vert_connect_path(py: Python) }
    bind_python! { bpy.ops.mesh.vertices_smooth() => pub fn vertices_smooth(py: Python, factor: f32, repeat: u32, xaxis: bool, yaxis: bool, zaxis: bool, wait_for_input: bool) }
    bind_python! { bpy.ops.mesh.vertices_smooth_laplacian() => pub fn vertices_smooth_laplacian(py: Python, repeat: u32, lambda_factor: f32, lambda_border: f32, use_x: bool, use_y: bool, use_z: bool, preserve_volume: bool) }
    bind_python! { bpy.ops.mesh.wireframe() => pub fn wireframe(py: Python, use_boundary: bool, use_even_offset: bool, use_relative_offset: bool, use_replace: bool, thickness: f32, offset: f32, use_crease: bool, crease_weight: f32) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.nla.html>
// pub mod nla {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.node.html>
pub mod node {
    use super::{bind_python, Path};

    bind_python! { bpy.ops.node.add_collection() => pub fn add_collection(py: Python, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.node.add_file() => pub fn add_file(py: Python, filepath: &Path, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.node.add_group() => pub fn add_group(py: Python, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.node.add_group_asset() => pub fn add_group_asset(py: Python) }
    bind_python! { bpy.ops.node.add_mask() => pub fn add_mask(py: Python, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.node.add_object() => pub fn add_object(py: Python, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.node.add_search() => pub fn add_search(py: Python, use_transform: bool) }
    bind_python! { bpy.ops.node.attach() => pub fn attach(py: Python) }
    bind_python! { bpy.ops.node.backimage_fit() => pub fn backimage_fit(py: Python) }
    bind_python! { bpy.ops.node.backimage_move() => pub fn backimage_move(py: Python) }
    bind_python! { bpy.ops.node.backimage_sample() => pub fn backimage_sample(py: Python) }
    bind_python! { bpy.ops.node.backimage_zoom() => pub fn backimage_zoom(py: Python, factor: f32) }
    bind_python! { bpy.ops.node.clear_viewer_border() => pub fn clear_viewer_border(py: Python) }
    bind_python! { bpy.ops.node.clipboard_copy() => pub fn clipboard_copy(py: Python) }
    bind_python! { bpy.ops.node.clipboard_paste() => pub fn clipboard_paste(py: Python, offset: [f32; 2]) }
    bind_python! { bpy.ops.node.collapse_hide_unused_toggle() => pub fn collapse_hide_unused_toggle(py: Python) }
    bind_python! { bpy.ops.node.cryptomatte_layer_add() => pub fn cryptomatte_layer_add(py: Python) }
    bind_python! { bpy.ops.node.cryptomatte_layer_remove() => pub fn cryptomatte_layer_remove(py: Python) }
    bind_python! { bpy.ops.node.deactivate_viewer() => pub fn deactivate_viewer(py: Python) }
    bind_python! { bpy.ops.node.delete() => pub fn delete(py: Python) }
    bind_python! { bpy.ops.node.delete_reconnect() => pub fn delete_reconnect(py: Python) }
    bind_python! { bpy.ops.node.detach() => pub fn detach(py: Python) }
    bind_python! { bpy.ops.node.duplicate() => pub fn duplicate(py: Python, keep_inputs: bool, linked: bool) }
    bind_python! { bpy.ops.node.find_node() => pub fn find_node(py: Python) }
    bind_python! { bpy.ops.node.gltf_settings_node_operator() => pub fn gltf_settings_node_operator(py: Python) }
    bind_python! { bpy.ops.node.group_edit() => pub fn group_edit(py: Python, exit: bool) }
    bind_python! { bpy.ops.node.group_insert() => pub fn group_insert(py: Python) }
    bind_python! { bpy.ops.node.group_make() => pub fn group_make(py: Python) }
    bind_python! { bpy.ops.node.group_separate() => pub fn group_separate(py: Python, r#type: &str) }
    bind_python! { bpy.ops.node.group_ungroup() => pub fn group_ungroup(py: Python) }
    bind_python! { bpy.ops.node.hide_socket_toggle() => pub fn hide_socket_toggle(py: Python) }
    bind_python! { bpy.ops.node.hide_toggle() => pub fn hide_toggle(py: Python) }
    bind_python! { bpy.ops.node.insert_offset() => pub fn insert_offset(py: Python) }
    bind_python! { bpy.ops.node.join() => pub fn join(py: Python) }
    bind_python! { bpy.ops.node.link() => pub fn link(py: Python, detach: bool, drag_start: [f32; 2], inside_padding: f32, outside_padding: f32, speed_ramp: f32, max_speed: f32, delay: f32, zoom_influence: f32) }
    bind_python! { bpy.ops.node.link_make() => pub fn link_make(py: Python, replace: bool) }
    bind_python! { bpy.ops.node.link_viewer() => pub fn link_viewer(py: Python) }
    bind_python! { bpy.ops.node.links_detach() => pub fn links_detach(py: Python) }
    bind_python! { bpy.ops.node.mute_toggle() => pub fn mute_toggle(py: Python) }
    bind_python! { bpy.ops.node.new_geometry_node_group_assign() => pub fn new_geometry_node_group_assign(py: Python) }
    bind_python! { bpy.ops.node.new_geometry_nodes_modifier() => pub fn new_geometry_nodes_modifier(py: Python) }
    bind_python! { bpy.ops.node.new_node_tree() => pub fn new_node_tree(py: Python, r#type: &str, name: &str) }
    bind_python! { bpy.ops.node.node_color_preset_add() => pub fn node_color_preset_add(py: Python, name: &str, remove_name: bool, remove_active: bool) }
    bind_python! { bpy.ops.node.node_copy_color() => pub fn node_copy_color(py: Python) }
    bind_python! { bpy.ops.node.options_toggle() => pub fn options_toggle(py: Python) }
    bind_python! { bpy.ops.node.output_file_add_socket() => pub fn output_file_add_socket(py: Python, file_path: &str) }
    bind_python! { bpy.ops.node.output_file_move_active_socket() => pub fn output_file_move_active_socket(py: Python, direction: &str) }
    bind_python! { bpy.ops.node.output_file_remove_active_socket() => pub fn output_file_remove_active_socket(py: Python) }
    bind_python! { bpy.ops.node.parent_set() => pub fn parent_set(py: Python) }
    bind_python! { bpy.ops.node.preview_toggle() => pub fn preview_toggle(py: Python) }
    bind_python! { bpy.ops.node.read_viewlayers() => pub fn read_viewlayers(py: Python) }
    bind_python! { bpy.ops.node.render_changed() => pub fn render_changed(py: Python) }
    bind_python! { bpy.ops.node.resize() => pub fn resize(py: Python) }
    bind_python! { bpy.ops.node.select() => pub fn select(py: Python, extend: bool, deselect: bool, toggle: bool, deselect_all: bool, select_passthrough: bool, location: [u32; 2], socket_select: bool, clear_viewer: bool) }
    bind_python! { bpy.ops.node.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.node.select_box() => pub fn select_box(py: Python, tweak: bool, xmin: i32, xmax: i32, ymin: i32, ymax: i32, wait_for_input: bool, mode: &str) }
    bind_python! { bpy.ops.node.select_circle() => pub fn select_circle(py: Python, x: i32, y: i32, radius: u32, wait_for_input: bool, mode: &str) }
    bind_python! { bpy.ops.node.select_grouped() => pub fn select_grouped(py: Python, extend: bool, r#type: &str) }
    bind_python! { bpy.ops.node.select_linked_from() => pub fn select_linked_from(py: Python) }
    bind_python! { bpy.ops.node.select_linked_to() => pub fn select_linked_to(py: Python) }
    bind_python! { bpy.ops.node.select_same_type_step() => pub fn select_same_type_step(py: Python, prev: bool) }
    bind_python! { bpy.ops.node.shader_script_update() => pub fn shader_script_update(py: Python) }
    bind_python! { bpy.ops.node.simulation_zone_item_add() => pub fn simulation_zone_item_add(py: Python) }
    bind_python! { bpy.ops.node.simulation_zone_item_move() => pub fn simulation_zone_item_move(py: Python, direction: &str) }
    bind_python! { bpy.ops.node.simulation_zone_item_remove() => pub fn simulation_zone_item_remove(py: Python) }
    bind_python! { bpy.ops.node.switch_view_update() => pub fn switch_view_update(py: Python) }
    bind_python! { bpy.ops.node.tree_path_parent() => pub fn tree_path_parent(py: Python) }
    bind_python! { bpy.ops.node.tree_socket_add() => pub fn tree_socket_add(py: Python, in_out: &str) }
    bind_python! { bpy.ops.node.tree_socket_change_subtype() => pub fn tree_socket_change_subtype(py: Python, socket_subtype: &str) }
    bind_python! { bpy.ops.node.tree_socket_change_type() => pub fn tree_socket_change_type(py: Python, in_out: &str, socket_type: &str) }
    bind_python! { bpy.ops.node.tree_socket_move() => pub fn tree_socket_move(py: Python, direction: &str, in_out: &str) }
    bind_python! { bpy.ops.node.tree_socket_remove() => pub fn tree_socket_remove(py: Python, in_out: &str) }
    bind_python! { bpy.ops.node.view_all() => pub fn view_all(py: Python) }
    bind_python! { bpy.ops.node.view_selected() => pub fn view_selected(py: Python) }
    bind_python! { bpy.ops.node.viewer_border() => pub fn viewer_border(py: Python, xmin: i32, xmax: i32, ymin: i32, ymax: i32, wait_for_input: bool) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.object.html>
pub mod object {
    use super::{
        bind_python, Alignment, LightType, ModifierType, ObjectMode, ObjectType, OriginCenter,
        OriginType, Path,
    };

    bind_python! { bpy.ops.object.add() => pub fn add(py: Python, radius: f32, r#type: ObjectType, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.add_named() => pub fn add_named(py: Python, linked: bool, name: &str, session_uuid: i32, matrix: ([f32; 4], [f32; 4], [f32; 4], [f32; 4]), drop_x: i32, drop_y: i32) }
    bind_python! { bpy.ops.object.anim_transforms_to_deltas() => pub fn anim_transforms_to_deltas(py: Python) }
    bind_python! { bpy.ops.object.armature_add() => pub fn armature_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.assign_property_defaults() => pub fn assign_property_defaults(py: Python, process_data: bool, process_bones: bool) }
    bind_python! { bpy.ops.object.bake_image() => pub fn bake_image(py: Python) }
    bind_python! { bpy.ops.object.camera_add() => pub fn camera_add(py: Python, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.clear_override_library() => pub fn clear_override_library(py: Python) }
    bind_python! { bpy.ops.object.collection_add() => pub fn collection_add(py: Python) }
    bind_python! { bpy.ops.object.collection_external_asset_drop() => pub fn collection_external_asset_drop(py: Python, session_uuid: i32, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3], use_instance: bool, drop_x: i32, drop_y: i32, collection: &str) }
    bind_python! { bpy.ops.object.collection_instance_add() => pub fn collection_instance_add(py: Python, name: &str, collection: &str, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3], session_uuid: i32, drop_x: i32, drop_y: i32) }
    bind_python! { bpy.ops.object.collection_link() => pub fn collection_link(py: Python, collection: &str) }
    bind_python! { bpy.ops.object.collection_objects_select() => pub fn collection_objects_select(py: Python) }
    bind_python! { bpy.ops.object.collection_remove() => pub fn collection_remove(py: Python) }
    bind_python! { bpy.ops.object.collection_unlink() => pub fn collection_unlink(py: Python) }
    bind_python! { bpy.ops.object.constraint_add() => pub fn constraint_add(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.constraint_add_with_targets() => pub fn constraint_add_with_targets(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.constraints_clear() => pub fn constraints_clear(py: Python) }
    bind_python! { bpy.ops.object.constraints_copy() => pub fn constraints_copy(py: Python) }
    bind_python! { bpy.ops.object.convert() => pub fn convert(py: Python, target: &str, keep_original: bool, merge_customdata: bool, angle: f32, thickness: u32, seams: bool, faces: bool, offset: f32) }
    bind_python! { bpy.ops.object.correctivesmooth_bind() => pub fn correctivesmooth_bind(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.curves_empty_hair_add() => pub fn curves_empty_hair_add(py: Python, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.curves_random_add() => pub fn curves_random_add(py: Python, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.data_instance_add() => pub fn data_instance_add(py: Python, name: &str, session_uuid: i32, r#type: &str, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3], drop_x: i32, drop_y: i32) }
    bind_python! { bpy.ops.object.data_transfer() => pub fn data_transfer(py: Python, use_reverse_transfer: bool, use_freeze: bool, data_type: &str, use_create: bool, vert_mapping: &str, edge_mapping: &str, loop_mapping: &str, poly_mapping: &str, use_auto_transform: bool, use_object_transform: bool, use_max_distance: bool, max_distance: f32, ray_radius: f32, islands_precision: f32, layers_select_src: &str, layers_select_dst: &str, mix_mode: &str, mix_factor: f32) }
    bind_python! { bpy.ops.object.datalayout_transfer() => pub fn datalayout_transfer(py: Python, modifier: &str, data_type: &str, use_delete: bool, layers_select_src: &str, layers_select_dst: &str) }
    bind_python! { bpy.ops.object.delete() => pub fn delete(py: Python, use_global: bool, confirm: bool) }
    bind_python! { bpy.ops.object.drop_geometry_nodes() => pub fn drop_geometry_nodes(py: Python, session_uuid: i32) }
    bind_python! { bpy.ops.object.drop_named_image() => pub fn drop_named_image(py: Python, filepath: &Path, relative_path: bool, name: &str, session_uuid: i32, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.drop_named_material() => pub fn drop_named_material(py: Python, name: &str, session_uuid: i32) }
    bind_python! { bpy.ops.object.duplicate() => pub fn duplicate(py: Python, linked: bool, mode: &str) }
    bind_python! { bpy.ops.object.duplicates_make_real() => pub fn duplicates_make_real(py: Python, use_base_parent: bool, use_hierarchy: bool) }
    bind_python! { bpy.ops.object.editmode_toggle() => pub fn editmode_toggle(py: Python) }
    bind_python! { bpy.ops.object.effector_add() => pub fn effector_add(py: Python, r#type: &str, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.empty_add() => pub fn empty_add(py: Python, r#type: &str, radius: f32, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.explode_refresh() => pub fn explode_refresh(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.face_map_add() => pub fn face_map_add(py: Python) }
    bind_python! { bpy.ops.object.face_map_assign() => pub fn face_map_assign(py: Python) }
    bind_python! { bpy.ops.object.face_map_deselect() => pub fn face_map_deselect(py: Python) }
    bind_python! { bpy.ops.object.face_map_move() => pub fn face_map_move(py: Python, direction: &str) }
    bind_python! { bpy.ops.object.face_map_remove() => pub fn face_map_remove(py: Python) }
    bind_python! { bpy.ops.object.face_map_remove_from() => pub fn face_map_remove_from(py: Python) }
    bind_python! { bpy.ops.object.face_map_select() => pub fn face_map_select(py: Python) }
    bind_python! { bpy.ops.object.forcefield_toggle() => pub fn forcefield_toggle(py: Python) }
    bind_python! { bpy.ops.object.geometry_node_tree_copy_assign() => pub fn geometry_node_tree_copy_assign(py: Python) }
    bind_python! { bpy.ops.object.geometry_nodes_input_attribute_toggle() => pub fn geometry_nodes_input_attribute_toggle(py: Python, prop_path: &str, modifier_name: &str) }
    bind_python! { bpy.ops.object.geometry_nodes_move_to_nodes() => pub fn geometry_nodes_move_to_nodes(py: Python) }
    bind_python! { bpy.ops.object.gpencil_add() => pub fn gpencil_add(py: Python, radius: f32, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3], r#type: &str, use_in_front: bool, stroke_depth_offset: f32, use_lights: bool, stroke_depth_order: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_add() => pub fn gpencil_modifier_add(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_apply() => pub fn gpencil_modifier_apply(py: Python, apply_as: &str, modifier: &str, report: bool) }
    bind_python! { bpy.ops.object.gpencil_modifier_copy() => pub fn gpencil_modifier_copy(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_copy_to_selected() => pub fn gpencil_modifier_copy_to_selected(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_move_down() => pub fn gpencil_modifier_move_down(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_move_to_index() => pub fn gpencil_modifier_move_to_index(py: Python, modifier: &str, index: i32) }
    bind_python! { bpy.ops.object.gpencil_modifier_move_up() => pub fn gpencil_modifier_move_up(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.gpencil_modifier_remove() => pub fn gpencil_modifier_remove(py: Python, modifier: &str, report: bool) }
    bind_python! { bpy.ops.object.hide_collection() => pub fn hide_collection(py: Python, collection_index: i32, toggle: bool, extend: bool) }
    bind_python! { bpy.ops.object.hide_render_clear_all() => pub fn hide_render_clear_all(py: Python) }
    bind_python! { bpy.ops.object.hide_view_clear() => pub fn hide_view_clear(py: Python, select: bool) }
    bind_python! { bpy.ops.object.hide_view_set() => pub fn hide_view_set(py: Python, unselected: bool) }
    bind_python! { bpy.ops.object.hook_add_newob() => pub fn hook_add_newob(py: Python) }
    bind_python! { bpy.ops.object.hook_add_selob() => pub fn hook_add_selob(py: Python, use_bone: bool) }
    bind_python! { bpy.ops.object.hook_assign() => pub fn hook_assign(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.hook_recenter() => pub fn hook_recenter(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.hook_remove() => pub fn hook_remove(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.hook_reset() => pub fn hook_reset(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.hook_select() => pub fn hook_select(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.instance_offset_from_cursor() => pub fn instance_offset_from_cursor(py: Python) }
    bind_python! { bpy.ops.object.instance_offset_from_object() => pub fn instance_offset_from_object(py: Python) }
    bind_python! { bpy.ops.object.instance_offset_to_cursor() => pub fn instance_offset_to_cursor(py: Python) }
    bind_python! { bpy.ops.object.isolate_type_render() => pub fn isolate_type_render(py: Python) }
    bind_python! { bpy.ops.object.join() => pub fn join(py: Python) }
    bind_python! { bpy.ops.object.join_shapes() => pub fn join_shapes(py: Python) }
    bind_python! { bpy.ops.object.join_uvs() => pub fn join_uvs(py: Python) }
    bind_python! { bpy.ops.object.laplaciandeform_bind() => pub fn laplaciandeform_bind(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.light_add() => pub fn light_add(py: Python, r#type: LightType, radius: f32, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.lightprobe_add() => pub fn lightprobe_add(py: Python, r#type: &str, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.lightprobe_cache_bake() => pub fn lightprobe_cache_bake(py: Python, delay: i32, subset: &str) }
    bind_python! { bpy.ops.object.lightprobe_cache_free() => pub fn lightprobe_cache_free(py: Python) }
    bind_python! { bpy.ops.object.lineart_bake_strokes() => pub fn lineart_bake_strokes(py: Python) }
    bind_python! { bpy.ops.object.lineart_bake_strokes_all() => pub fn lineart_bake_strokes_all(py: Python) }
    bind_python! { bpy.ops.object.lineart_clear() => pub fn lineart_clear(py: Python) }
    bind_python! { bpy.ops.object.lineart_clear_all() => pub fn lineart_clear_all(py: Python) }
    bind_python! { bpy.ops.object.link_to_collection() => pub fn link_to_collection(py: Python, collection_index: i32, is_new: bool, new_collection_name: &str) }
    bind_python! { bpy.ops.object.load_background_image() => pub fn load_background_image(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.object.load_reference_image() => pub fn load_reference_image(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.object.location_clear() => pub fn location_clear(py: Python, clear_delta: bool) }
    bind_python! { bpy.ops.object.make_dupli_face() => pub fn make_dupli_face(py: Python) }
    bind_python! { bpy.ops.object.make_links_data() => pub fn make_links_data(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.make_links_scene() => pub fn make_links_scene(py: Python, scene: &str) }
    bind_python! { bpy.ops.object.make_local() => pub fn make_local(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.make_override_library() => pub fn make_override_library(py: Python, collection: i32) }
    bind_python! { bpy.ops.object.make_single_user() => pub fn make_single_user(py: Python, r#type: &str, object: bool, obdata: bool, material: bool, animation: bool, obdata_animation: bool) }
    bind_python! { bpy.ops.object.material_slot_add() => pub fn material_slot_add(py: Python) }
    bind_python! { bpy.ops.object.material_slot_assign() => pub fn material_slot_assign(py: Python) }
    bind_python! { bpy.ops.object.material_slot_copy() => pub fn material_slot_copy(py: Python) }
    bind_python! { bpy.ops.object.material_slot_deselect() => pub fn material_slot_deselect(py: Python) }
    bind_python! { bpy.ops.object.material_slot_move() => pub fn material_slot_move(py: Python, direction: &str) }
    bind_python! { bpy.ops.object.material_slot_remove() => pub fn material_slot_remove(py: Python) }
    bind_python! { bpy.ops.object.material_slot_remove_unused() => pub fn material_slot_remove_unused(py: Python) }
    bind_python! { bpy.ops.object.material_slot_select() => pub fn material_slot_select(py: Python) }
    bind_python! { bpy.ops.object.meshdeform_bind() => pub fn meshdeform_bind(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.metaball_add() => pub fn metaball_add(py: Python, r#type: &str, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.mode_set() => pub fn mode_set(py: Python, mode: ObjectMode, toggle: bool) }
    bind_python! { bpy.ops.object.modifier_add() => pub fn modifier_add(py: Python, r#type: ModifierType) }
    bind_python! { bpy.ops.object.modifier_apply() => pub fn modifier_apply(py: Python, modifier: &str, report: bool, merge_customdata: bool, single_user: bool) }
    bind_python! { bpy.ops.object.modifier_apply_as_shapekey() => pub fn modifier_apply_as_shapekey(py: Python, keep_modifier: bool, modifier: &str, report: bool) }
    bind_python! { bpy.ops.object.modifier_convert() => pub fn modifier_convert(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.modifier_copy() => pub fn modifier_copy(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.modifier_copy_to_selected() => pub fn modifier_copy_to_selected(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.modifier_move_down() => pub fn modifier_move_down(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.modifier_move_to_index() => pub fn modifier_move_to_index(py: Python, modifier: &str, index: i32) }
    bind_python! { bpy.ops.object.modifier_move_up() => pub fn modifier_move_up(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.modifier_remove() => pub fn modifier_remove(py: Python, modifier: &str, report: bool) }
    bind_python! { bpy.ops.object.modifier_set_active() => pub fn modifier_set_active(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.move_to_collection() => pub fn move_to_collection(py: Python, collection_index: i32, is_new: bool, new_collection_name: &str) }
    bind_python! { bpy.ops.object.multires_base_apply() => pub fn multires_base_apply(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.multires_external_pack() => pub fn multires_external_pack(py: Python) }
    bind_python! { bpy.ops.object.multires_external_save() => pub fn multires_external_save(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.object.multires_higher_levels_delete() => pub fn multires_higher_levels_delete(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.multires_rebuild_subdiv() => pub fn multires_rebuild_subdiv(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.multires_reshape() => pub fn multires_reshape(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.multires_subdivide() => pub fn multires_subdivide(py: Python, modifier: &str, mode: &str) }
    bind_python! { bpy.ops.object.multires_unsubdivide() => pub fn multires_unsubdivide(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.ocean_bake() => pub fn ocean_bake(py: Python, modifier: &str, free: bool) }
    bind_python! { bpy.ops.object.origin_clear() => pub fn origin_clear(py: Python) }
    bind_python! { bpy.ops.object.origin_set() => pub fn origin_set(py: Python, r#type: OriginType, center: OriginCenter) }
    bind_python! { bpy.ops.object.parent_clear() => pub fn parent_clear(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.parent_inverse_apply() => pub fn parent_inverse_apply(py: Python) }
    bind_python! { bpy.ops.object.parent_no_inverse_set() => pub fn parent_no_inverse_set(py: Python, confirm: bool, keep_transform: bool) }
    bind_python! { bpy.ops.object.parent_set() => pub fn parent_set(py: Python, r#type: &str, xmirror: bool, keep_transform: bool) }
    bind_python! { bpy.ops.object.particle_system_add() => pub fn particle_system_add(py: Python) }
    bind_python! { bpy.ops.object.particle_system_remove() => pub fn particle_system_remove(py: Python) }
    bind_python! { bpy.ops.object.paths_calculate() => pub fn paths_calculate(py: Python, display_type: &str, range: &str) }
    bind_python! { bpy.ops.object.paths_clear() => pub fn paths_clear(py: Python, only_selected: bool) }
    bind_python! { bpy.ops.object.paths_update() => pub fn paths_update(py: Python) }
    bind_python! { bpy.ops.object.paths_update_visible() => pub fn paths_update_visible(py: Python) }
    bind_python! { bpy.ops.object.pointcloud_add() => pub fn pointcloud_add(py: Python, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.posemode_toggle() => pub fn posemode_toggle(py: Python) }
    bind_python! { bpy.ops.object.quadriflow_remesh() => pub fn quadriflow_remesh(py: Python, use_mesh_symmetry: bool, use_preserve_sharp: bool, use_preserve_boundary: bool, preserve_paint_mask: bool, smooth_normals: bool, mode: &str, target_ratio: f32, target_edge_length: f32, target_faces: u32, mesh_area: f32, seed: u32) }
    bind_python! { bpy.ops.object.quick_explode() => pub fn quick_explode(py: Python, style: &str, amount: u32, frame_duration: u32, frame_start: u32, frame_end: u32, velocity: f32, fade: bool) }
    bind_python! { bpy.ops.object.quick_fur() => pub fn quick_fur(py: Python, density: &str, length: f32, radius: f32, view_percentage: f32, apply_hair_guides: bool, use_noise: bool, use_frizz: bool) }
    bind_python! { bpy.ops.object.quick_liquid() => pub fn quick_liquid(py: Python, show_flows: bool) }
    bind_python! { bpy.ops.object.quick_smoke() => pub fn quick_smoke(py: Python, style: &str, show_flows: bool) }
    bind_python! { bpy.ops.object.randomize_transform() => pub fn randomize_transform(py: Python, random_seed: u32, use_delta: bool, use_loc: bool, loc: [f32; 3], use_rot: bool, rot: [f32; 3], use_scale: bool, scale_even: bool, scale: [f32; 3]) }
    bind_python! { bpy.ops.object.reset_override_library() => pub fn reset_override_library(py: Python) }
    bind_python! { bpy.ops.object.rotation_clear() => pub fn rotation_clear(py: Python, clear_delta: bool) }
    bind_python! { bpy.ops.object.scale_clear() => pub fn scale_clear(py: Python, clear_delta: bool) }
    bind_python! { bpy.ops.object.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.object.select_by_type() => pub fn select_by_type(py: Python, extend: bool, r#type: &str) }
    bind_python! { bpy.ops.object.select_camera() => pub fn select_camera(py: Python, extend: bool) }
    bind_python! { bpy.ops.object.select_grouped() => pub fn select_grouped(py: Python, extend: bool, r#type: &str) }
    bind_python! { bpy.ops.object.select_hierarchy() => pub fn select_hierarchy(py: Python, direction: &str, extend: bool) }
    bind_python! { bpy.ops.object.select_less() => pub fn select_less(py: Python) }
    bind_python! { bpy.ops.object.select_linked() => pub fn select_linked(py: Python, extend: bool, r#type: &str) }
    bind_python! { bpy.ops.object.select_mirror() => pub fn select_mirror(py: Python, extend: bool) }
    bind_python! { bpy.ops.object.select_more() => pub fn select_more(py: Python) }
    bind_python! { bpy.ops.object.select_pattern() => pub fn select_pattern(py: Python, pattern: &str, case_sensitive: bool, extend: bool) }
    bind_python! { bpy.ops.object.select_random() => pub fn select_random(py: Python, ratio: f32, seed: u32, action: &str) }
    bind_python! { bpy.ops.object.select_same_collection() => pub fn select_same_collection(py: Python, collection: &str) }
    bind_python! { bpy.ops.object.shade_flat() => pub fn shade_flat(py: Python) }
    bind_python! { bpy.ops.object.shade_smooth() => pub fn shade_smooth(py: Python, use_auto_smooth: bool, auto_smooth_angle: f32) }
    bind_python! { bpy.ops.object.shaderfx_add() => pub fn shaderfx_add(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.shaderfx_copy() => pub fn shaderfx_copy(py: Python, shaderfx: &str) }
    bind_python! { bpy.ops.object.shaderfx_move_down() => pub fn shaderfx_move_down(py: Python, shaderfx: &str) }
    bind_python! { bpy.ops.object.shaderfx_move_to_index() => pub fn shaderfx_move_to_index(py: Python, shaderfx: &str, index: i32) }
    bind_python! { bpy.ops.object.shaderfx_move_up() => pub fn shaderfx_move_up(py: Python, shaderfx: &str) }
    bind_python! { bpy.ops.object.shaderfx_remove() => pub fn shaderfx_remove(py: Python, shaderfx: &str, report: bool) }
    bind_python! { bpy.ops.object.shape_key_add() => pub fn shape_key_add(py: Python, from_mix: bool) }
    bind_python! { bpy.ops.object.shape_key_clear() => pub fn shape_key_clear(py: Python) }
    bind_python! { bpy.ops.object.shape_key_mirror() => pub fn shape_key_mirror(py: Python, use_topology: bool) }
    bind_python! { bpy.ops.object.shape_key_move() => pub fn shape_key_move(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.shape_key_remove() => pub fn shape_key_remove(py: Python, all: bool, apply_mix: bool) }
    bind_python! { bpy.ops.object.shape_key_retime() => pub fn shape_key_retime(py: Python) }
    bind_python! { bpy.ops.object.shape_key_transfer() => pub fn shape_key_transfer(py: Python, mode: &str, use_clamp: bool) }
    bind_python! { bpy.ops.object.simulation_nodes_cache_bake() => pub fn simulation_nodes_cache_bake(py: Python, selected: bool) }
    bind_python! { bpy.ops.object.simulation_nodes_cache_calculate_to_frame() => pub fn simulation_nodes_cache_calculate_to_frame(py: Python, selected: bool) }
    bind_python! { bpy.ops.object.simulation_nodes_cache_delete() => pub fn simulation_nodes_cache_delete(py: Python, selected: bool) }
    bind_python! { bpy.ops.object.skin_armature_create() => pub fn skin_armature_create(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.skin_loose_mark_clear() => pub fn skin_loose_mark_clear(py: Python, action: &str) }
    bind_python! { bpy.ops.object.skin_radii_equalize() => pub fn skin_radii_equalize(py: Python) }
    bind_python! { bpy.ops.object.skin_root_mark() => pub fn skin_root_mark(py: Python) }
    bind_python! { bpy.ops.object.speaker_add() => pub fn speaker_add(py: Python, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.subdivision_set() => pub fn subdivision_set(py: Python, level: u32, relative: bool) }
    bind_python! { bpy.ops.object.surfacedeform_bind() => pub fn surfacedeform_bind(py: Python, modifier: &str) }
    bind_python! { bpy.ops.object.text_add() => pub fn text_add(py: Python, radius: f32, enter_editmode: bool, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.track_clear() => pub fn track_clear(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.track_set() => pub fn track_set(py: Python, r#type: &str) }
    bind_python! { bpy.ops.object.transfer_mode() => pub fn transfer_mode(py: Python, use_flash_on_transfer: bool) }
    bind_python! { bpy.ops.object.transform_apply() => pub fn transform_apply(py: Python, location: bool, rotation: bool, scale: bool, properties: bool, isolate_users: bool) }
    bind_python! { bpy.ops.object.transform_axis_target() => pub fn transform_axis_target(py: Python) }
    bind_python! { bpy.ops.object.transform_to_mouse() => pub fn transform_to_mouse(py: Python, name: &str, session_uuid: i32, matrix: ([f32; 4], [f32; 4], [f32; 4], [f32; 4]), drop_x: i32, drop_y: i32) }
    bind_python! { bpy.ops.object.transforms_to_deltas() => pub fn transforms_to_deltas(py: Python, mode: &str, reset_values: bool) }
    bind_python! { bpy.ops.object.unlink_data() => pub fn unlink_data(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_add() => pub fn vertex_group_add(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_assign() => pub fn vertex_group_assign(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_assign_new() => pub fn vertex_group_assign_new(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_clean() => pub fn vertex_group_clean(py: Python, group_select_mode: &str, limit: f32, keep_single: bool) }
    bind_python! { bpy.ops.object.vertex_group_copy() => pub fn vertex_group_copy(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_copy_to_selected() => pub fn vertex_group_copy_to_selected(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_deselect() => pub fn vertex_group_deselect(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_invert() => pub fn vertex_group_invert(py: Python, group_select_mode: &str, auto_assign: bool, auto_remove: bool) }
    bind_python! { bpy.ops.object.vertex_group_levels() => pub fn vertex_group_levels(py: Python, group_select_mode: &str, offset: f32, gain: f32) }
    bind_python! { bpy.ops.object.vertex_group_limit_total() => pub fn vertex_group_limit_total(py: Python, group_select_mode: &str, limit: u32) }
    bind_python! { bpy.ops.object.vertex_group_lock() => pub fn vertex_group_lock(py: Python, action: &str, mask: &str) }
    bind_python! { bpy.ops.object.vertex_group_mirror() => pub fn vertex_group_mirror(py: Python, mirror_weights: bool, flip_group_names: bool, all_groups: bool, use_topology: bool) }
    bind_python! { bpy.ops.object.vertex_group_move() => pub fn vertex_group_move(py: Python, direction: &str) }
    bind_python! { bpy.ops.object.vertex_group_normalize() => pub fn vertex_group_normalize(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_normalize_all() => pub fn vertex_group_normalize_all(py: Python, group_select_mode: &str, lock_active: bool) }
    bind_python! { bpy.ops.object.vertex_group_quantize() => pub fn vertex_group_quantize(py: Python, group_select_mode: &str, steps: u32) }
    bind_python! { bpy.ops.object.vertex_group_remove() => pub fn vertex_group_remove(py: Python, all: bool, all_unlocked: bool) }
    bind_python! { bpy.ops.object.vertex_group_remove_from() => pub fn vertex_group_remove_from(py: Python, use_all_groups: bool, use_all_verts: bool) }
    bind_python! { bpy.ops.object.vertex_group_select() => pub fn vertex_group_select(py: Python) }
    bind_python! { bpy.ops.object.vertex_group_set_active() => pub fn vertex_group_set_active(py: Python, group: &str) }
    bind_python! { bpy.ops.object.vertex_group_smooth() => pub fn vertex_group_smooth(py: Python, group_select_mode: &str, factor: f32, repeat: u32, expand: f32) }
    bind_python! { bpy.ops.object.vertex_group_sort() => pub fn vertex_group_sort(py: Python, sort_type: &str) }
    bind_python! { bpy.ops.object.vertex_parent_set() => pub fn vertex_parent_set(py: Python, confirm: bool) }
    bind_python! { bpy.ops.object.vertex_weight_copy() => pub fn vertex_weight_copy(py: Python) }
    bind_python! { bpy.ops.object.vertex_weight_delete() => pub fn vertex_weight_delete(py: Python, weight_group: i32) }
    bind_python! { bpy.ops.object.vertex_weight_normalize_active_vertex() => pub fn vertex_weight_normalize_active_vertex(py: Python) }
    bind_python! { bpy.ops.object.vertex_weight_paste() => pub fn vertex_weight_paste(py: Python, weight_group: i32) }
    bind_python! { bpy.ops.object.vertex_weight_set_active() => pub fn vertex_weight_set_active(py: Python, weight_group: i32) }
    bind_python! { bpy.ops.object.visual_transform_apply() => pub fn visual_transform_apply(py: Python) }
    bind_python! { bpy.ops.object.volume_add() => pub fn volume_add(py: Python, align: Alignment, location: [f32; 3], rotation: [f32; 3], scale: [f32; 3]) }
    bind_python! { bpy.ops.object.voxel_remesh() => pub fn voxel_remesh(py: Python) }
    bind_python! { bpy.ops.object.voxel_size_edit() => pub fn voxel_size_edit(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.outliner.html>
// pub mod outliner {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.paint.html>
// pub mod paint {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.paintcurve.html>
// pub mod paintcurve {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.palette.html>
// pub mod palette {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.particle.html>
// pub mod particle {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.pose.html>
// pub mod pose {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.poselib.html>
// pub mod poselib {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.preferences.html>
/// Note: This module is incomplete.
pub mod preferences {
    use super::{bind_python, Path};

    bind_python! { bpy.ops.preferences.addon_disable() => pub fn addon_disable(py: Python, module: &str) }
    bind_python! { bpy.ops.preferences.addon_enable() => pub fn addon_enable(py: Python, module: &str) }
    bind_python! { bpy.ops.preferences.addon_expand() => pub fn addon_expand(py: Python, module: &str) }
    bind_python! { bpy.ops.preferences.addon_install() => pub fn addon_install(py: Python, filepath: &Path, overwrite: bool) }
    bind_python! { bpy.ops.preferences.addon_refresh() => pub fn addon_refresh(py: Python) }
    bind_python! { bpy.ops.preferences.addon_remove() => pub fn addon_remove(py: Python, module: &str) }
    bind_python! { bpy.ops.preferences.addon_show() => pub fn addon_show(py: Python, module: &str) }
    bind_python! { bpy.ops.preferences.script_directory_add() => pub fn script_directory_add(py: Python, directory: &Path) }
    bind_python! { bpy.ops.preferences.script_directory_remove() => pub fn script_directory_remove(py: Python, index: u32) }
    bind_python! { bpy.ops.preferences.asset_library_add() => pub fn asset_library_add(py: Python, directory: &Path) }
    bind_python! { bpy.ops.preferences.asset_library_remove() => pub fn asset_library_remove(py: Python, index: u32) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.ptcache.html>
// pub mod ptcache {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.render.html>
/// Note: This module is incomplete.
pub mod render {
    use super::bind_python;

    bind_python! { bpy.ops.render.opengl() => pub fn opengl(py: Python, animation: bool, render_keyed_only: bool, sequencer: bool, write_still: bool, view_context: bool) }
    bind_python! { bpy.ops.render.render() => pub fn render(py: Python, animation: bool, write_still: bool, use_viewport: bool, layer: &str, scene: &str) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.rigidbody.html>
// pub mod rigidbody {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.scene.html>
/// Note: This module is incomplete.
pub mod scene {
    use super::bind_python;

    bind_python! { bpy.ops.scene.delete() => pub fn delete(py: Python) }
    bind_python! { bpy.ops.scene.new() => pub fn new(py: Python, r#type: &str) }
    bind_python! { bpy.ops.scene.render_view_add() => pub fn render_view_add(py: Python) }
    bind_python! { bpy.ops.scene.render_view_remove() => pub fn render_view_remove(py: Python) }
    bind_python! { bpy.ops.scene.view_layer_add() => pub fn view_layer_add(py: Python, r#type: &str) }
    bind_python! { bpy.ops.scene.view_layer_remove() => pub fn view_layer_remove(py: Python) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.screen.html>
/// Note: This module is incomplete.
pub mod screen {
    use super::{bind_python, Path};

    bind_python! {bpy.ops.screen.screenshot() => pub fn screenshot(py: Python, filepath: &Path) }
    bind_python! {bpy.ops.screen.screenshot_area() => pub fn screenshot_area(py: Python, filepath: &Path) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.script.html>
pub mod script {
    use super::{bind_python, Path};

    bind_python! { bpy.ops.script.execute_preset() => pub fn execute_preset(py: Python, filepath: &Path, menu_idname: &str) }
    bind_python! { bpy.ops.script.python_file_run() => pub fn python_file_run(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.script.reload() => pub fn reload(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.sculpt.html>
// pub mod sculpt {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.sculpt_curves.html>
// pub mod sculpt_curves {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.sequencer.html>
// pub mod sequencer {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.sound.html>
// pub mod sound {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.spreadsheet.html>
// pub mod spreadsheet {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.surface.html>
// pub mod surface {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.text.html>
// pub mod text {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.texture.html>
pub mod texture {
    use super::bind_python;

    bind_python! { bpy.ops.texture.new() => pub fn new(py: Python) }
    bind_python! { bpy.ops.texture.slot_copy() => pub fn slot_copy(py: Python) }
    bind_python! { bpy.ops.texture.slot_paste() => pub fn slot_paste(py: Python) }
    bind_python! { bpy.ops.texture.slot_move() => pub fn slot_move(py: Python, r#type: &str) }
}

/// <https://docs.blender.org/api/latest/bpy.ops.transform.html>
pub mod transform {
    use super::{bind_python, AxisXYZ, SnapElement, TransformOrientation};

    bind_python! { bpy.ops.transform.bbone_resize() => pub fn bbone_resize(py: Python, value: [f32; 3], orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.bend() => pub fn bend(py: Python, value: f32, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.create_orientation() => pub fn create_orientation(py: Python, name: &str, use_view: bool, r#use: bool, overwrite: bool) }
    bind_python! { bpy.ops.transform.delete_orientation() => pub fn delete_orientation(py: Python) }
    bind_python! { bpy.ops.transform.edge_bevelweight() => pub fn edge_bevelweight(py: Python, value: f32, snap: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.edge_crease() => pub fn edge_crease(py: Python, value: f32, snap: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.edge_slide() => pub fn edge_slide(py: Python, value: f32, single_side: bool, use_even: bool, flipped: bool, use_clamp: bool, mirror: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], correct_uv: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.from_gizmo() => pub fn from_gizmo(py: Python) }
    bind_python! { bpy.ops.transform.mirror() => pub fn mirror(py: Python, orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.push_pull() => pub fn push_pull(py: Python, value: f32, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.resize() => pub fn resize(py: Python, value: [f32; 3], mouse_dir_constraint: [f32; 3], orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], gpencil_strokes: bool, texture_space: bool, remove_on_cancel: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.rotate() => pub fn rotate(py: Python, value: f32, orient_axis: AxisXYZ, orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.rotate_normal() => pub fn rotate_normal(py: Python, value: f32, orient_axis: AxisXYZ, orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.select_orientation() => pub fn select_orientation(py: Python, orientation: &str) }
    bind_python! { bpy.ops.transform.seq_slide() => pub fn seq_slide(py: Python, value: [f32; 2], snap: bool, view2d_edge_pan: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.shear() => pub fn shear(py: Python, value: f32, orient_axis: AxisXYZ, orient_axis_ortho: &str, orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, gpencil_strokes: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.shrink_fatten() => pub fn shrink_fatten(py: Python, value: f32, use_even_offset: bool, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.skin_resize() => pub fn skin_resize(py: Python, value: [f32; 3], orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.tilt() => pub fn tilt(py: Python, value: f32, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.tosphere() => pub fn tosphere(py: Python, value: f32, mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.trackball() => pub fn trackball(py: Python, value: [f32; 2], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.transform() => pub fn transform(py: Python, mode: &str, value: [f32; 4], orient_axis: AxisXYZ, orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], snap_align: bool, snap_normal: [f32; 3], gpencil_strokes: bool, center_override: [f32; 3], release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.translate() => pub fn translate(py: Python, value: [f32; 3], orient_type: TransformOrientation, orient_matrix: [[f32; 3]; 3], orient_matrix_type: TransformOrientation, constraint_axis: [bool; 3], mirror: bool, use_proportional_edit: bool, proportional_edit_falloff: &str, proportional_size: f32, use_proportional_connected: bool, use_proportional_projected: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], snap_align: bool, snap_normal: [f32; 3], gpencil_strokes: bool, cursor_transform: bool, texture_space: bool, remove_on_cancel: bool, view2d_edge_pan: bool, release_confirm: bool, use_accurate: bool, use_automerge_and_split: bool) }
    bind_python! { bpy.ops.transform.vert_crease() => pub fn vert_crease(py: Python, value: f32, snap: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.vert_slide() => pub fn vert_slide(py: Python, value: f32, use_even: bool, flipped: bool, use_clamp: bool, mirror: bool, snap: bool, snap_elements: SnapElement, use_snap_project: bool, snap_target: &str, use_snap_self: bool, use_snap_edit: bool, use_snap_nonedit: bool, use_snap_selectable: bool, snap_point: [f32; 3], correct_uv: bool, release_confirm: bool, use_accurate: bool) }
    bind_python! { bpy.ops.transform.vertex_random() => pub fn vertex_random(py: Python, offset: f32, uniform: f32, normal: f32, seed: u32, wait_for_input: bool) }
    bind_python! { bpy.ops.transform.vertex_warp() => pub fn vertex_warp(py: Python, warp_angle: f32, offset_angle: f32, min: f32, max: f32, viewmat: ([f32; 4], [f32; 4], [f32; 4], [f32; 4]), center: [f32; 3]) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.ui.html>
// pub mod ui {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.uilist.html>
// pub mod uilist {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.uv.html>
pub mod uv {
    use super::{bind_python, Alignment, Path};

    bind_python! { bpy.ops.uv.align() => pub fn align(py: Python, axis: &str) }
    bind_python! { bpy.ops.uv.align_rotation() => pub fn align_rotation(py: Python, method: &str, axis: &str) }
    bind_python! { bpy.ops.uv.average_islands_scale() => pub fn average_islands_scale(py: Python, scale_uv: bool, shear: bool) }
    bind_python! { bpy.ops.uv.copy() => pub fn copy(py: Python) }
    bind_python! { bpy.ops.uv.cube_project() => pub fn cube_project(py: Python, cube_size: f32, correct_aspect: bool, clip_to_bounds: bool, scale_to_bounds: bool) }
    bind_python! { bpy.ops.uv.cursor_set() => pub fn cursor_set(py: Python, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.cylinder_project() => pub fn cylinder_project(py: Python, direction: &str, align: Alignment, pole: &str, seam: bool, radius: f32, correct_aspect: bool, clip_to_bounds: bool, scale_to_bounds: bool) }
    bind_python! { bpy.ops.uv.export_layout() => pub fn export_layout(py: Python, filepath: &Path, modified: bool, mode: &str, size: [u32; 2], opacity: f32) }
    bind_python! { bpy.ops.uv.follow_active_quads() => pub fn follow_active_quads(py: Python, mode: &str) }
    bind_python! { bpy.ops.uv.hide() => pub fn hide(py: Python, unselected: bool) }
    bind_python! {
        #[allow(non_snake_case)]
        bpy.ops.uv.lightmap_pack() => pub fn lightmap_pack(py: Python, PREF_CONTEXT: &str, PREF_PACK_IN_ONE: bool, PREF_NEW_UVLAYER: bool, PREF_BOX_DIV: u32, PREF_MARGIN_DIV: f32)
    }
    bind_python! { bpy.ops.uv.mark_seam() => pub fn mark_seam(py: Python, clear: bool) }
    bind_python! { bpy.ops.uv.minimize_stretch() => pub fn minimize_stretch(py: Python, fill_holes: bool, blend: f32, iterations: u32) }
    bind_python! { bpy.ops.uv.pack_islands() => pub fn pack_islands(py: Python, udim_source: &str, rotate: bool, rotate_method: &str, scale: bool, merge_overlap: bool, margin_method: &str, margin: f32, pin: bool, pin_method: &str, shape_method: &str) }
    bind_python! { bpy.ops.uv.paste() => pub fn paste(py: Python) }
    bind_python! { bpy.ops.uv.pin() => pub fn pin(py: Python, clear: bool) }
    bind_python! { bpy.ops.uv.project_from_view() => pub fn project_from_view(py: Python, orthographic: bool, camera_bounds: bool, correct_aspect: bool, clip_to_bounds: bool, scale_to_bounds: bool) }
    bind_python! { bpy.ops.uv.randomize_uv_transform() => pub fn randomize_uv_transform(py: Python, random_seed: u32, use_loc: bool, loc: [f32; 2], use_rot: bool, rot: f32, use_scale: bool, scale_even: bool, scale: [f32; 2]) }
    bind_python! { bpy.ops.uv.remove_doubles() => pub fn remove_doubles(py: Python, threshold: f32, use_unselected: bool) }
    bind_python! { bpy.ops.uv.reset() => pub fn reset(py: Python) }
    bind_python! { bpy.ops.uv.reveal() => pub fn reveal(py: Python, select: bool) }
    bind_python! { bpy.ops.uv.rip() => pub fn rip(py: Python, mirror: bool, release_confirm: bool, use_accurate: bool, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.seams_from_islands() => pub fn seams_from_islands(py: Python, mark_seams: bool, mark_sharp: bool) }
    bind_python! { bpy.ops.uv.select() => pub fn select(py: Python, extend: bool, deselect: bool, toggle: bool, deselect_all: bool, select_passthrough: bool, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.select_all() => pub fn select_all(py: Python, action: &str) }
    bind_python! { bpy.ops.uv.select_box() => pub fn select_box(py: Python, pinned: bool, xmin: i32, xmax: i32, ymin: i32, ymax: i32, wait_for_input: bool, mode: &str) }
    bind_python! { bpy.ops.uv.select_circle() => pub fn select_circle(py: Python, x: i32, y: i32, radius: u32, wait_for_input: bool, mode: &str) }
    bind_python! { bpy.ops.uv.select_edge_ring() => pub fn select_edge_ring(py: Python, extend: bool, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.select_lasso() => pub fn select_lasso(py: Python, path: &Path, mode: &str) }
    bind_python! { bpy.ops.uv.select_less() => pub fn select_less(py: Python) }
    bind_python! { bpy.ops.uv.select_linked() => pub fn select_linked(py: Python) }
    bind_python! { bpy.ops.uv.select_linked_pick() => pub fn select_linked_pick(py: Python, extend: bool, deselect: bool, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.select_loop() => pub fn select_loop(py: Python, extend: bool, location: [f32; 2]) }
    bind_python! { bpy.ops.uv.select_mode() => pub fn select_mode(py: Python, r#type: &str) }
    bind_python! { bpy.ops.uv.select_more() => pub fn select_more(py: Python) }
    bind_python! { bpy.ops.uv.select_overlap() => pub fn select_overlap(py: Python, extend: bool) }
    bind_python! { bpy.ops.uv.select_pinned() => pub fn select_pinned(py: Python) }
    bind_python! { bpy.ops.uv.select_similar() => pub fn select_similar(py: Python, r#type: &str, compare: &str, threshold: f32) }
    bind_python! { bpy.ops.uv.select_split() => pub fn select_split(py: Python) }
    bind_python! { bpy.ops.uv.shortest_path_pick() => pub fn shortest_path_pick(py: Python, use_face_step: bool, use_topology_distance: bool, use_fill: bool, skip: u32, nth: u32, offset: i32, object_index: i32, index: i32) }
    bind_python! { bpy.ops.uv.shortest_path_select() => pub fn shortest_path_select(py: Python, use_face_step: bool, use_topology_distance: bool, use_fill: bool, skip: u32, nth: u32, offset: i32) }
    bind_python! { bpy.ops.uv.smart_project() => pub fn smart_project(py: Python, angle_limit: f32, margin_method: &str, island_margin: f32, area_weight: f32, correct_aspect: bool, scale_to_bounds: bool) }
    bind_python! { bpy.ops.uv.snap_cursor() => pub fn snap_cursor(py: Python, target: &str) }
    bind_python! { bpy.ops.uv.snap_selected() => pub fn snap_selected(py: Python, target: &str) }
    bind_python! { bpy.ops.uv.sphere_project() => pub fn sphere_project(py: Python, direction: &str, align: Alignment, pole: &str, seam: bool, correct_aspect: bool, clip_to_bounds: bool, scale_to_bounds: bool) }
    bind_python! { bpy.ops.uv.unwrap() => pub fn unwrap(py: Python, method: &str, fill_holes: bool, correct_aspect: bool, use_subsurf_data: bool, margin_method: &str, margin: f32) }
    bind_python! { bpy.ops.uv.weld() => pub fn weld(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.view2d.html>
// pub mod view2d {
//     use super::*;
//     bind_python! {}
// }

// /// <https://docs.blender.org/api/latest/bpy.ops.view3d.html>
// pub mod view3d {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.wm.html>
pub mod wm {
    use super::{bind_python, Path, PyAny};

    bind_python! { bpy.ops.wm.alembic_export => pub fn fn_alembic_export(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.alembic_import => pub fn fn_alembic_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.blend_strings_utf8_validate() => pub fn blend_strings_utf8_validate(py: Python) }
    bind_python! { bpy.ops.wm.call_menu() => pub fn call_menu(py: Python, name: &str) }
    bind_python! { bpy.ops.wm.call_menu_pie() => pub fn call_menu_pie(py: Python, name: &str) }
    bind_python! { bpy.ops.wm.call_panel() => pub fn call_panel(py: Python, name: &str, keep_open: bool) }
    bind_python! { bpy.ops.wm.collada_export => pub fn fn_collada_export(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.collada_import => pub fn fn_collada_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.context_collection_boolean_set() => pub fn context_collection_boolean_set(py: Python, data_path_iter: &str, data_path_item: &str, r#type: &str) }
    bind_python! { bpy.ops.wm.context_cycle_array() => pub fn context_cycle_array(py: Python, data_path: &str, reverse: bool) }
    bind_python! { bpy.ops.wm.context_cycle_enum() => pub fn context_cycle_enum(py: Python, data_path: &str, reverse: bool, wrap: bool) }
    bind_python! { bpy.ops.wm.context_cycle_int() => pub fn context_cycle_int(py: Python, data_path: &str, reverse: bool, wrap: bool) }
    bind_python! { bpy.ops.wm.context_menu_enum() => pub fn context_menu_enum(py: Python, data_path: &str) }
    bind_python! { bpy.ops.wm.context_modal_mouse() => pub fn context_modal_mouse(py: Python, data_path_iter: &str, data_path_item: &str, header_text: &str, input_scale: f32, invert: bool, initial_x: i32) }
    bind_python! { bpy.ops.wm.context_pie_enum() => pub fn context_pie_enum(py: Python, data_path: &str) }
    bind_python! { bpy.ops.wm.context_scale_float() => pub fn context_scale_float(py: Python, data_path: &str, value: f32) }
    bind_python! { bpy.ops.wm.context_scale_int() => pub fn context_scale_int(py: Python, data_path: &str, value: f32, always_step: bool) }
    bind_python! { bpy.ops.wm.context_set_boolean() => pub fn context_set_boolean(py: Python, data_path: &str, value: bool) }
    bind_python! { bpy.ops.wm.context_set_enum() => pub fn context_set_enum(py: Python, data_path: &str, value: &str) }
    bind_python! { bpy.ops.wm.context_set_float() => pub fn context_set_float(py: Python, data_path: &str, value: f32, relative: bool) }
    bind_python! { bpy.ops.wm.context_set_id() => pub fn context_set_id(py: Python, data_path: &str, value: &str) }
    bind_python! { bpy.ops.wm.context_set_int() => pub fn context_set_int(py: Python, data_path: &str, value: i32, relative: bool) }
    bind_python! { bpy.ops.wm.context_set_string() => pub fn context_set_string(py: Python, data_path: &str, value: &str) }
    bind_python! { bpy.ops.wm.context_set_value() => pub fn context_set_value(py: Python, data_path: &str, value: &str) }
    bind_python! { bpy.ops.wm.context_toggle() => pub fn context_toggle(py: Python, data_path: &str, module: &str) }
    bind_python! { bpy.ops.wm.context_toggle_enum() => pub fn context_toggle_enum(py: Python, data_path: &str, value_1: &str, value_2: &str) }
    bind_python! { bpy.ops.wm.debug_menu() => pub fn debug_menu(py: Python, debug_value: i32) }
    bind_python! { bpy.ops.wm.doc_view() => pub fn doc_view(py: Python, doc_id: &str) }
    bind_python! { bpy.ops.wm.doc_view_manual() => pub fn doc_view_manual(py: Python, doc_id: &str) }
    bind_python! { bpy.ops.wm.doc_view_manual_ui_context() => pub fn doc_view_manual_ui_context(py: Python) }
    bind_python! { bpy.ops.wm.drop_blend_file() => pub fn drop_blend_file(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.gpencil_export_pdf => pub fn fn_gpencil_export_pdf(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.gpencil_export_svg => pub fn fn_gpencil_export_svg(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.gpencil_import_svg => pub fn fn_gpencil_import_svg(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.interface_theme_preset_add() => pub fn interface_theme_preset_add(py: Python, name: &str, remove_name: bool, remove_active: bool) }
    bind_python! { bpy.ops.wm.keyconfig_preset_add() => pub fn keyconfig_preset_add(py: Python, name: &str, remove_name: bool, remove_active: bool) }
    bind_python! { bpy.ops.wm.lib_reload() => pub fn lib_reload(py: Python, library: &str, filepath: &Path, directory: &Path, filename: &str) }
    bind_python! { bpy.ops.wm.memory_statistics() => pub fn memory_statistics(py: Python) }
    bind_python! { bpy.ops.wm.obj_export => pub fn fn_obj_export(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.obj_import => pub fn fn_obj_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.open_mainfile() => pub fn open_mainfile(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.operator_cheat_sheet() => pub fn operator_cheat_sheet(py: Python) }
    bind_python! { bpy.ops.wm.operator_defaults() => pub fn operator_defaults(py: Python) }
    bind_python! { bpy.ops.wm.operator_pie_enum() => pub fn operator_pie_enum(py: Python, data_path: &str, prop_string: &str) }
    bind_python! { bpy.ops.wm.operator_preset_add() => pub fn operator_preset_add(py: Python, name: &str, remove_name: bool, remove_active: bool, operator: &str) }
    bind_python! { bpy.ops.wm.owner_disable() => pub fn owner_disable(py: Python, owner_id: &str) }
    bind_python! { bpy.ops.wm.owner_enable() => pub fn owner_enable(py: Python, owner_id: &str) }
    bind_python! { bpy.ops.wm.path_open() => pub fn path_open(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.ply_export => pub fn fn_ply_export(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.ply_import => pub fn fn_ply_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.previews_ensure() => pub fn previews_ensure(py: Python) }
    bind_python! { bpy.ops.wm.properties_add() => pub fn properties_add(py: Python, data_path: &str) }
    bind_python! { bpy.ops.wm.properties_context_change() => pub fn properties_context_change(py: Python, context: &str) }
    bind_python! { bpy.ops.wm.properties_edit_value() => pub fn properties_edit_value(py: Python, data_path: &str, property_name: &str, eval_string: &str) }
    bind_python! { bpy.ops.wm.properties_remove() => pub fn properties_remove(py: Python, data_path: &str, property_name: &str) }
    bind_python! { bpy.ops.wm.quit_blender() => pub fn quit_blender(py: Python) }
    bind_python! { bpy.ops.wm.radial_control() => pub fn radial_control(py: Python, data_path_primary: &str, data_path_secondary: &str, use_secondary: &str, rotation_path: &str, color_path: &str, fill_color_path: &str, fill_color_override_path: &str, fill_color_override_test_path: &str, zoom_path: &str, image_id: &str, secondary_tex: bool, release_confirm: bool) }
    bind_python! { bpy.ops.wm.read_factory_settings() => pub fn read_factory_settings(py: Python, use_empty: bool) }
    bind_python! { bpy.ops.wm.read_factory_userpref() => pub fn read_factory_userpref(py: Python) }
    bind_python! { bpy.ops.wm.read_history() => pub fn read_history(py: Python) }
    bind_python! { bpy.ops.wm.read_homefile() => pub fn read_homefile(py: Python, filepath: &Path, load_ui: bool, use_splash: bool, use_factory_startup: bool, app_template: &str, use_empty: bool) }
    bind_python! { bpy.ops.wm.read_userpref() => pub fn read_userpref(py: Python) }
    bind_python! { bpy.ops.wm.recover_auto_save() => pub fn recover_auto_save(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.recover_last_session() => pub fn recover_last_session(py: Python, use_scripts: bool) }
    bind_python! { bpy.ops.wm.redraw_timer() => pub fn redraw_timer(py: Python, r#type: &str, iterations: u32, time_limit: f32) }
    bind_python! { bpy.ops.wm.revert_mainfile() => pub fn revert_mainfile(py: Python, use_scripts: bool) }
    bind_python! { bpy.ops.wm.save_as_mainfile() => pub fn save_as_mainfile(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.save_homefile() => pub fn save_homefile(py: Python) }
    bind_python! { bpy.ops.wm.save_mainfile() => pub fn save_mainfile(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.save_userpref() => pub fn save_userpref(py: Python) }
    bind_python! { bpy.ops.wm.search_menu() => pub fn search_menu(py: Python) }
    bind_python! { bpy.ops.wm.search_operator() => pub fn search_operator(py: Python) }
    bind_python! { bpy.ops.wm.set_stereo_3d() => pub fn set_stereo_3d(py: Python, display_mode: &str, anaglyph_type: &str, interlace_type: &str, use_interlace_swap: bool, use_sidebyside_crosseyed: bool) }
    bind_python! { bpy.ops.wm.splash() => pub fn splash(py: Python) }
    bind_python! { bpy.ops.wm.splash_about() => pub fn splash_about(py: Python) }
    bind_python! { bpy.ops.wm.stl_import => pub fn fn_stl_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.sysinfo() => pub fn sysinfo(py: Python, filepath: &Path) }
    bind_python! { bpy.ops.wm.tool_set_by_id() => pub fn tool_set_by_id(py: Python, name: &str, cycle: bool, as_fallback: bool, space_type: &str) }
    bind_python! { bpy.ops.wm.tool_set_by_index() => pub fn tool_set_by_index(py: Python, index: i32, cycle: bool, expand: bool, as_fallback: bool, space_type: &str) }
    bind_python! { bpy.ops.wm.toolbar() => pub fn toolbar(py: Python) }
    bind_python! { bpy.ops.wm.toolbar_fallback_pie() => pub fn toolbar_fallback_pie(py: Python) }
    bind_python! { bpy.ops.wm.toolbar_prompt() => pub fn toolbar_prompt(py: Python) }
    bind_python! { bpy.ops.wm.url_open() => pub fn url_open(py: Python, url: &str) }
    bind_python! { bpy.ops.wm.url_open_preset() => pub fn url_open_preset(py: Python, r#type: &str, id: &str) }
    bind_python! { bpy.ops.wm.usd_export => pub fn fn_usd_export(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.usd_import => pub fn fn_usd_import(py: Python) -> Result<&PyAny> }
    bind_python! { bpy.ops.wm.window_close() => pub fn window_close(py: Python) }
    bind_python! { bpy.ops.wm.window_fullscreen_toggle() => pub fn window_fullscreen_toggle(py: Python) }
    bind_python! { bpy.ops.wm.window_new() => pub fn window_new(py: Python) }
    bind_python! { bpy.ops.wm.window_new_main() => pub fn window_new_main(py: Python) }
    bind_python! { bpy.ops.wm.xr_navigation_fly() => pub fn xr_navigation_fly(py: Python, mode: &str, lock_location_z: bool, lock_direction: bool, speed_frame_based: bool, speed_min: f32, speed_max: f32, speed_interpolation0: [f32; 2], speed_interpolation1: [f32; 2]) }
    bind_python! { bpy.ops.wm.xr_navigation_grab() => pub fn xr_navigation_grab(py: Python, lock_location: bool, lock_location_z: bool, lock_rotation: bool, lock_rotation_z: bool, lock_scale: bool) }
    bind_python! { bpy.ops.wm.xr_navigation_reset() => pub fn xr_navigation_reset(py: Python, location: bool, rotation: bool, scale: bool) }
    bind_python! { bpy.ops.wm.xr_navigation_teleport() => pub fn xr_navigation_teleport(py: Python, teleport_axes: [bool; 3], interpolation: f32, offset: f32, selectable_only: bool, distance: f32, from_viewer: bool, axis: [f32; 3], color: [f32; 4]) }
    bind_python! { bpy.ops.wm.xr_session_toggle() => pub fn xr_session_toggle(py: Python) }
}

// /// <https://docs.blender.org/api/latest/bpy.ops.workspace.html>
// pub mod workspace {
//     use super::*;
//     bind_python! {}
// }

/// <https://docs.blender.org/api/latest/bpy.ops.world.html>
pub mod world {
    use super::{bind_python, PyAny};

    bind_python! { bpy.ops.world.new() => pub fn new(py: Python) -> Result<&PyAny> }
}
