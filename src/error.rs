use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccessError {
    #[error("Invalid role provided")]
    InvalidRole,

    #[error("User not found")]
    UserNotFound,

    #[error("Access denied")]
    AccessDenied,
}
