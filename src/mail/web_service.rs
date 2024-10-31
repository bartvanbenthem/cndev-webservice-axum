use axum::{
    extract::State,
    extract::Form,
    http::StatusCode,
    response::IntoResponse,
};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::Tls;
use lettre::{Message, SmtpTransport, Transport};
use serde::Deserialize;
use std::sync::Arc;

use super::configuration::MailConfiguration;

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
        .from(form.from.parse().expect("Invalid from address"))
        .to(config.mail_address.parse().expect("Invalid to address"))
        .subject(&form.subject)
        .header(ContentType::TEXT_PLAIN)
        .body(mail)
        .expect("Failed to build email");

    let creds = Credentials::new(config.smtp_user.clone(), config.smtp_password.clone());

    let mailer: SmtpTransport;
    if !config.tls {
        mailer = match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp
                .tls(Tls::None)
                .port(config.smtp_port)
                .credentials(creds)
                .build(),
            Err(err) => {
                eprintln!("Failed to create SMTP transport: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response();
            }
        };
    } else {
        mailer = match SmtpTransport::relay(&config.smtp_host) {
            Ok(smtp) => smtp.port(config.smtp_port).credentials(creds).build(),
            Err(err) => {
                eprintln!("Failed to create SMTP transport: {}", err);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response();
            }
        };
    }

    // Send the email
    match mailer.send(&email) {
        Ok(_) => (StatusCode::OK, "Email sent successfully").into_response(),
        Err(err) => {
            eprintln!("Failed to send email: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to send email").into_response()
        }
    }
}