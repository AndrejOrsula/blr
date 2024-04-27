pub mod collection;
pub mod materials;
pub mod node_links;
pub mod node_trees;
pub mod node_trees_io;
pub mod nodes;
pub mod object_collection;
pub mod object_modifiers;

pub use collection::Collection;
pub use materials::Materials;
pub use node_links::NodeLinks;
pub use node_trees::NodeTrees;
pub use node_trees_io::{NodeTreeInputs, NodeTreeOutputs};
pub use nodes::Nodes;
pub use object_collection::ObjectCollection;
pub use object_modifiers::ObjectModifiers;
pub use pyo3::{intern, FromPyObject, PyObject, PyResult, Python};
use pyo3_macros_more::bind_python;

/// Implementation of <https://docs.blender.org/api/latest/bpy.types.bpy_prop_collection.html>
pub trait CollectionImpl<'py>:
    std::ops::Deref<Target = PyObject> + std::ops::DerefMut<Target = PyObject>
{
    type Item: FromPyObject<'py>;

    fn find(&self, py: Python, key: &str) -> PyResult<usize> {
        match self
            .as_ref(py)
            .call_method1(intern!(py, "find"), (key,))?
            .extract()?
        {
            -1_isize => Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Key '{key}' cannot be found in collection",
            ))),
            index => Ok(index as usize),
        }
    }

    fn get(&'py self, py: Python<'py>, key: &str) -> PyResult<Self::Item> {
        let value = self.as_ref(py).call_method1(intern!(py, "get"), (key,))?;
        if value.is_none() {
            Err(pyo3::exceptions::PyKeyError::new_err(format!(
                "Key '{key}' cannot be found in collection",
            )))
        } else {
            value.extract()
        }
    }

    bind_python! { self.items() => fn items(&'py self, py: Python<'py>) -> Result<Vec<(String, Self::Item)>> }
    bind_python! { self.keys() => fn keys(&self, py: Python) -> Result<Vec<String>> }
    bind_python! { self.values() => fn values(&'py self, py: Python<'py>) -> Result<Vec<Self::Item>> }
}
