use lettre::{
    message::{header::ContentType, MessageBuilder, Mailbox},
    AsyncSmtpTransport, AsyncStd1Executor, AsyncTransport,
};
use log::{error, info};

use super::errors::{AppError, AppErrorType};

pub async fn send_email(
    to_email: Mailbox,
    subject: &str,
    body: &str,
    mailer: &AsyncSmtpTransport<AsyncStd1Executor>,
) -> Result<(), AppError> {
    let email = MessageBuilder::new()
        .to(to_email.to_owned())
        .from(to_email.to_owned())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(String::from(body))
        .unwrap();

    match mailer.send(email).await {
        Ok(_) => info!("Registration email {} is successfully sent", &to_email),
        Err(e) => {
            error!("Error sending email to {}. caused by: {:?}", &to_email, e);
            Err(AppError {
                cause: Some(e.to_string()),
                message: Some(format!("Failed sending email to {} ", &to_email).to_string()),
                status: AppErrorType::InternalError,
            })?
        }
    }
    Ok(())
}
