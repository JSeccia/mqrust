use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;

pub fn init_new_consumer(group_id: &str, brokers: &str) -> Result<StreamConsumer, KafkaError> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer
        .subscribe(&["test-topic"])
        .expect("Can't subscribe to specified topic");

    Ok(consumer)
}
