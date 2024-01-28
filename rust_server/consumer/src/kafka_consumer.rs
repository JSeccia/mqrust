use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::error::KafkaError;

pub fn init_new_consumer(group_id: &str, brokers: &String) -> Result<StreamConsumer, KafkaError> {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("auto.offset.reset", "earliest")
        .create()?;

    consumer
        .subscribe(&["stocks"])
        .expect("Can't subscribe to stocks topic");

    Ok(consumer)
}
