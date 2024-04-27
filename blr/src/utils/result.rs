/// Result wrapper for `BlError`.
pub type BlResult<T> = std::result::Result<T, crate::BlError>;

/// Crate-local alias for `BlResult`.
pub(crate) type Result<T> = BlResult<T>;
