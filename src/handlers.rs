use warp::{http::Response, Filter, Rejection, Reply};

pub fn hello_route() -> warp::filters::BoxedFilter<()> {
    warp::path("hello").boxed()
}

pub fn name_route() -> warp::filters::BoxedFilter<(String,)> {
    warp::path::param().boxed()
}

pub async fn greet_handler(name: String) -> Result<impl Reply, Rejection> {
    let reply = format!("Hello, {}", name);
    Ok(Response::builder().body(reply))
}
