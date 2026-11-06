use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Validation failed: {0}")]
    Validation(String),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Repository error: {0}")]
    Repository(String),
}

pub type ApplicationResult<T> = Result<T, ApplicationError>;
