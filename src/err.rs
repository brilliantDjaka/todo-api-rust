use rocket::http::Status;
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
pub fn convert_err(err: Error) -> Status {
    match err {
        Error::InternalServerError => Status::InternalServerError,
        Error::BadRequestError => Status::BadRequest,
        Error::NotFoundError => Status::NotFound,
    }
}
