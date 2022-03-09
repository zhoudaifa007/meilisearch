use meilisearch_auth::error::AuthControllerError;
use meilisearch_error::{Code, ErrorCode};

#[derive(Debug, thiserror::Error)]
pub enum AuthenticationError {
    #[error("The Authorization header is missing. It must use the bearer authorization method.")]
    MissingAuthorizationHeader,
    #[error("The provided API key is invalid.")]
    InvalidToken(String),
    #[error(transparent)]
    InvalidJwtFormat(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    AuthControllerError(#[from] AuthControllerError),
    // Triggered on configuration error.
    #[error("An internal error has occurred. `Irretrievable state`.")]
    IrretrievableState,
    #[error("TODO")]
    Todo,
}

impl ErrorCode for AuthenticationError {
    fn error_code(&self) -> Code {
        match self {
            AuthenticationError::MissingAuthorizationHeader => Code::MissingAuthorizationHeader,
            AuthenticationError::InvalidToken(_) => Code::InvalidToken,
            AuthenticationError::IrretrievableState => Code::Internal,
            AuthenticationError::InvalidJwtFormat(_) => Code::InvalidToken,
            AuthenticationError::AuthControllerError(_) => Code::InvalidToken,
            AuthenticationError::Todo => Code::Internal,
        }
    }
}
