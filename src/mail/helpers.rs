use lettre::transport::smtp::client::{TlsParameters, Certificate};
use std::error::Error;
use std::fs;

use crate::mail::MailConfiguration;

pub fn build_tls_parameters(config: &MailConfiguration) -> Result<TlsParameters, Box<dyn Error>> {
    // Read the certificate file
    let pem_cert = fs::read(&config.tls_cert)
        .map_err(|e| format!("Failed to read certificate file: {}", e))?;
    
    // Convert PEM to rustls Certificate
    let cert = Certificate::from_pem(&pem_cert)
        .map_err(|e| format!("Failed to build certificate from PEM: {}", e))?;

    let tls = TlsParameters::builder(config.tls_domain.to_owned())
        .add_root_certificate(cert)
        .build()
        .map_err(|e| format!("Failed to build TlsParameters: {}", e))?;

    Ok(tls)
}