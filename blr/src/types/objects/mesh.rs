use crate::{
    bpy,
    enums::{AxisXYZ, MeshSelectMode, ObjectMode, ObjectType, TransformOrientation},
    error::BlError,
    result::Result,
    types::{BpyID, Collection, CollectionImpl, IdMaterials, Object},
};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyAny, PyObject, Python, ToPyObject};
use pyo3_macros_more::bind_python;

/// Wrapper for <https://docs.blender.org/api/latest/bpy.types.Mesh.html>
#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct Mesh(PyObject);

impl BpyID for Mesh {}

impl Mesh {
    pub fn try_from_name(name: &str) -> Result<Self> {
        Python::with_gil(|py| Ok(bpy::data::meshes(py)?.get(py, name)?.into()))
    }

    pub fn try_from_active(py: Python) -> Result<Self> {
        Object::from_active(py)?.try_into()
    }

    pub fn from_data(
        self,
        py: Python,
        name: &str,
        vertices: Vec<[f32; 3]>,
        edges: Vec<[usize; 2]>,
        faces: Vec<Vec<usize>>,
    ) -> Result<Self> {
        let mut mesh = self;
        mesh.from_pydata(py, vertices, edges, faces)?;
        if mesh.validate(py, false, true)? {
            Ok(mesh)
        } else {
            Err(BlError::ValueError(format!(
                "Mesh '{name}' cannot be created from the provided data",
            )))
        }
    }

    pub fn get_parent_objects(&self, py: Python) -> Result<Vec<Object>> {
        let mesh_name = self.name_full(py)?;
        bpy::data::objects(py)?
            .values(py)?
            .into_iter()
            .filter_map(|object| -> Option<Result<Object>> {
                match object.object_type(py) {
                    Ok(ObjectType::Mesh) => match object.name_full(py) {
                        Ok(name) => {
                            if mesh_name.contains(&name) {
                                Some(Ok(object))
                            } else {
                                None
                            }
                        }
                        Err(e) => Some(Err(e.into())),
                    },
                    _ => None,
                }
            })
            .collect()
    }

    /// Note: Works only if the mesh has the same name as the parent object
    pub fn find_parent_object(&self, py: Python) -> Result<Option<Object>> {
        let mesh_name = self.name_full(py)?;
        bpy::data::objects(py)?
            .values(py)?
            .into_iter()
            .find_map(|object| -> Option<Result<Object>> {
                match object.object_type(py) {
                    Ok(ObjectType::Mesh) => match object.name_full(py) {
                        Ok(name) => {
                            if mesh_name.contains(&name) {
                                Some(Ok(object))
                            } else {
                                None
                            }
                        }
                        Err(e) => Some(Err(e.into())),
                    },
                    _ => None,
                }
            })
            .transpose()
    }

    pub fn set_edit_mode(&self, py: Python) -> Result<()> {
        match self.find_parent_object(py)? {
            None => Err(BlError::ValueError(format!(
                "Mesh '{name}' has no parent object",
                name = self.name(py)?
            ))),
            Some(object) => {
                object.set_active(py)?;
                object.set_mode(py, ObjectMode::Edit)
            }
        }
    }

    pub fn set_select_mode(&self, py: Python, mode: MeshSelectMode) -> Result<()> {
        Ok(bpy::ops::mesh::select_mode(
            py, false, false, mode, "ENABLE",
        )?)
    }

