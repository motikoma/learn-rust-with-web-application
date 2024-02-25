pub mod label;
pub mod todo;

use thiserror::Error;

#[derive(Debug, Error)]
enum RepositoryError {
    #[error("Unexpected error: {0}")]
    Unexpected(String),
    #[error("Not found, id is {0}")]
    NotFound(i32),
    #[error("Duplicate data, id is {0}")]
    Duplicate(i32),
}
