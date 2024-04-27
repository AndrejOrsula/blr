use super::CollectionImpl;
use crate::{enums::ModifierType, result::Result, types::Modifier};
use derive_more::{Deref, DerefMut, Display};
use pyo3::{PyObject, Python};
use pyo3_macros_more::bind_python;

#[repr(transparent)]
#[derive(Clone, Debug, Deref, DerefMut, Display)]
pub struct ObjectModifiers(PyObject);

impl<'py> CollectionImpl<'py> for ObjectModifiers {
    type Item = Modifier;
}

impl ObjectModifiers {
    pub fn get_of_type(&self, py: Python, r#type: ModifierType) -> Result<Vec<Modifier>> {
        Ok(self
            .values(py)?
            .into_iter()
            .filter(|modifier| Into::<ModifierType>::into(modifier) == r#type)
            .collect())
    }

    pub fn find_of_type(&self, py: Python, r#type: ModifierType) -> Result<Vec<usize>> {
        Ok(self
            .values(py)?
            .into_iter()
            .enumerate()
            .filter_map(
                |(index, modifier)| match Into::<ModifierType>::into(modifier) {
                    modifier_type if modifier_type == r#type => Some(index),
                    _ => None,
                },
            )
            .collect())
    }

    bind_python! { self.active => pub fn active(&self, py: Python) -> Result<Modifier> }
    bind_python! { self.active = pub fn set_active(&mut self, py: Python, value: &Modifier) }
    bind_python! { self.new() => pub fn new(&self, py: Python, name: &str, r#type: ModifierType) -> Result<Modifier> }
    bind_python! { self.remove() => pub fn remove(&self, py: Python, modifier: Modifier) }
    bind_python! { self.clear() => pub fn clear(&self, py: Python) }
    bind_python! { self.move() => pub fn r#move(&self, py: Python, from_index: usize, to_index: usize) }
}

impl From<pyo3::PyObject> for ObjectModifiers {
    fn from(value: pyo3::PyObject) -> Self {
        Self(value)
    }
}

impl From<&pyo3::PyAny> for ObjectModifiers {
    fn from(value: &pyo3::PyAny) -> Self {
        Self(value.into())
    }
}

impl pyo3::FromPyObject<'_> for ObjectModifiers {
    fn extract(value: &pyo3::PyAny) -> pyo3::PyResult<Self> {
        Ok(Self(value.into()))
    }
}
