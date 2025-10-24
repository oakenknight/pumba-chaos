#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Timeout(String),
    BadRequest(String),
    InternalError(String),
}
