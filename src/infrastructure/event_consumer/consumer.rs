use async_trait::async_trait;
use rdkafka::{
    consumer::{CommitMode, Consumer, StreamConsumer},
    ClientConfig, Message,
};
use tokio::sync::mpsc::Sender;

use crate::{
    models::notification::Notification, services::provider_interfaces::EventConsumerInterface,
};

pub struct EventConsumer {
    pub consumer: StreamConsumer,
    pub tx: Sender<Notification>,
}

impl EventConsumer {
    pub fn new(kafka_url: &str, tx: Sender<Notification>) -> Self {
        let mut config = ClientConfig::new();
        config
            .set("bootstrap.servers", kafka_url)
            .set("auto.offset.reset", "earliest")
            .set("group.id", "test-group")
            .set("socket.timeout.ms", "4000");

        let consumer = config.create().expect("Failure in creating consumer");
        Self { consumer, tx }
    }
}

#[async_trait]
impl EventConsumerInterface for EventConsumer {
    async fn consume_notification(&self) {
        self.consumer
            .subscribe(&["notification"])
            .expect("Error of subscribing");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    if let Some(Ok(payload)) = message.payload_view::<str>() {
                        match serde_json::from_str::<Notification>(payload) {
                            Ok(notification) => {
                                if self.tx.send(notification).await.is_err() {
                                    eprintln!("Error to send data in channel");
                                    break;
                                }
                            }
                            Err(e) => eprintln!("Error deserialize: {:?}", e),
                        }
                    } else {
                        eprintln!("Empty message");
                    }

                    if let Err(e) = self.consumer.commit_message(&message, CommitMode::Async) {
                        eprintln!("Error of commiting message: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error of getting message: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            }
        }
    }
}