    pub fn select_all(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::select_all(py, "SELECT")?)
    }

    pub fn deselect_all(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::select_all(py, "DESELECT")?)
    }

    pub fn toggle_selection(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::select_all(py, "TOGGLE")?)
    }

    pub fn invert_selection(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::select_all(py, "INVERT")?)
    }

    pub fn remove_doubles(
        &mut self,
        py: Python,
        threshold: f32,
        use_unselected: bool,
    ) -> Result<()> {
        Ok(bpy::ops::mesh::remove_doubles(
            py,
            threshold,
            use_unselected,
            false,
        )?)
    }

    pub fn tris_to_quads(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::tris_convert_to_quads(
            py, 0.698_132, 0.698_132, false, false, false, false, false,
        )?)
    }

    pub fn quads_to_tris(&mut self, py: Python) -> Result<()> {
        Ok(bpy::ops::mesh::quads_convert_to_tris(
            py, "BEAUTY", "BEAUTY",
        )?)
    }

    pub fn make_normals_consistent(&mut self, py: Python, inside: bool) -> Result<()> {
        Ok(bpy::ops::mesh::normals_make_consistent(py, inside)?)
    }

    pub fn rotate_normals(&mut self, py: Python, value: f32, orient_axis: AxisXYZ) -> Result<()> {
        Ok(bpy::ops::transform::rotate_normal(
            py,
            value,
            orient_axis,
            TransformOrientation::Global,
            [[0.0, 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, 0.0, 0.0]],
            TransformOrientation::Global,
            [false, false, false],
            false,
            false,
            false,
        )?)
    }

    pub fn get_output_attribute_data(&self, py: Python, key: &str) -> Result<PyObject> {
        let parent_object = self.find_parent_object(py)?.unwrap();

        let evaluated_object: Object = parent_object
            .evaluated_get(
                py,
                bpy::context::evaluated_depsgraph_get(py)
                    .unwrap()
                    .call0()
                    .unwrap(),
            )?
            .extract()?;
        let evaluated_mesh: Mesh = evaluated_object.try_into()?;

        let attributes = evaluated_mesh.attributes(py)?;
        let attribute_data: Collection = attributes.get(py, key)?.getattr("data")?.extract()?;
        let vector_data = attribute_data.values(py)?[0].getattr("vector")?;
        Ok(vector_data.to_object(py))
    }

    bind_python! { self.animation_data => pub fn animation_data<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.attributes => pub fn attributes(&self, py: Python) -> Result<Collection> }
    bind_python! { self.auto_smooth_angle => pub fn auto_smooth_angle(&self, py: Python) -> Result<f32> }
    bind_python! { self.auto_smooth_angle = pub fn set_auto_smooth_angle(&mut self, py: Python, value: f32) }
    bind_python! { self.auto_texspace => pub fn auto_texspace(&self, py: Python) -> Result<bool> }
    bind_python! { self.auto_texspace = pub fn set_auto_texspace(&mut self, py: Python, value: bool) }
    bind_python! { self.color_attributes => pub fn color_attributes(&self, py: Python) -> Result<Collection> }
    bind_python! { self.corner_normals => pub fn corner_normals(&self, py: Python) -> Result<Collection> }
    bind_python! { self.cycles => pub fn cycles<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.edge_creases => pub fn edge_creases(&self, py: Python) -> Result<Collection> }
    bind_python! { self.edges => pub fn edges(&self, py: Python) -> Result<Collection> }
    bind_python! { self.face_maps => pub fn face_maps(&self, py: Python) -> Result<Collection> }
    bind_python! { self.has_bevel_weight_edge => pub fn has_bevel_weight_edge(&self, py: Python) -> Result<bool> }
    bind_python! { self.has_bevel_weight_vertex => pub fn has_bevel_weight_vertex(&self, py: Python) -> Result<bool> }
    bind_python! { self.has_crease_edge => pub fn has_crease_edge(&self, py: Python) -> Result<bool> }
    bind_python! { self.has_crease_vertex => pub fn has_crease_vertex(&self, py: Python) -> Result<bool> }
    bind_python! { self.has_custom_normals => pub fn has_custom_normals(&self, py: Python) -> Result<bool> }
    bind_python! { self.is_editmode => pub fn is_editmode(&self, py: Python) -> Result<bool> }
    bind_python! { self.loop_triangle_polygons => pub fn loop_triangle_polygons(&self, py: Python) -> Result<Collection> }
    bind_python! { self.loop_triangles => pub fn loop_triangles(&self, py: Python) -> Result<Collection> }
    bind_python! { self.loops => pub fn loops(&self, py: Python) -> Result<Collection> }
    bind_python! { self.materials => pub fn materials(&self, py: Python) -> Result<IdMaterials> }
    bind_python! { self.polygon_layers_float => pub fn polygon_layers_float(&self, py: Python) -> Result<Collection> }
    bind_python! { self.polygon_layers_int => pub fn polygon_layers_int(&self, py: Python) -> Result<Collection> }
    bind_python! { self.polygon_layers_string => pub fn polygon_layers_string(&self, py: Python) -> Result<Collection> }
    bind_python! { self.polygon_normals => pub fn polygon_normals(&self, py: Python) -> Result<Collection> }
    bind_python! { self.polygons => pub fn polygons(&self, py: Python) -> Result<Collection> }
    bind_python! { self.remesh_mode => pub fn remesh_mode(&self, py: Python) -> Result<String> }
    bind_python! { self.remesh_mode = pub fn set_remesh_mode(&mut self, py: Python, value: &str) }
    bind_python! { self.remesh_voxel_adaptivity => pub fn remesh_voxel_adaptivity(&self, py: Python) -> Result<f32> }
    bind_python! { self.remesh_voxel_adaptivity = pub fn set_remesh_voxel_adaptivity(&mut self, py: Python, value: f32) }
    bind_python! { self.remesh_voxel_size => pub fn remesh_voxel_size(&self, py: Python) -> Result<f32> }
    bind_python! { self.remesh_voxel_size = pub fn set_remesh_voxel_size(&mut self, py: Python, value: f32) }
    bind_python! { self.sculpt_vertex_colors => pub fn sculpt_vertex_colors(&self, py: Python) -> Result<Collection> }
    bind_python! { self.shape_keys => pub fn shape_keys<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.skin_vertices => pub fn skin_vertices<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.texco_mesh => pub fn texco_mesh(&self, py: Python) -> Result<Self> }
    bind_python! { self.texco_mesh = pub fn set_texco_mesh(&mut self, py: Python, value: &Self) }
    bind_python! { self.texspace_location => pub fn texspace_location<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.texspace_location = pub fn set_texspace_location(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.texspace_size => pub fn texspace_size<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.texspace_size = pub fn set_texspace_size(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.texture_mesh => pub fn texture_mesh(&self, py: Python) -> Result<Self> }
    bind_python! { self.texture_mesh = pub fn set_texture_mesh(&mut self, py: Python, value: &Self) }
    bind_python! { self.total_edge_sel => pub fn total_edge_sel(&self, py: Python) -> Result<u32> }
    bind_python! { self.total_face_sel => pub fn total_face_sel(&self, py: Python) -> Result<u32> }
    bind_python! { self.total_vert_sel => pub fn total_vert_sel(&self, py: Python) -> Result<u32> }
    bind_python! { self.use_auto_smooth => pub fn use_auto_smooth(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_auto_smooth = pub fn set_use_auto_smooth(&mut self, py: Python, value: bool) }
    bind_python! { self.use_auto_texspace => pub fn use_auto_texspace(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_auto_texspace = pub fn set_use_auto_texspace(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mirror_topology => pub fn use_mirror_topology(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mirror_topology = pub fn set_use_mirror_topology(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mirror_vertex_groups => pub fn use_mirror_vertex_groups(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mirror_vertex_groups = pub fn set_use_mirror_vertex_groups(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mirror_x => pub fn use_mirror_x(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mirror_x = pub fn set_use_mirror_x(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mirror_y => pub fn use_mirror_y(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mirror_y = pub fn set_use_mirror_y(&mut self, py: Python, value: bool) }
    bind_python! { self.use_mirror_z => pub fn use_mirror_z(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_mirror_z = pub fn set_use_mirror_z(&mut self, py: Python, value: bool) }
    bind_python! { self.use_paint_mask => pub fn use_paint_mask(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_paint_mask = pub fn set_use_paint_mask(&mut self, py: Python, value: bool) }
    bind_python! { self.use_paint_mask_vertex => pub fn use_paint_mask_vertex(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_paint_mask_vertex = pub fn set_use_paint_mask_vertex(&mut self, py: Python, value: bool) }
    bind_python! { self.use_remesh_fix_poles => pub fn use_remesh_fix_poles(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_remesh_fix_poles = pub fn set_use_remesh_fix_poles(&mut self, py: Python, value: bool) }
    bind_python! { self.use_remesh_preserve_paint_mask => pub fn use_remesh_preserve_paint_mask(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_remesh_preserve_paint_mask = pub fn set_use_remesh_preserve_paint_mask(&mut self, py: Python, value: bool) }
    bind_python! { self.use_remesh_preserve_sculpt_face_sets => pub fn use_remesh_preserve_sculpt_face_sets(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_remesh_preserve_sculpt_face_sets = pub fn set_use_remesh_preserve_sculpt_face_sets(&mut self, py: Python, value: bool) }
    bind_python! { self.use_remesh_preserve_vertex_colors => pub fn use_remesh_preserve_vertex_colors(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_remesh_preserve_vertex_colors = pub fn set_use_remesh_preserve_vertex_colors(&mut self, py: Python, value: bool) }
    bind_python! { self.use_remesh_preserve_volume => pub fn use_remesh_preserve_volume(&self, py: Python) -> Result<bool> }
    bind_python! { self.use_remesh_preserve_volume = pub fn set_use_remesh_preserve_volume(&mut self, py: Python, value: bool) }
    bind_python! { self.uv_layer_clone => pub fn uv_layer_clone<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.uv_layer_clone = pub fn set_uv_layer_clone(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.uv_layer_clone_index => pub fn uv_layer_clone_index(&self, py: Python) -> Result<usize> }
    bind_python! { self.uv_layer_clone_index = pub fn set_uv_layer_clone_index(&mut self, py: Python, value: usize) }
    bind_python! { self.uv_layer_stencil => pub fn uv_layer_stencil<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.uv_layer_stencil = pub fn set_uv_layer_stencil(&mut self, py: Python, value: &PyAny) }
    bind_python! { self.uv_layer_stencil_index => pub fn uv_layer_stencil_index(&self, py: Python) -> Result<usize> }
    bind_python! { self.uv_layer_stencil_index = pub fn set_uv_layer_stencil_index(&mut self, py: Python, value: usize) }
    bind_python! { self.uv_layers => pub fn uv_layers(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_colors => pub fn vertex_colors(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_creases => pub fn vertex_creases(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_layers_float => pub fn vertex_layers_float(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_layers_int => pub fn vertex_layers_int(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_layers_string => pub fn vertex_layers_string(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_normals => pub fn vertex_normals(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertex_paint_masks => pub fn vertex_paint_masks(&self, py: Python) -> Result<Collection> }
    bind_python! { self.vertices => pub fn vertices(&self, py: Python) -> Result<Collection> }
    bind_python! { self.edge_keys => pub fn edge_keys<'py>(&'py self, py: Python<'py>) -> Result<&'py PyAny> }
    bind_python! { self.transform() => pub fn transform(&mut self, py: Python, matrix: &PyAny, shape_keys: bool) }
    bind_python! { self.flip_normals() => pub fn flip_normals(&mut self, py: Python) }
    bind_python! { self.calc_normals() => pub fn calc_normals(&self, py: Python) }
    bind_python! { self.create_normals_split() => pub fn create_normals_split(&self, py: Python) }
    bind_python! { self.calc_normals_split() => pub fn calc_normals_split(&self, py: Python) }
    bind_python! { self.free_normals_split() => pub fn free_normals_split(&self, py: Python) }
    bind_python! { self.split_faces() => pub fn split_faces(&self, py: Python, free_loop_normals: bool) }
    bind_python! { self.calc_tangents() => pub fn calc_tangents(&self, py: Python, uvmap: &str) }
    bind_python! { self.free_tangents() => pub fn free_tangents(&self, py: Python) }
    bind_python! { self.calc_loop_triangles() => pub fn calc_loop_triangles(&self, py: Python) }
    bind_python! { self.calc_smooth_groups() => pub fn calc_smooth_groups<'py>(&'py self, py: Python<'py>, use_bitflags: bool) -> Result<&'py PyAny> }
    bind_python! { self.normals_split_custom_set() => pub fn normals_split_custom_set(&self, py: Python, normals: &PyAny) }
    bind_python! { self.normals_split_custom_set_from_vertices() => pub fn normals_split_custom_set_from_vertices(&self, py: Python, normals: &PyAny) }
    bind_python! { self.update() => pub fn update(&mut self, py: Python, calc_edges: bool, calc_edges_loose: bool) }
    bind_python! { self.update_gpu_tag() => pub fn update_gpu_tag(&mut self, py: Python) }
    bind_python! { self.unit_test_compare() => pub fn unit_test_compare(&self, py: Python, mesh: &Self, threshold: f32) -> Result<String> }
    bind_python! { self.clear_geometry() => pub fn clear_geometry(&mut self, py: Python) }
    bind_python! { self.validate() => pub fn validate(&self, py: Python, verbose: bool, clean_customdata: bool) -> Result<bool> }
    bind_python! { self.validate_material_indices() => pub fn validate_material_indices(&self, py: Python) -> Result<bool> }
    bind_python! { self.count_selected_items() => pub fn count_selected_items(&self, py: Python) -> Result<[usize; 3]> }
    bind_python! { self.from_pydata() => fn from_pydata(&mut self, py: Python, vertices: Vec<[f32; 3]>, edges: Vec<[usize; 2]>, faces: Vec<Vec<usize>>) }
    bind_python! { self.shade_flat() => pub fn shade_flat(&mut self, py: Python) }
    bind_python! { self.shade_smooth() => pub fn shade_smooth(&mut self, py: Python) }
}

impl From<pyo3::PyObject> for Mesh {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for Mesh {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for Mesh {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}

impl pyo3::ToPyObject for Mesh {
    fn to_object(&self, py: pyo3::Python<'_>) -> pyo3::PyObject {
        self.as_ref(py).to_object(py)
    }
}
