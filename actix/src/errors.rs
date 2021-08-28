use actix_web::{
    error,
    http::header,
    http::StatusCode,
    HttpResponse,
    // dev::HttpResponseBuilder,
    HttpResponseBuilder,
};
use derive_more::{Display, Error};
use serde::Serialize;

// pub type MyResult<T> = Result<T, Err = MyError>;
pub type MyResult<T> = Result<T, MyError>;

#[derive(Debug, Display, Error, Serialize)]
pub enum MyError {
    #[display(fmt = "internal error")]
    InternalError,

    #[display(fmt = "bad request")]
    #[allow(dead_code)]
    BadClientData,

    #[display(fmt = "timeout")]
    #[allow(dead_code)]
    Timeout,

    #[display(fmt = "Sql execution error")]
    SqlError,

    #[display(fmt = "Field already exists")]
    FieldAlreadyExist,

    #[display(fmt = "login failed")]
    LoginFailed,

    #[display(fmt = "token error")]
    TokenError,
}

impl error::ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header(header::ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadClientData => StatusCode::BAD_REQUEST,
            MyError::Timeout => StatusCode::GATEWAY_TIMEOUT,
            MyError::LoginFailed => StatusCode::UNAUTHORIZED,
            MyError::TokenError => StatusCode::UNAUTHORIZED,
            MyError::FieldAlreadyExist => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
