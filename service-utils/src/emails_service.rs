use google_cloud_default::WithAuthExt;
use google_cloud_pubsub::client::{Client, ClientConfig};
use tokio_util::sync::CancellationToken;
use tonic::Status;

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
        subscription
            .receive(
                |message, _| async move {
                    // Handle data.
                    println!("Got Message: {:?}", message.message.data);

                    // Ack or Nack message.
                    let _ = message.ack().await;
                },
                cancel.clone(),
                None,
            )
            .await
            .unwrap();
    });

    Ok(())
}
