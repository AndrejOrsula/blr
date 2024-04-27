pub mod view_layer;

pub use view_layer::ViewLayer;

pub type Scene<'py> = &'py pyo3::PyAny;
pub type Screen<'py> = &'py pyo3::PyAny;
pub type SpaceView3D<'py> = &'py pyo3::PyAny;
pub type Window<'py> = &'py pyo3::PyAny;
pub type World<'py> = &'py pyo3::PyAny;
