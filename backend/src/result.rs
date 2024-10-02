use actix_web::{body::{BoxBody, MessageBody}, http::StatusCode, HttpResponse, ResponseError};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorised,
    Hasher(password_hash::Error),
    Io(std::io::Error),
    Sql(sqlx::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This will be shown to the client; don't leak internal error details
        let reason = self.status_code()
            .canonical_reason()
            .unwrap_or("Unknown error");
        write!(f, "{reason}")
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorised => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let reason = format!("{self}");
        HttpResponse::new(self.status_code())
            .set_body(MessageBody::boxed(reason))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sql(err)
    }
}

impl From<password_hash::Error> for AppError {
    fn from(err: password_hash::Error) -> Self {
        AppError::Hasher(err)
    }
}
