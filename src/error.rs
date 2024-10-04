use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use derive_more::From;
use sqlx::error::Error as SqlxError;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, From)]
pub enum Error {
    #[from]
    Sqlx(SqlxError),
    #[from]
    Io(std::io::Error),
    #[from]
    Uuid(uuid::Error),
    NotFound(String),
    TaskJoin(String),
}

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Sqlx(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::Io(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            Error::TaskJoin(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
            Error::Uuid(e) => (StatusCode::BAD_REQUEST, e.to_string()),
            Error::NotFound(e) => (StatusCode::NOT_FOUND, e),
        };

        (status, error_message).into_response()
    }
}
