use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something wrong with server")]
    InternalServerError(Option<String>),
    #[error("data not found")]
    NotFoundError(Option<String>),
    #[error("something wrong with client")]
    BadRequestError(Option<String>),
}
pub fn convert_err(err: Error) -> HttpResponse {
    match err {
        Error::InternalServerError(message) => {
            HttpResponse::InternalServerError().body(message.unwrap_or_default())
        }
        Error::BadRequestError(message) => {
            HttpResponse::BadRequest().body(message.unwrap_or_default())
        }
        Error::NotFoundError(message) => HttpResponse::NotFound().body(message.unwrap_or_default()),
    }
}
