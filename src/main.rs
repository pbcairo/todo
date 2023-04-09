use axum::{*, routing::{get, post}, response::Html};
mod app;
use app::*;

#[tokio::main]
async fn main() {
    let api = Router::new()
        .route("/assets/:name", get(api::asset))
        ;

    let app = Router::new()
        .nest("/api", api)
        .route("/", get(front::index))
        .route("/todo/:name", get(front::todo_app))
        .route("/sign_up", get(front::sign_up))
        ;

    let server = Server::bind(&"0.0.0.0:5023".parse().unwrap())
        .serve(app.into_make_service())
        ;

    server.await.unwrap();
}