use std::sync::Arc;

use async_trait::async_trait;
use tokio::sync::mpsc::Receiver;

use crate::models::notification::Notification;

use super::{
    notification_service_interface::NotificationServiceInterface,
    provider_interfaces::NotificationProviderInterface,
};

pub struct NotificationService {
    notification_provider: Arc<dyn NotificationProviderInterface>,
    rx: Receiver<Notification>,
}

impl NotificationService {
    pub fn new(
        notification_provider: Arc<dyn NotificationProviderInterface>,
        rx: Receiver<Notification>,
    ) -> Self {
        Self {
            notification_provider,
            rx,
        }
    }
}

#[async_trait]
impl NotificationServiceInterface for NotificationService {
    async fn fetch_and_send_notification(&mut self) {
        while let Some(notification) = self.rx.recv().await {
            let provider = Arc::clone(&self.notification_provider);
            tokio::task::spawn(async move {
                let message = provider.create_message(
                    &notification.device_token,
                    &notification.title,
                    &notification.body,
                );
                if let Err(e) = provider.send_notification(message).await {
                    eprintln!("{}", e);
                }
            });
        }
    }
}
