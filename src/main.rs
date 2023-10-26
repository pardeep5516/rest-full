use axum::{routing::get, Router};
use std::net::SocketAddr;
mod controller;
mod models;
use controller::routes;
#[tokio::main]
async fn main() {
    let app: Router = Router::new()
        .route("/", get(routes::index))
        .route("/user", get(routes::create_user));

    let listener: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Srver in listening on {:?}", listener);
    axum::Server::bind(&listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
