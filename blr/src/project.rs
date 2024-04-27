//! Safe and idiomatic interface for a Blender project.
use crate::{
    bpy,
    error::BlError,
    export::{BlendExport, BlendExporter},
    import::{BlendImport, BlendImporter},
    result::Result,
};
use pyo3::Python;
use std::path::{Path, PathBuf};

/// Blender project using the Python API of Blender.
///
///
/// # Examples
///
/// ```no_run
/// use blr::BlendProject;
///
/// let blender_project = pyo3::Python::with_gil(|py| {
///    BlendProject::empty(py).unwrap()
/// });
/// ```
///
/// # Remarks
///
/// The Python API of Blender is not thread-safe. Therefore, this type is neither `Send` nor `Sync`.
/// Furthermore, this type is a singleton, i.e. there can only be one instance of this type in the
/// current process to ensure the thread-safety of the Python API of Blender.
///
/// Therefore, the following examples fail to compile:
///
/// ```compile_fail
/// use blr::BlendProject;
///
/// const fn trait_impls_noop_send<T: Send>() {}
/// trait_impls_noop_send::<BlendProject>();
/// ```
///
/// ```compile_fail
/// use blr::BlendProject;
///
/// const fn trait_impls_noop_sync<T: Sync>() {}
/// trait_impls_noop_sync::<BlendProject>();
/// ```
#[derive(Clone, Debug)]
pub struct BlendProject {
    /// The Python API of Blender is not thread-safe, so this type is neither `Send` nor `Sync`
    no_send_sync: std::marker::PhantomData<*const ()>,
}

impl BlendProject {
    /// Returns a new instance of `BlendProject` for the given Blender project file.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the Blender project file.
    ///
    /// # Errors
    ///
    /// Returns an error if the requirements for the Python API of Blender are not met
    /// or if the singleton of this type is already instantiated in the current process.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn empty(py: Python) -> Result<Self> {
        Self::ensure_thread_safety()?;

        bpy::ops::wm::read_factory_settings(py, true)?;

        Ok(Self {
            no_send_sync: std::marker::PhantomData,
        })
    }

    /// Returns a new instance of `BlendProject` from a Blender project `filepath`.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the Blender project file.
    ///
    /// # Errors
    ///
    /// Returns an error if the requirements for the Python API of Blender are not met
    /// or if the singleton of this type is already instantiated in the current process.
    /// Error is also returned if the given `filepath` does not point to a valid file.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn open(&self, py: Python, filepath: impl AsRef<Path>) -> Result<Self> {
        Self::ensure_thread_safety()?;

        let filepath = filepath.as_ref();
        if !filepath.is_file() {
            return Err(BlError::ValueError(format!(
                "Filepath '{}' does not point to a valid file.",
                filepath.display()
            )));
        }

        bpy::ops::wm::open_mainfile(py, filepath)?;

        Ok(Self {
            no_send_sync: std::marker::PhantomData,
        })
    }

    /// Saves the current Blender project to a given `filepath`.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the Blender project file.
    ///
    /// # Errors
    ///
    /// Returns an error if the given `filepath` is not valid.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn save(&self, py: Python, filepath: impl AsRef<Path>) -> Result<PathBuf> {
        let filepath = Self::check_save_filepath(filepath)?;

        bpy::ops::wm::save_mainfile(py, filepath.as_ref())?;

        Ok(filepath)
    }

    /// Imports the current Blender project to a given `filepath` using a given `importer`.
    ///
    /// # Arguments
    ///
    /// * `importer` - The importer configuration to use.
    /// * `filepath` - The path to the imported file.
    ///
    /// # Errors
    ///
    /// Returns an error if the given `filepath` is not valid.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn import<E: BlendImport>(&self, importer: E, filepath: impl AsRef<Path>) -> Result<()> {
        importer.import(filepath)
    }

    /// Imports the current Blender project to a given `filepath`. The importer is determined
    /// by the file extension of the given `filepath` and used with default settings.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the imported file.
    ///
    /// # Errors
    ///
    /// Returns an error if the given `filepath` is not valid or if it cannot be mapped to a
    /// supported importer.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn import_default(&self, filepath: impl AsRef<Path>) -> Result<()> {
        BlendImporter::from_filepath_extension(&filepath)?.import(&filepath)
    }

    /// Exports the current Blender project to a given `filepath` using a given `exporter`.
    ///
    /// # Arguments
    ///
    /// * `exporter` - The exporter configuration to use.
    /// * `filepath` - The path to the exported file.
    ///
    /// # Errors
    ///
    /// Returns an error if the given `filepath` is not valid.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn export<E: BlendExport>(
        &self,
        exporter: E,
        filepath: impl AsRef<Path>,
    ) -> Result<PathBuf> {
        exporter.export(filepath)
    }

    /// Exports the current Blender project to a given `filepath`. The exporter is determined
    /// by the file extension of the given `filepath` and used with default settings.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the exported file.
    ///
    /// # Errors
    ///
    /// Returns an error if the given `filepath` is not valid or if it cannot be mapped to a
    /// supported exporter.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    pub fn export_default(&self, filepath: impl AsRef<Path>) -> Result<PathBuf> {
        BlendExporter::from_filepath_extension(&filepath)?.export(&filepath)
    }

    fn check_save_filepath(filepath: impl AsRef<Path>) -> Result<PathBuf> {
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
            Some(invalid_ext) if invalid_ext.to_ascii_lowercase() != "blend" => {
                Err(BlError::ValueError(format!(
                    "Invalid file extension (expected: 'blend', actual: '{invalid_ext}')",
                    invalid_ext = invalid_ext.to_str().unwrap()
                )))
            }
            _ => Ok(filepath.with_extension("blend")),
        }
    }
}

impl Default for BlendProject {
    /// Returns a new instance of `BlendProject` with the default factory settings loaded.
    ///
    /// # Panics
    ///
    /// The instantiation panics if the requirements for the Python API of Blender are not met
    /// or if the singleton of this type is already instantiated in the current process.
    /// Furthermore, exceptions from the Python API of Blender are propagated.
    fn default() -> Self {
        Self::ensure_thread_safety().unwrap();

        Python::with_gil(|py| {
            bpy::ops::wm::read_factory_settings(py, false).unwrap();
        });

        Self {
            no_send_sync: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn trait_impls() {
        // Arrange
        const fn trait_impls_noop<T: Sized + Unpin + Clone + Debug>() {}

        // Act (successful compilation serves as the assertion)
        trait_impls_noop::<BlendProject>();
    }
}
