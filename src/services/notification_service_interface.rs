use async_trait::async_trait;

#[async_trait]
pub trait NotificationServiceInterface {
    async fn fetch_and_send_notification(&mut self);
}
