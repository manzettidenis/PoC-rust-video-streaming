// Domain-Driven Design Structure
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod shared;

// Re-export main types for convenience
pub use domain::video::*;
pub use application::services::*;
pub use infrastructure::http::*;
pub use shared::config::*; 