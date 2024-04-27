//! Rust bindings for the Python API of Blender.
pub mod bpy;
pub mod enums;
pub mod export;
pub mod import;
pub mod project;
pub mod types;
pub mod utils;

/// Prelude module for the blr crate that re-exports the most commonly used items.
pub mod prelude {
    pub use crate::bpy;
    pub use crate::error::BlError;
    pub use crate::project::BlendProject;
    pub use crate::result::BlResult;
    pub use crate::types::{BpyID, BpyStruct, CollectionImpl, CurveImpl, ModifierImpl};
    pub use crate::{
        utils::thread_safety::{bpy_thread_id, is_current_thread_bpy_safe},
        version::{bpy_version, bpy_version_major, bpy_version_minor, bpy_version_patch},
    };
}

pub use enums::*;
pub use project::BlendProject;
pub use types::*;
pub(crate) use utils::macros;
pub use utils::{
    error::{self, BlError},
    result::{self, BlResult},
    thread_safety, version,
};
