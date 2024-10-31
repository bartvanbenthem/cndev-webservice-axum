use anyhow::Result;
use config::Config;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MailConfiguration {
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub smtp_port: u16,
    pub mail_address: String,
    pub tls: bool,
    pub tls_domain: String,
    pub tls_cert: String,
}

impl MailConfiguration {
    pub fn load() -> Result<Self> {
        // Load any .env files
        // Ignore the result of loading .env --- it's ok if it doesn't exist
        let _ = dotenvy::dotenv();

        let settings_reader = Config::builder()
            .add_source(config::File::with_name("settings").required(false))
            .add_source(config::Environment::with_prefix("MAIL"))
            .build()?;

        let settings = settings_reader.try_deserialize()?;

        Ok(settings)
    }
}
