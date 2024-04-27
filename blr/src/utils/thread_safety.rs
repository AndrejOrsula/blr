use crate::{error::BlError, result::Result, BlendProject};
use std::sync::Mutex;

/// Static that indicates the thread in which the Python API of Blender is instantiated.
static BPY_THREAD_ID: Mutex<Option<std::thread::ThreadId>> = Mutex::new(None);

/// Returns the thread in which the Python API of Blender is instantiated.
/// Returns `None` if the Python API of Blender has not been instantiated yet.
pub fn bpy_thread_id() -> Option<std::thread::ThreadId> {
    *BPY_THREAD_ID.lock().unwrap()
}

/// Returns true if the current thread is the thread in which the Python API of Blender is instantiated
/// or if the Python API of Blender has not been instantiated yet.
/// Returns false otherwise if called from a different thread.
#[must_use]
pub fn is_current_thread_bpy_safe() -> bool {
    bpy_thread_id().map_or(true, |bpy_thread_id| {
        std::thread::current().id() == bpy_thread_id
    })
}

impl BlendProject {
    pub(crate) fn ensure_thread_safety() -> Result<()> {
        let mut bpy_thread_id = BPY_THREAD_ID.lock().unwrap();
        match *bpy_thread_id {
            Some(current_thread_id) => {
                if current_thread_id != std::thread::current().id() {
                    return Err(BlError::ValueError(
                        "Cannot re-instantiate the Python API of Blender in a new thread after it has been \
                         instantiated in a different thread."
                            .to_string(),
                    ));
                }
            }
            None => {
                *bpy_thread_id = Some(std::thread::current().id());
            }
        }

        Ok(())
    }
}
