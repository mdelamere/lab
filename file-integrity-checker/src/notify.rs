use lettre::{Message, SmtpTransport, Transport};
use lettre::message::Mailbox;
use lettre::transport::smtp::authentication::Credentials;

pub fn send_email(
    subject: &str,
    body: &str,
    to: &str,
    from: &str,
    smtp_server: &str,
    smtp_port: u16,
    username: &str,
    password: &str,
) -> Result<(), lettre::transport::smtp::Error> {
    let email = Message::builder()
        .from(from.parse::<Mailbox>().unwrap())
        .to(to.parse::<Mailbox>().unwrap())
        .subject(subject)
        .body(body.to_string())
        .unwrap();

    let creds = Credentials::new(username.to_string(), password.to_string());

    let mailer = SmtpTransport::relay(smtp_server)
        .unwrap()
        .port(smtp_port)
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}
