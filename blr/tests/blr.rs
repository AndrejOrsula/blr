//! Run blr tests in a single thread.

#[cfg(test)]
mod tests {
    use blr::{
        export::{
            AbcExporter, BlendExport, DaeExporter, FbxExporter, GltfExporter, ObjExporter,
            PlyExporter, StlExporter, UsdExporter, X3dExporter,
        },
        prelude::*,
        BlResult, Object,
    };
    use pyo3::Python;
    use std::path::Path;
    use tempfile::tempdir;

    /// All tests must be run sequentially because the Python API of Blender is not thread-safe
    #[test]
    fn blr() -> BlResult<()> {
        Python::with_gil(|py| {
            thread_safety(py)?;
            parse_version(py)?;
            let blend = setup_scene(py)?;
            export(&blend)?;
            Ok(())
        })
    }

    fn thread_safety(py: Python) -> BlResult<()> {
        // Assert (any thread is compatible before the first instantiation)
        assert!(is_current_thread_bpy_safe());

        // Arrange + Act
        let blend1 = BlendProject::empty(py);
        let blend2 = BlendProject::empty(py);

        // Assert
        if let Err(e) = blend1 {
            panic!("Failed to instantiate BlendProject: {e}");
        }
        if let Err(e) = blend2 {
            panic!("Failed to instantiate BlendProject: {e}");
        }
        assert!(is_current_thread_bpy_safe());

        drop(blend1);
        drop(blend2);

        std::thread::spawn(|| {
            assert!(!is_current_thread_bpy_safe());
        })
        .join()
        .unwrap();

        Ok(())
    }

    fn parse_version(py: Python) -> BlResult<()> {
        // Act
        let version = bpy_version(py)?;
        let version_major = bpy_version_major(py)?;
        let version_minor = bpy_version_minor(py)?;
        let version_patch = bpy_version_patch(py)?;

        // Assert
        assert_eq!(version_major, version.major as u8);
        assert_eq!(version_minor, version.minor as u8);
        assert_eq!(version_patch, version.patch as u8);
        Ok(())
    }

    fn setup_scene(py: Python) -> BlResult<BlendProject> {
        let blend = BlendProject::empty(py)?;

        let _object = Object::new_mesh_primitive_cube(
            py,
            1.0,
            true,
            [0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0],
            [1.0, 1.0, 1.0],
        )?;

        Ok(blend)
    }

    fn export(blend: &BlendProject) -> BlResult<()> {
        // Act + Assert
        export_default(blend)?;
        export_default_from_extension(blend)?;
        export_with_builder(blend)?;
        export_invalid_filepath(blend);

        fn export_default(blend: &BlendProject) -> BlResult<()> {
            // Arrange
            let tempdir = tempdir().unwrap();
            let dirpath = tempdir.path();

            fn test_with<T: BlendExport + Default>(
                blend: &BlendProject,
                dirpath: &Path,
            ) -> BlResult<()> {
                // Arrange
                let exporter = T::default();
                let filepath = dirpath.join(format!("test.{}", exporter.extension()));

                // Act
                let result_filepath = blend.export(exporter, &filepath)?;

                // Assert
                assert_eq!(result_filepath, filepath);
                Ok(())
            }
            test_with::<AbcExporter>(blend, dirpath)?;
            test_with::<DaeExporter>(blend, dirpath)?;
            test_with::<FbxExporter>(blend, dirpath)?;
            test_with::<GltfExporter>(blend, dirpath)?;
            test_with::<ObjExporter>(blend, dirpath)?;
            test_with::<PlyExporter>(blend, dirpath)?;
            test_with::<StlExporter>(blend, dirpath)?;
            test_with::<UsdExporter>(blend, dirpath)?;
            test_with::<X3dExporter>(blend, dirpath)?;
            Ok(())
        }

        fn export_default_from_extension(blend: &BlendProject) -> BlResult<()> {
            // Arrange
            let tempdir = tempdir().unwrap();
            let dirpath = tempdir.path();

            fn test_with<T: BlendExport + Default>(
                blend: &BlendProject,
                dirpath: &Path,
            ) -> BlResult<()> {
                // Arrange
                let filepath = dirpath.join(format!("test.{}", T::EXTENSIONS[0]));

                // Act
                let result_filepath = blend.export_default(&filepath)?;

                // Assert
                assert_eq!(result_filepath, filepath);
                Ok(())
            }
            test_with::<AbcExporter>(blend, dirpath)?;
            test_with::<DaeExporter>(blend, dirpath)?;
            test_with::<FbxExporter>(blend, dirpath)?;
            test_with::<GltfExporter>(blend, dirpath)?;
            test_with::<ObjExporter>(blend, dirpath)?;
            test_with::<PlyExporter>(blend, dirpath)?;
            test_with::<StlExporter>(blend, dirpath)?;
            test_with::<UsdExporter>(blend, dirpath)?;
            test_with::<X3dExporter>(blend, dirpath)?;
            Ok(())
        }

        fn export_with_builder(blend: &BlendProject) -> BlResult<()> {
            // Arrange
            let tempdir = tempdir().unwrap();
            let dirpath = tempdir.path();
            let filepath = dirpath.join("test");

            macro_rules! test_with {
                ($exporter:ty) => {
                    // Arrange
                    let exporter = <$exporter>::builder().build();
                    let ext = exporter.extension().to_string();

                    // Act
                    let result_filepath = blend.export(exporter, &filepath)?;

                    // Assert
                    assert_eq!(
                        result_filepath.to_str().unwrap(),
                        format!("{filepath}.{ext}", filepath = filepath.display(),)
                    );
                };
            }
            test_with!(AbcExporter);
            test_with!(DaeExporter);
            test_with!(FbxExporter);
            test_with!(GltfExporter);
            test_with!(ObjExporter);
            test_with!(PlyExporter);
            test_with!(StlExporter);
            test_with!(UsdExporter);
            test_with!(X3dExporter);
            Ok(())
        }

        fn export_invalid_filepath(blend: &BlendProject) {
            // Arrange
            let dir = tempdir().unwrap();
            let dirpath = dir.path();

            // Act + Assert (invalid filepath - existing directory)
            let result = blend.export(StlExporter::default(), dirpath);
            assert!(result.is_err());

            // Act + Assert (invalid filepath - invalid extension)
            let result = blend.export(ObjExporter::default(), dirpath.join("test.stl"));
            assert!(result.is_err());

            // Act + Assert (invalid filepath - unsupported extension)
            let result = blend.export_default(dirpath.join("test.txt"));
            assert!(result.is_err());

            // Act + Assert (invalid filepath - no extension)
            let result = blend.export_default(dirpath.join("test"));
            assert!(result.is_err());
        }

        Ok(())
    }
}
