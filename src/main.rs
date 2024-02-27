mod handlers;
mod models;
use crate::models::name::Name;
use handlers::{greet_handler, handle_rejection};
use warp::{Filter, Rejection};

#[tokio::main]
async fn main() {
    run_server().await;
}

pub async fn run_server() {
    let routes = warp::path("hello")
        .and(warp::get())
        .and(name_route())
        .and_then(greet_handler)
        .recover(handle_rejection);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn name_route() -> impl Filter<Extract = (Name,), Error = Rejection> + Clone {
    warp::path::param().and_then(models::name::validate_name)
}
