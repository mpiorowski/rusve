use anyhow::Result;
use rusve_users::Env;
use sendgrid::{Destination, Mail, SGClient};

use crate::proto::Email;

pub async fn send_email(env: &Env, email: Email) -> Result<()> {
    let sg = SGClient::new(env.sendgrid_api_key.as_str());
    let mail_info = Mail::new()
        .add_to(Destination {
            address: email.email_to.as_str(),
            name: email.email_to.as_str(),
        })
        .add_from("email@rusve.app")
        .add_from_name("Rusve - rust")
        .add_subject(email.email_subject.as_str())
        .add_html(email.email_body.as_str());

    sg.send(mail_info).await?;
    Ok(())
}
