use warp::Filter;

mod handlers;
mod models;

#[tokio::main]
async fn main() {
    let routes = handlers::hello_route()
        .and(handlers::name_route())
        .and_then(handlers::greet_handler);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
