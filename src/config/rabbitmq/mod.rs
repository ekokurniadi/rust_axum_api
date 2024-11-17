use anyhow::{Ok, Result};
use lapin::{
    options::*, types::FieldTable, BasicProperties, Channel, Connection, ConnectionProperties,
};
use serde::Serialize;
use std::env;

use crate::task::message::Message;

#[derive(Clone, Debug)]
pub struct RabbitMQ {
    pub channel: Channel,
}

impl RabbitMQ {
    pub async fn connect() -> Result<Self> {
        let addr = env::var("RABBITMQ_URL").expect("Rabbit MQ URL must be set");
        let connection = Connection::connect(&addr, ConnectionProperties::default()).await?;
        let channel = connection.create_channel().await?;

        channel
            .exchange_declare(
                "main_exchange",
                lapin::ExchangeKind::Topic,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(RabbitMQ { channel })
    }

    pub async fn publish_event<T>(&self, event: Message<T>) -> Result<()>
    where
        T: Serialize + std::fmt::Debug,
    {
        let payload = serde_json::to_vec(&event)?;
        let confirm = self
            .channel
            .basic_publish(
                "main_exchange",
                "",
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        if confirm.is_ack() {
            println!("Message successfully published with payload {:?}", event);
        } else {
            println!("Message failed to publish!");
        }

        Ok(())
    }
}
