use lettre::transport::smtp::client::{Certificate, TlsParameters};
use lettre::SmtpTransport;
use std::error::Error;
use std::fs;


use crate::mail::MailConfiguration;

pub fn build_tls_parameters(config: &MailConfiguration) -> Result<TlsParameters, Box<dyn Error>> {
    // Read the certificate file
    let pem_cert = fs::read(&config.tls_cert)
        .map_err(|e| format!("Failed to read certificate file: {}", e))?;

    // Convert PEM to rustls Certificate
    let cert = Certificate::from_pem(&pem_cert)
        .map_err(|e| format!("Failed to convert certificate to PEM: {}", e))?;

    let tls = TlsParameters::builder(config.tls_domain.to_owned())
        .add_root_certificate(cert)
        .build()
        .map_err(|e| format!("Failed to build TlsParameters: {}", e))?;

    Ok(tls)
}

#[allow(dead_code)]
pub async fn check_smtp_server(smtp_server: &str, smtp_port: u16) -> Result<(), Box<dyn Error>> {
    // Create a transport instance
    let transport = SmtpTransport::builder_dangerous(smtp_server)
        .port(smtp_port)
        .build();

    // Try to connect to the server
    match transport.test_connection() {
        Ok(_) => {
            tracing::info!("Successfully connected to the SMTP server at {}:{}", smtp_server, smtp_port);
            Ok(())
        }
        Err(err) => {
            tracing::error!("Failed to connect to the SMTP server: {}", err);
            Err(Box::new(err))
        }
    }
}
