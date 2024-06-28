use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::error::BrokerError;

#[derive(Debug, Clone)]
pub struct Config {
    #[cfg(feature = "file_logger")]
    pub log_directory: Option<String>,
    #[cfg(feature = "file_logger")]
    pub log_rotation: Option<LogRotation>,
    #[cfg(feature = "windows_debug")]
    pub use_windows_debug: bool,
    #[cfg(feature = "linux_syslog")]
    pub use_linux_syslog: bool,
    #[cfg(feature = "rabbitmq")]
    pub rabbitmq_url: Option<String>,
    #[cfg(feature = "rabbitmq")]
    pub rabbitmq_exchange: Option<String>,
    #[cfg(feature = "kafka")]
    pub kafka_brokers: Option<Vec<String>>,
    #[cfg(feature = "kafka")]
    pub kafka_topic: Option<String>,
}

#[cfg(feature = "file_logger")]
#[derive(Debug, Clone)]
pub enum LogRotation {
    Daily,
    Weekly,
    Monthly,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, BrokerError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = Config::default();

        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.starts_with('#') || line.is_empty() {
                continue;
            }
            let parts: Vec<&str> = line.splitn(2, '=').collect();
            if parts.len() != 2 {
                continue;
            }
            let key = parts[0].trim();
            let _value = parts[1].trim();

            match key {
                #[cfg(feature = "file_logger")]
                "log_directory" => config.log_directory = Some(_value.to_string()),
                #[cfg(feature = "file_logger")]
                "log_rotation" => config.log_rotation = Some(match _value.to_lowercase().as_str() {
                    "daily" => LogRotation::Daily,
                    "weekly" => LogRotation::Weekly,
                    "monthly" => LogRotation::Monthly,
                    _ => return Err(BrokerError::ConfigError("Invalid log rotation value".into())),
                }),
                #[cfg(feature = "windows_debug")]
                "use_windows_debug" => config.use_windows_debug = value.to_lowercase() == "true",
                #[cfg(feature = "linux_syslog")]
                "use_linux_syslog" => config.use_linux_syslog = value.to_lowercase() == "true",
                #[cfg(feature = "rabbitmq")]
                "rabbitmq_url" => config.rabbitmq_url = Some(value.to_string()),
                #[cfg(feature = "rabbitmq")]
                "rabbitmq_exchange" => config.rabbitmq_exchange = Some(value.to_string()),
                #[cfg(feature = "kafka")]
                "kafka_brokers" => config.kafka_brokers = Some(value.split(',').map(|s| s.trim().to_string()).collect()),
                #[cfg(feature = "kafka")]
                "kafka_topic" => config.kafka_topic = Some(value.to_string()),
                _ => return Err(BrokerError::ConfigError(format!("Unknown configuration key: {}", key))),
            }
        }

        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            #[cfg(feature = "file_logger")]
            log_directory: None,
            #[cfg(feature = "file_logger")]
            log_rotation: None,
            #[cfg(feature = "windows_debug")]
            use_windows_debug: false,
            #[cfg(feature = "linux_syslog")]
            use_linux_syslog: false,
            #[cfg(feature = "rabbitmq")]
            rabbitmq_url: None,
            #[cfg(feature = "rabbitmq")]
            rabbitmq_exchange: None,
            #[cfg(feature = "kafka")]
            kafka_brokers: None,
            #[cfg(feature = "kafka")]
            kafka_topic: None,
        }
    }
}