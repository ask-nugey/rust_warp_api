use crate::models::name::{Name, NameError};
use std::convert::Infallible;
use warp::{http::StatusCode, Rejection, Reply};

pub async fn greet_handler(name: Name) -> Result<impl Reply, Rejection> {
    Ok(warp::reply::html(format!("Hello, {}!", name)))
}

pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    let (code, message) = if err.is_not_found() {
        (StatusCode::NOT_FOUND, "Not Found")
    } else if let Some(e) = err.find::<NameError>() {
        (StatusCode::BAD_REQUEST, e.message())
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    };

    Ok(warp::reply::with_status(message, code))
}
