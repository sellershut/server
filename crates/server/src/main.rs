mod log;
use std::net::SocketAddr;

use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    http::{header, HeaderValue, Method},
    response::{Html, IntoResponse},
    routing, Extension, Router,
};

#[cfg(debug_assertions)]
use dotenvy::dotenv;
use entity::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};

use tower_http::{cors::CorsLayer, trace::TraceLayer};

#[tokio::main]
async fn main() {
    log::log();
    #[cfg(debug_assertions)]
    dotenv().ok();

    let schema = api::build_schema().await;

    let app = Router::new()
        .route("/api/graphql", routing::get(playground).post(handler))
        .layer(TraceLayer::new_for_http())
        .layer(Extension(schema))
        .layer(
            CorsLayer::new()
                .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
                .allow_headers([header::CONTENT_TYPE])
                .allow_methods([Method::GET, Method::POST]),
        );

    let address = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("Server is live on {address}");

    if let Err(e) = axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
    {
        tracing::error!("{e}");
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
