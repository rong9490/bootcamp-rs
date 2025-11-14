pub mod vector;
pub mod matrix;
pub mod metrics;

pub use matrix::{multiply, Matrix};
pub use metrics::{AmapMetrics, CmapMetrics};
pub use vector::{dot_product, MyVector};