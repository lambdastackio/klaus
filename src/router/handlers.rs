use hyper::server::{Request, Response};
use hyper::status::StatusCode;

pub fn default_404_handler(_: Request) -> Response {
    Response::new().with_status(StatusCode::NotFound)
}

pub fn method_not_supported_handler(_: Request) -> Response {
    Response::new().with_status(StatusCode::MethodNotAllowed)
}

pub fn internal_server_error_handler(_: Request) -> Response {
    Response::new().with_status(StatusCode::InternalServerError)
}

pub fn not_implemented_handler(_: Request) -> Response {
    Response::new().with_status(StatusCode::NotImplemented)
}
