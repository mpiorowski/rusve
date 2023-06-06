use anyhow::Result;
use google_cloud_default::WithAuthExt;
use google_cloud_pubsub::{
    client::{Client, ClientConfig},
    subscriber::ReceivedMessage,
};
use sendgrid::{Destination, Mail, SGClient};
use serde::Deserialize;
use std::time::Duration;
use tokio_util::sync::CancellationToken;

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
        .add_from_name("Rusve")
        .add_subject(email_message.subject.as_str())
        .add_html(email_message.message.as_str());

    sg.send(mail_info).await?;
    Ok(())
}

pub async fn subscribe_to_emails() -> Result<()> {
    // Create pubsub client.
    let config = ClientConfig::default().with_auth().await?;
    let client = Client::new(config).await?;
    let subscription = client.subscription("email-sub");

    // Check if subscription exists.
    if !subscription.exists(None).await? {
        // let config = SubscriptionConfig {
        //     // Enable message ordering if needed (https://cloud.google.com/pubsub/docs/ordering)
        //     enable_message_ordering: true,
        //     ..Default::default()
        // };
        // subscription.create(topic.fully_qualified_name(), config, None).await?;
        return Err(anyhow::anyhow!("Subscription does not exist"));
    }

    // Token for cancel.
    let cancel = CancellationToken::new();
    let cancel2 = cancel.clone();
    tokio::spawn(async move {
        // Cancel after 10 seconds.
        tokio::time::sleep(Duration::from_secs(10)).await;
        cancel2.cancel();
    });

    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.
    subscription
        .receive(
            |message, cancel| async move {
                let _ = match send_email(&message).await {
                    Err(e) => {
                        println!("Error sending email: {:?}", e);
                        cancel.cancel();
                        message.nack().await
                    }
                    Ok(_) => message.ack().await,
                };
            },
            cancel.clone(),
            None,
        )
        .await?;

    // Delete subscription if needed.
    subscription.delete(None).await?;

    Ok(())
}
