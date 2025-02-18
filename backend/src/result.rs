use actix_web::{
    body::{BoxBody, MessageBody},
    http::StatusCode,
    HttpResponse,
    ResponseError,
};

pub type Result<T, E = AppError> = std::result::Result<T, E>;

#[derive(Debug)]
pub enum AppError {
    Status(StatusCode),
    Auth(authlogic::Error),
    Mail(crate::auth::MailError),
    Io(std::io::Error),
    Sql(sqlx::Error),
}

impl AppError {
    pub const BAD_REQUEST: AppError = AppError::Status(StatusCode::BAD_REQUEST);
    pub const UNAUTHORIZED: AppError = AppError::Status(StatusCode::UNAUTHORIZED);
    pub const NOT_FOUND: AppError = AppError::Status(StatusCode::NOT_FOUND);

    pub fn http_reason(&self) -> &str {
        self.status_code()
            .canonical_reason()
            .unwrap_or("Unknown error")
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // This will be shown to the client; don't leak internal error details
        self.http_reason().fmt(f)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Auth(err) => err.status_code(),
            AppError::Status(code) => *code,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    
    fn error_response(&self) -> HttpResponse<BoxBody> {
        // Display the full error in a debug build, but just the status in a
        // release build
        let reason = if cfg!(debug_assertions) {
            format!("{self:?}") 
        } else {
            self.http_reason().to_string()
        };

        if self.status_code().is_server_error() {
            log::error!("{}: {self:?}", self.http_reason());
        }

        HttpResponse::new(self.status_code())
            .set_body(MessageBody::boxed(reason))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io(err)
    }
}

impl From<authlogic::Error> for AppError {
    fn from(err: authlogic::Error) -> Self {
        AppError::Auth(err)
    }
}

impl From<crate::auth::MailError> for AppError {
    fn from(err: crate::auth::MailError) -> Self {
        AppError::Mail(err)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Sql(err)
    }
}

pub trait OrAppError<T> {
    fn or_400_bad_request(self) -> Result<T>;
    fn or_401_unauthorised(self) -> Result<T>;
    fn or_404_not_found(self) -> Result<T>;
}

impl <T> OrAppError<T> for Option<T> {
    /// Converts this `Option` into a `Result`, replacing `None` with an HTTP
    /// "400 Bad Request" error.
    fn or_400_bad_request(self) -> Result<T> {
        self.map_or(Err(AppError::BAD_REQUEST), Ok)
    }
    
    /// Converts this `Option` into a `Result`, replacing `None` with an HTTP
    /// "401 Unauthorized" error.
    fn or_401_unauthorised(self) -> Result<T> {
        self.map_or(Err(AppError::UNAUTHORIZED), Ok)
    }
    
    /// Converts this `Option` into a `Result`, replacing `None` with an HTTP
    /// "404 Not Found" error.
    fn or_404_not_found(self) -> Result<T> {
        self.map_or(Err(AppError::NOT_FOUND), Ok)
    }
}
