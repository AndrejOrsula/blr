//! Low-level "unsafe" bindings for the Python module of Blender ([`bpy`](https://docs.blender.org/api/latest/)).
#![allow(clippy::too_many_arguments)]

use crate::{
    enums::{
        Alignment, AxisXYZ, ContextMode, LightType, MeshSelectMode, ModifierType, ObjectMode,
        ObjectType, OriginCenter, OriginType, SnapElement, TransformOrientation,
    },
    types::{Collection, Materials, NodeTrees, Object, ObjectCollection, ViewLayer},
};
use pyo3::{types::PyDict, PyAny};
use pyo3_macros_more::bind_python;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

pub mod app;
pub mod context;
pub mod data;
pub mod ops;
pub mod utils;
