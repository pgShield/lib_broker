# 📬 lib_broker

lib_broker is a high-performance, cross-language message broker library primarily designed for use with pgShield, but engineered to serve as a versatile, portable SDK for various programming environments. Leveraging Rust's cross-platform capabilities and performance, it offers a modular infrastructure mediator pattern with optional RabbitMQ and Apache Kafka integration.

## 🌟 Key Aspects

1. 🛡️ **pgShield Integration**: Optimized for seamless integration with pgShield, enhancing its functionality and performance.
2. 🧩 **Portable SDK**: Designed to be easily adaptable for use in diverse projects and programming environments beyond pgShield.
3. 🌐 **Cross-Platform**: Utilizes Rust's capabilities to ensure consistent performance across different operating systems.
4. 🗣️ **Language Agnostic**: Provides easy integration with C, C++, Java, C#, and other languages, making it a versatile choice for multilingual projects.
5. 🏗️ **Modular Design**: Implements a flexible mediator pattern, allowing for easy extension and customization of messaging infrastructure.

## 🚀 Features

- 🦀 **Rust Core**: Leverages Rust's performance and safety guarantees.
- 🔌 **Cross-Language Support**: Easy integration with C, C++, Java, C#, and more.
- ⚙️ **Flexible Configuration**: Simple .cfg file for setup and customization.
- 🔧 **Optional Integrations**: 
  - 🐰 RabbitMQ support
  - 🐘 Apache Kafka support
  - 📁 File logging
  - 🪟 Windows OutputDebugString
  - 🐧 Linux syslog
- ⚡ **Asynchronous Architecture**: Built on Tokio for efficient async operations.
- 🔒 **Thread-Safe**: Designed for concurrent use in multi-threaded environments.

## 🛡️ pgShield and lib_broker

lib_broker was originally developed as a core component for pgShield, a PostgreSQL security enhancement tool. It serves as the primary message broker and communication facilitator within pgShield's architecture. However, recognizing its potential for broader applications, lib_broker has been designed with portability and flexibility in mind, allowing it to be used as a standalone SDK in various other projects.

## 📦 Installation

### Rust

Add this to your `Cargo.toml`:

```toml
[dependencies]
lib_broker = "0.1.0"
```

### Other Languages

1. Download the pre-compiled binary for your platform from the [releases page](https://github.com/pgShield/lib_broker/releases).
2. Include the library in your project:
   - For C/C++: Link against the provided .dll/.so file.
   - For Java: Use the provided JAR file with JNA.
   - For C#: Use P/Invoke to call the library functions.

## 💻 Usage

### Rust

```rust
use lib_broker::{get_broker, Message, Handler, BrokerError};

#[message_handler(TextMessage)]
fn handle_text_message(message: &TextMessage) -> Result<(), BrokerError> {
    println!("Received: {:?}", message);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), BrokerError> {
    let broker = get_broker().await?;
    broker.send(TextMessage("Hello, World!".to_string())).await?;
    Ok(())
}
```

### Java

```java
public class LibBrokerExample {
    public static void main(String[] args) {
        try (LibBroker broker = new LibBroker("lib_broker.cfg")) {
            broker.registerHandler("TextMessage", (messageType, message) -> {
                System.out.println("Received message: " + message);
                return 0; // Success
            });

            broker.send("TextMessage", "Hello from Java!");
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
}
```

### C#

```csharp
using System;

class LibBrokerExample
{
    static void Main()
    {
        using (var broker = new LibBroker("lib_broker.cfg"))
        {
            broker.RegisterHandler("TextMessage", (messageType, message) =>
            {
                Console.WriteLine($"Received message: {message}");
                return 0; // Success
            });

            broker.Send("TextMessage", "Hello from C#!");
        }
    }
}
```

## 🛠️ Configuration

Create a `lib_broker.cfg` file in your project directory:

```
# Logging configuration
log_directory = /var/log/lib_broker
log_rotation = daily
use_windows_debug = false
use_linux_syslog = false

# RabbitMQ configuration (if feature is enabled)
rabbitmq_url = amqp://guest:guest@localhost:5672
rabbitmq_exchange = lib_broker_exchange

# Kafka configuration (if feature is enabled)
kafka_brokers = localhost:9092,localhost:9093
kafka_topic = lib_broker_topic
```

## 🏗️ Building from Source

1. Ensure you have Rust and Cargo installed.
2. Clone the repository:
   ```
   git clone https://github.com/pgshield/lib_broker.git
   ```
3. Build the project:
   ```
   cd lib_broker
   cargo build --release
   ```

## 🧪 Running Tests

Execute the test suite with:

```
cargo test
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- The Rust community for their excellent documentation and crates
- Contributors who have invested time and effort to help this project

## 📬 Contact

Indrit - indrit.zeqiri@gmail.com

Project Link: [https://github.com/pgshield/lib_broker](https://github.com/pgshield/lib_broker)

---

Built with ❤️ by the pgShield team
