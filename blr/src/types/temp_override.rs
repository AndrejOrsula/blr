use crate::{bpy, result::Result};
use pyo3::{intern, types::PyDict, PyAny, Python};

#[repr(transparent)]
#[derive(derive_more::Deref)]
pub struct TempOverride<'py>(&'py PyAny);

impl<'py> TempOverride<'py> {
    pub fn new(py: Python<'py>, overrides: &'py PyDict) -> Result<Self> {
        let temp_override: Self = bpy::context::fn_temp_override(py)?
            .call((), Some(overrides))?
            .into();
        temp_override.enter(py)?;
        Ok(temp_override)
    }

    fn enter(&'py self, py: Python<'py>) -> Result<()> {
        self.call_method0(intern!(py, "__enter__"))?;
        Ok(())
    }

    fn exit(&'py self, py: Python<'py>) -> Result<()> {
        self.call_method0(intern!(py, "__exit__"))?;
        Ok(())
    }
}

impl<'py> Drop for TempOverride<'py> {
    fn drop(&mut self) {
        Python::with_gil(|py| self.exit(py)).unwrap();
    }
}

impl<'py> From<&'py PyAny> for TempOverride<'py> {
    fn from(value: &'py PyAny) -> Self {
        Self(value)
    }
}
