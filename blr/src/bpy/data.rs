//! Bindings for [`bpy.data`](https://docs.blender.org/api/latest/bpy.data.html).
use super::{
    bind_python, Collection, Materials, NodeTrees, ObjectCollection, PathBuf, PyAny, PyDict,
};

bind_python! { bpy.data.actions => pub fn actions(py: Python) -> Result<Collection> }
bind_python! { bpy.data.armatures => pub fn armatures(py: Python) -> Result<Collection> }
bind_python! { bpy.data.brushes => pub fn brushes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.cache_files => pub fn cache_files(py: Python) -> Result<Collection> }
bind_python! { bpy.data.cameras => pub fn cameras(py: Python) -> Result<Collection> }
bind_python! { bpy.data.collections => pub fn collections(py: Python) -> Result<Collection> }
bind_python! { bpy.data.curves => pub fn curves(py: Python) -> Result<Collection> }
bind_python! { bpy.data.filepath => pub fn filepath(py: Python) -> Result<PathBuf> }
bind_python! { bpy.data.fonts => pub fn fonts(py: Python) -> Result<Collection> }
bind_python! { bpy.data.grease_pencils => pub fn grease_pencils(py: Python) -> Result<Collection> }
bind_python! { bpy.data.hair_curves => pub fn hair_curves(py: Python) -> Result<Collection> }
bind_python! { bpy.data.images => pub fn images(py: Python) -> Result<Collection> }
bind_python! { bpy.data.is_dirty => pub fn is_dirty(py: Python) -> Result<bool> }
bind_python! { bpy.data.is_saved => pub fn is_saved(py: Python) -> Result<bool> }
bind_python! { bpy.data.lattices => pub fn lattices(py: Python) -> Result<Collection> }
bind_python! { bpy.data.libraries => pub fn libraries(py: Python) -> Result<Collection> }
bind_python! { bpy.data.lightprobes => pub fn lightprobes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.lights => pub fn lights(py: Python) -> Result<Collection> }
bind_python! { bpy.data.linestyles => pub fn linestyles(py: Python) -> Result<Collection> }
bind_python! { bpy.data.masks => pub fn masks(py: Python) -> Result<Collection> }
bind_python! { bpy.data.materials => pub fn materials(py: Python) -> Result<Materials> }
bind_python! { bpy.data.meshes => pub fn meshes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.metaballs => pub fn metaballs(py: Python) -> Result<Collection> }
bind_python! { bpy.data.movieclips => pub fn movieclips(py: Python) -> Result<Collection> }
bind_python! { bpy.data.node_groups => pub fn node_groups(py: Python) -> Result<NodeTrees> }
bind_python! { bpy.data.objects => pub fn objects(py: Python) -> Result<ObjectCollection> }
bind_python! { bpy.data.orphans_purge() => pub fn orphans_purge(py: Python, do_local_ids: bool, do_linked_ids: bool, do_recursive: bool) -> Result<isize> }
bind_python! { bpy.data.paint_curves => pub fn paint_curves(py: Python) -> Result<Collection> }
bind_python! { bpy.data.palettes => pub fn palettes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.particles => pub fn particles(py: Python) -> Result<Collection> }
bind_python! { bpy.data.pointclouds => pub fn pointclouds(py: Python) -> Result<Collection> }
bind_python! { bpy.data.scenes => pub fn scenes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.screens => pub fn screens(py: Python) -> Result<Collection> }
bind_python! { bpy.data.shape_keys => pub fn shape_keys(py: Python) -> Result<Collection> }
bind_python! { bpy.data.sounds => pub fn sounds(py: Python) -> Result<Collection> }
bind_python! { bpy.data.speakers => pub fn speakers(py: Python) -> Result<Collection> }
bind_python! { bpy.data.temp_data() => pub fn temp_data(py: Python) -> Result<&PyAny> }
bind_python! { bpy.data.texts => pub fn texts(py: Python) -> Result<Collection> }
bind_python! { bpy.data.textures => pub fn textures(py: Python) -> Result<Collection> }
bind_python! { bpy.data.use_autopack => pub fn use_autopack(py: Python) -> Result<bool> }
bind_python! { bpy.data.user_map() => pub fn user_map(py: Python) -> Result<&PyDict> }
bind_python! { bpy.data.version => pub fn version(py: Python) -> Result<[u8; 3]> }
bind_python! { bpy.data.volumes => pub fn volumes(py: Python) -> Result<Collection> }
bind_python! { bpy.data.window_managers => pub fn window_managers(py: Python) -> Result<Collection> }
bind_python! { bpy.data.workspaces => pub fn workspaces(py: Python) -> Result<Collection> }
bind_python! { bpy.data.worlds => pub fn worlds(py: Python) -> Result<Collection> }
