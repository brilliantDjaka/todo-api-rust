use actix_web::{HttpResponse, HttpResponseBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something wrong with server")]
    InternalServerError,
    #[error("data not found")]
    NotFoundError,
    #[error("something wrong with client")]
    BadRequestError,
}
pub fn convert_err(err: Error) -> HttpResponseBuilder {
    match err {
        Error::InternalServerError => HttpResponse::InternalServerError(),
        Error::BadRequestError => HttpResponse::BadRequest(),
        Error::NotFoundError => HttpResponse::NotFound(),
    }
}
