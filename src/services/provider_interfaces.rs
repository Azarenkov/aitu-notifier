use anyhow::Result;
use async_trait::async_trait;
use fcm_rs::models::Message;

#[async_trait]
pub trait NotificationProviderInterface: Send + Sync {
    async fn send_notification(&self, message: Message) -> Result<()>;
    fn create_message(&self, device_token: &str, title: &str, body: &str) -> Message;
}

#[async_trait]
pub trait EventConsumerInterface: Send + Sync {
    async fn consume_notification(&self);
}
