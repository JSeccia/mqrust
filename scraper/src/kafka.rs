use std::error::Error;

use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer};

pub fn create_kafka_producer(host: &str, port: &str) -> Result<FutureProducer, Box<dyn Error>> {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", format!("{host}:{port}"))
        .set("message.timeout.ms", "5000")
        .create()?;
    Ok(producer)
}
