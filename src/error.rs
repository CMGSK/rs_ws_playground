use axum::response::IntoResponse;
use http::StatusCode;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
       print!("{self:?}"); 

       return (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLI_ERR").into_response();
    }
}