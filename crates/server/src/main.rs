use std::net::SocketAddr;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    routing, Extension, Router,
};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use entity::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = api::build_schema().await;

    let app = Router::new()
        .route("/api/graphql", routing::get(playground).post(handler))
        .layer(Extension(schema));

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server is live on {address}");

    if let Err(e) = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
    {
        eprintln!("{e}");
    }
}

async fn playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/api/graphql",
    )))
}

async fn handler(schema: Extension<api::AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
