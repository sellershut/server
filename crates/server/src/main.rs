use std::net::SocketAddr;

use axum::{response::Html, routing, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", routing::get(handler));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is live on {address}");

    if let Err(e) = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("{e}");
    }
}

async fn handler() -> Html<&'static str> {
    Html("<p>Hello server</p>")
}
