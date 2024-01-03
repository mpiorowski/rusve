use anyhow::Result;
use sendgrid::{Destination, Mail, SGClient};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EmailMessage {
    email: String,
    subject: String,
    message: String,
}

async fn send_email(message: &ReceivedMessage) -> Result<()> {
    let email_message: EmailMessage = serde_json::from_slice(&message.message.data)?;
    let sendgrid_api_key = std::env::var("SENDGRID_API_KEY").unwrap();
    let sg = SGClient::new(sendgrid_api_key);
    let mail_info = Mail::new()
        .add_to(Destination {
            address: email_message.email.as_str(),
            name: email_message.email.as_str(),
        })
        .add_from("email@rusve.app")
        .add_from_name("Rusve - rust")
        .add_subject(email_message.subject.as_str())
        .add_html(email_message.message.as_str());

    sg.send(mail_info).await?;
    Ok(())
}
