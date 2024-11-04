use crate::mail::helpers::build_tls_parameters;
use crate::mail::MailConfiguration;

use axum::{extract::Form, extract::State, http::StatusCode, response::IntoResponse};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;

use std::sync::Arc;

#[derive(Deserialize, Debug, Clone)]
pub struct FormData {
    pub name: String,
    pub from: String,
    pub subject: String,
    pub message: String,
    pub phone: String,
}

pub async fn send_email(
    State(config): State<Arc<MailConfiguration>>,
    Form(form): Form<FormData>,
) -> impl IntoResponse {
    let mail = format!(
        "Name: {}\nSender: {}\nSubject: {} \nPhone:{} \n\n{}",
        &form.name, &form.from, &form.subject, &form.phone, &form.message
    );

    let email = Message::builder()
        .from(config.mail_address.parse().expect("Invalid from address"))
        .to(config.mail_address.parse().expect("Invalid to address"))
        .subject(&form.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(mail)
        .expect("Failed to build email");

    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

    // Setup SMTP transport with appropriate TLS setting
    let mailer = if config.tls && config.tls_cert.len() == 0 {
        match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp.port(config.smtp_port).credentials(creds).build(),
            Err(err) => {
                tracing::error!("Failed to create SMTP transport with TLS: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response();
            }
        }
    } else if config.tls && config.tls_cert.len() != 0 {
        let tls_result = build_tls_parameters(&config);
        let tls = match tls_result {
            Ok(tls) => tls,
            Err(err) => {
                tracing::error!("Failed to build TLS parameters: {}", err);
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to build TLS Parameters",
                )
                    .into_response();
            }
        };
        match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp
                .port(config.smtp_port)
                .tls(Tls::Wrapper(tls))
                .credentials(creds)
                .build(),
            Err(err) => {
                tracing::error!("Failed to create SMTP transport with TLS: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response();
            }
        }
    } else {
        match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp
                .tls(Tls::None) // No TLS for non-secure connections
                .port(config.smtp_port)
                .credentials(creds)
                .build(),
            Err(err) => {
                tracing::error!("Failed to create SMTP transport without TLS: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response();
            }
        }
    };

    // Send the email
    match mailer.send(&email) {
        Ok(_) => (StatusCode::OK, "Email sent successfully").into_response(),
        Err(err) => {
            tracing::error!("Failed to send email: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response()
        }
    }
}
