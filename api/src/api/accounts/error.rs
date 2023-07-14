// use std::fmt;

// use actix_web::{error, HttpResponse, http::{header::ContentType, StatusCode}};

// #[derive(Debug)]
// pub enum AccountsApiError {
//     NotFound,
//     InternalError
// }

// impl fmt::Display for AccountsApiError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{:?}", *self)
//     }
// }


// impl error::ResponseError for AccountsApiError {
//     fn error_response(&self) -> HttpResponse {
//         HttpResponse::build(self.status_code())
//             .insert_header(ContentType::html())
//             .body(self.to_string())
//     }

//     fn status_code(&self) -> StatusCode {
//         match *self {
//             AccountsApiError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
//             AccountsApiError::NotFound => StatusCode::NOT_FOUND
//         }
//     }
// }