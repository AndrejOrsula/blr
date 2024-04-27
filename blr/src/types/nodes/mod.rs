pub mod node;
pub mod node_link;
pub mod node_socket;
pub mod node_socket_interface;
pub mod node_tree;

pub use node::Node;
pub use node_link::NodeLink;
pub use node_socket::NodeSocket;
pub use node_socket_interface::NodeSocketInterface;
pub use node_tree::NodeTree;

// pub type NodeTreePath<'py> = &'py pyo3::PyAny;
// pub type GeometryNode<'py> = &'py pyo3::PyAny;
// pub type ShaderNode<'py> = &'py pyo3::PyAny;
