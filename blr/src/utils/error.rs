use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlError {
    #[error(transparent)]
    BuilderUninitializedFieldError(#[from] derive_builder::UninitializedFieldError),

    #[error("Dependency error: {0}")]
    DependencyError(String),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    PyError(#[from] pyo3::PyErr),

    #[error("Type error: {0}")]
    TypeError(String),

    #[error("Value error: {0}")]
    ValueError(String),
}

impl From<BlError> for pyo3::PyErr {
    fn from(e: BlError) -> Self {
        match e {
            BlError::BuilderUninitializedFieldError(e) => {
                pyo3::exceptions::PyValueError::new_err(e.to_string())
            }
            BlError::DependencyError(e) => pyo3::exceptions::PyImportError::new_err(e),
            BlError::IoError(e) => pyo3::exceptions::PyIOError::new_err(e),
            BlError::PyError(e) => e,
            BlError::TypeError(e) => pyo3::exceptions::PyTypeError::new_err(e),
            BlError::ValueError(e) => pyo3::exceptions::PyValueError::new_err(e),
        }
    }
}
