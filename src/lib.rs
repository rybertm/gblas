mod algebra;
mod backend;
mod complement_mask;
mod descriptor;
mod error;
mod indices;
mod mask;
mod matrix;
mod operations;
mod structure_mask;
mod types;
mod vector;

pub use error::{ApiError, ExecutionError, GblasError};

pub type GblasResult<T> = Result<T, GblasError>;

pub static GRB_VERSION: usize = 1;
pub static GRB_SUB_VERSION: usize = 3;

pub fn version() -> String {
    format!("{}.{}", GRB_VERSION, GRB_SUB_VERSION)
}
