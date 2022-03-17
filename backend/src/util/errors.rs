use actix_web::*;
use actix_web::error::{Error as ActixError, BlockingError as ActixBlockingError};
use diesel::result::{DatabaseErrorKind, Error as DieselError};

///
/// These are our public error types.
///
#[derive(Fail, Debug)]
pub enum AppError {
    #[fail(display = "Invalid telegram user login")]
    TelegramAuth,

    #[fail(display = "Unauthorised")]
    Unauthorised { msg: String },

    #[fail(display = "Bad request")]
    BadClientData { msg: String },

    #[fail(display = "Internal server error")]
    Internal,

    #[fail(display = "Not found")]
    NotFound { msg: String },
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    version: u32,
    error: String,
}

impl error::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            AppError::TelegramAuth => {
                let resp = ErrorResponse {
                    version: 1,
                    error: "Invalid telegram user login".to_string(),
                };
                HttpResponse::BadRequest().json(resp)
            }
            AppError::Unauthorised { ref msg } => {
                let resp = ErrorResponse {
                    version: 1,
                    error: msg.to_string(),
                };
                HttpResponse::Unauthorized().json(resp)
            }
            AppError::BadClientData { ref msg } => {
                let resp = ErrorResponse {
                    version: 1,
                    error: msg.to_string(),
                };
                HttpResponse::BadRequest().json(resp)
            }
            AppError::NotFound { ref msg } => {
                let resp = ErrorResponse {
                    version: 1,
                    error: msg.to_string(),
                };
                HttpResponse::NotFound().json(resp)
            }
            AppError::Internal => {
                let resp = ErrorResponse {
                    version: 1,
                    error: "Internal server error".to_string(),
                };
                HttpResponse::InternalServerError().json(resp)
            }
        }
    }
}

///
/// Map Dependency crate errors to our own error types for exposing to users
///

// DB Errors should bubble up in different ways
impl From<DieselError> for AppError {
    fn from(error: DieselError) -> AppError {
        match error {
            DieselError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    // TODO: Send message through or modify?
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    error!("{:?}", message);
                    return AppError::BadClientData { msg: message };
                }

                // TODO: Add FK error
                AppError::Internal
            }
            _ => AppError::Internal,
        }
    }
}

impl From<ActixError> for AppError {
    fn from(error: ActixError) -> AppError {
        error!("Actix error: {:?}", error);
        AppError::Internal
    }
}

// Do we need this?
impl From<ActixBlockingError<DieselError>> for AppError {
    fn from(error: ActixBlockingError<DieselError>) -> AppError {
        error!("Actix blocking error: {:?}", error);
        AppError::Internal
    }
}


