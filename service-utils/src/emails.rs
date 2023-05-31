use google_cloud_default::WithAuthExt;
use google_cloud_pubsub::client::{Client, ClientConfig};
use sendgrid::{Destination, Mail, SGClient};
use tokio_util::sync::CancellationToken;
use tonic::Status;

#[derive(serde::Deserialize, Debug)]
struct EmailMessage {
    email: String,
    subject: String,
    message: String,
}

// TODO - this is done in another thread, if error occurs, it will not be logged, need to fix it
pub async fn subscribe_to_email() -> Result<(), Status> {
    let env = std::env::var("ENV").unwrap();
    if env == "development" {
        return Ok(());
    }
    // Create pubsub client.
    // TODO - make it generic
    let config = match ClientConfig::default().with_auth().await {
        Ok(config) => config,
        Err(e) => return Err(Status::internal(e.to_string())),
    };
    let client = Client::new(config).await.unwrap();

    let subscription = client.subscription("email-sub");

    // Receive blocks until the ctx is cancelled or an error occurs.
    // Or simply use the `subscription.subscribe` method.
    let cancel = CancellationToken::new();

    // move it to another thread
    tokio::spawn(async move {
        let sub = subscription
            .receive(
                |message, _| async move {
                    // Handle data.
                    let email_message: EmailMessage =
                        match serde_json::from_slice(&message.message.data) {
                            Ok(email_message) => email_message,
                            Err(e) => {
                                println!("Error: {}", e);
                                let _ = message.nack().await;
                                return;
                            }
                        };

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

                    match sg.send(mail_info) {
                        Err(err) => {
                            let _ = message.nack().await;
                            println!("Error: {}", err);
                            return;
                        }
                        Ok(body) => println!("Response: {:?}", body),
                    };
                    let _ = message.ack().await;
                },
                cancel.clone(),
                None,
            )
            .await;
        match sub {
            Ok(_) => println!("Subscription stopped"),
            Err(e) => println!("Error: {}", e),
        }
    });

    Ok(())
}
