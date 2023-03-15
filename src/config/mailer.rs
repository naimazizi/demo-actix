use lettre::transport::smtp::authentication::Credentials;
use lettre::{AsyncSmtpTransport, AsyncStd1Executor};

use crate::config::config::Config;

pub fn init(config: &Config) -> AsyncSmtpTransport<AsyncStd1Executor> {
    let creds = Credentials::new(
        config.smtp_username.to_owned(),
        config.smtp_password.to_owned(),
    );

    let mailer: AsyncSmtpTransport<AsyncStd1Executor> =
        AsyncSmtpTransport::<AsyncStd1Executor>::starttls_relay(config.smtp_url.as_str())
            .unwrap()
            .credentials(creds)
            .build();
    mailer
}
