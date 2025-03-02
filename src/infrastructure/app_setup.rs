use std::{error::Error, sync::Arc};

use fcm_rs::client::FcmClient;
use tokio::sync::mpsc::{self, Receiver, Sender};

use crate::{
    config::Config,
    models::notification::Notification,
    services::{
        notification_service::NotificationService,
        notification_service_interface::NotificationServiceInterface,
        provider_interfaces::EventConsumerInterface,
    },
};

use super::{
    event_consumer::consumer::EventConsumer,
    notifications::firebase_cloud_messaging::FirebaseMessagesClient,
};

pub struct AppDependencies {
    pub notification_service: Box<dyn NotificationServiceInterface>,
    pub event_consumer: Box<dyn EventConsumerInterface>,
}

pub async fn initialize_dependencies(config: &Config) -> Result<AppDependencies, Box<dyn Error>> {
    let fcm_client = FcmClient::new(&config.service_account_key).await?;
    let fcm = Arc::new(FirebaseMessagesClient::new(fcm_client));
    let (tx, rx): (Sender<Notification>, Receiver<Notification>) = mpsc::channel(64);
    let event_consumer = Box::new(EventConsumer::new(&config.kafka_url, tx));
    let notification_service = Box::new(NotificationService::new(fcm, rx));
    Ok(AppDependencies {
        notification_service,
        event_consumer,
    })
}

pub async fn spawn_background_tasks(
    event_consumer: Box<dyn EventConsumerInterface>,
    mut notification_service: Box<dyn NotificationServiceInterface>,
) {
    tokio::spawn(async move {
        event_consumer.consume_notification().await;
    });

    notification_service.fetch_and_send_notification().await;
}
