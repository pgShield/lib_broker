use std::fmt;
use std::error::Error;
use thiserror::Error;

pub trait Message: Send + Sync + 'static {}

pub struct TextMessage(pub String);
impl Message for TextMessage {}

pub struct JsonMessage(pub serde_json::Value);
impl Message for JsonMessage {}

#[derive(Error, Debug)]
pub enum BrokerError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Connection error: {0}")]
    ConnectionError(String),

    #[error("Unknown message type: {0}")]
    UnknownMessageType(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Message sending failed: {0}")]
    SendError(String),

    #[error("Lock error: {0}")]
    LockError(String),

    #[error("Message receiving failed: {0}")]
    ReceiveError(String),

    #[error("Handler registration failed: {0}")]
    HandlerError(String),

    #[error("Deserialization error: {0}")]
    DeserializationError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("RabbitMQ error: {0}")]
    #[cfg(feature = "rabbitmq")]
    RabbitMQError(#[from] lapin::Error),

    #[error("Kafka error: {0}")]
    #[cfg(feature = "kafka")]
    KafkaError(#[from] rdkafka::error::KafkaError),

    #[error("Unknown error occurred")]
    Unknown,
}
