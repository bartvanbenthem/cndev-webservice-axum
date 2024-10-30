mod configuration;
mod db;
mod web_service;
use crate::auth::auth_layers;
use crate::auth::auth_layers::ListenPort;
use anyhow::Result;
use axum::{
    middleware,
    routing::{get, post},
    Extension, Router,
};
use tower_http::cors::CorsLayer;

pub async fn setup_service(listen_port: String) -> Result<Router> {
    let config = configuration::PostConfiguration::load()?;
    let db_pool = db::get_connection_pool(&config.db_filename).await?;

    db::perform_migrations(db_pool.clone()).await?;

    let secure_router = Router::new()
        .layer(CorsLayer::very_permissive())
        .layer(Extension(config.clone()))
        .layer(Extension(db_pool.clone()))
        .route("/posts/add", post(web_service::add_post))
        .route("/posts/delete/:id", get(web_service::delete_post))
        .route("/posts/update/:id", post(web_service::update_post))
        .route_layer(middleware::from_fn(auth_layers::require_remote_token))
        .layer(Extension(ListenPort(listen_port)));

    let router = Router::new()
        .merge(secure_router)
        .route("/posts", get(web_service::all_posts))
        .route("/posts/:id", get(web_service::get_post))
        .route("/about", get(web_service::get_about))
        .layer(Extension(config))
        .layer(Extension(db_pool));

    Ok(router)
}
