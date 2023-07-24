use std::fmt;

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalError,
    Unauthorized,
    BadRequest,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", *self)
    }
}

impl error::ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(match self {
                Self::NotFound => "Entity not found",
                Self::InternalError => "Internal server error",
                Self::Unauthorized => "Unauthorized",
                Self::BadRequest => "Bad Request",
            })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            ApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}
