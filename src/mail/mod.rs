mod configuration;
mod helpers;
mod web_service;

use crate::mail::configuration::*;
use crate::mail::web_service::*;

use anyhow::Result;
use axum::{routing::post, Router};

use std::sync::Arc;

pub async fn setup_service() -> Result<Router> {
    let config = configuration::MailConfiguration::load()?;

    println!("{:?}", config);

    let router = Router::new()
        .route("/send", post(send_email))
        .with_state(Arc::new(config));

    Ok(router)
}
