pub mod id_materials;
pub mod material;

pub use id_materials::IdMaterials;
pub use material::Material;

pub type Texture<'py> = &'py pyo3::PyAny;
