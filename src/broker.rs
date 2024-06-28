use std::any::{TypeId, Any};
use std::collections::HashMap;
use std::sync::RwLock;
use async_trait::async_trait;
use lazy_static::lazy_static;
use crate::error::BrokerError;
use crate::config::Config;


#[cfg(feature = "rabbitmq")]
use crate::rabbitmq::RabbitMQClient;
#[cfg(feature = "kafka")]
use crate::kafka::KafkaClient;

pub trait Message: Send + Sync + 'static {}

#[async_trait]
pub trait Handler: Send + Sync + 'static {
    async fn handle(&self, message: Box<dyn Any + Send>) -> Result<(), BrokerError>;
}

pub struct Broker {
    handlers: RwLock<HashMap<TypeId, Box<dyn Handler>>>,
    #[cfg(feature = "rabbitmq")]
    rabbitmq: Option<RabbitMQClient>,
    #[cfg(feature = "kafka")]
    kafka: Option<KafkaClient>,
    config: Config,
}

impl Broker {
    pub async fn new(config: Config) -> Result<Self, BrokerError> {
        let broker = Self {
            handlers: RwLock::new(HashMap::new()),
            #[cfg(feature = "rabbitmq")]
            rabbitmq: None,
            #[cfg(feature = "kafka")]
            kafka: None,
            config,
        };

        #[cfg(feature = "file_logger")]
        if let Some(ref log_dir) = broker.config.log_directory {
            utils::ensure_directory_exists(log_dir)?;
            let file_appender = tracing_appender::rolling::RollingFileAppender::new(
                tracing_appender::rolling::Rotation::from(broker.config.log_rotation.clone().unwrap_or(LogRotation::Daily)),
                log_dir,
                "lib_broker.log",
            );
            let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
            tracing_subscriber::fmt()
                .with_writer(non_blocking)
                .init();
        }

        #[cfg(feature = "rabbitmq")]
        if let (Some(url), Some(exchange)) = (broker.config.rabbitmq_url.as_ref(), broker.config.rabbitmq_exchange.as_ref()) {
            broker.rabbitmq = Some(RabbitMQClient::new(url, exchange).await?);
        }

        #[cfg(feature = "kafka")]
        if let (Some(brokers), Some(topic)) = (broker.config.kafka_brokers.as_ref(), broker.config.kafka_topic.as_ref()) {
            broker.kafka = Some(KafkaClient::new(brokers, topic)?);
        }

        Ok(broker)
    }

    pub async fn send<M: Message + 'static>(&self, message: M) -> Result<(), BrokerError> {
        let handlers = self.handlers.read().map_err(|e| BrokerError::LockError(e.to_string()))?;
        
        if let Some(handler) = handlers.get(&TypeId::of::<M>()) {
            handler.handle(Box::new(message)).await?;
        }

        #[cfg(feature = "rabbitmq")]
        if let Some(ref rabbitmq) = self.rabbitmq {
            rabbitmq.publish(&message).await?;
        }

        #[cfg(feature = "kafka")]
        if let Some(ref kafka) = self.kafka {
            kafka.publish(&message).await?;
        }

        #[cfg(feature = "windows_debug")]
        if self.config.use_windows_debug {
            utils::windows_output_debug_string(&format!("Message sent: {:?}", message));
        }

        #[cfg(feature = "linux_syslog")]
        if self.config.use_linux_syslog {
            utils::linux_syslog(&format!("Message sent: {:?}", message));
        }

        Ok(())
    }

    pub fn register<M: Message + 'static, H: Handler + 'static>(&self, handler: H) -> Result<(), BrokerError> {
        let mut handlers = self.handlers.write().map_err(|e| BrokerError::LockError(e.to_string()))?;
        handlers.insert(TypeId::of::<M>(), Box::new(handler));
        Ok(())
    }
}

lazy_static! {
    static ref GLOBAL_BROKER: tokio::sync::Mutex<Option<Broker>> = tokio::sync::Mutex::new(None);
}

pub async fn get_broker() -> Result<&'static Broker, BrokerError> {
    let mut broker = GLOBAL_BROKER.lock().await;
    if broker.is_none() {
        let config = Config::from_file("lib_broker.cfg")?;
        *broker = Some(Broker::new(config).await?);
    }
    Ok(unsafe { &*(&*broker.as_ref().unwrap() as *const Broker) })
}
