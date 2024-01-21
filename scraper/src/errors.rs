use std::fmt;
use std::error::Error;

use rdkafka::error::KafkaError;
use reqwest::Error as ReqwestError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum ScraperError {
    Io(std::io::Error),
    Join(JoinError),
    Kafka(KafkaError),
    Reqwest(ReqwestError),
    Send((KafkaError, rdkafka::message::OwnedMessage)),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::Io(e) => write!(f, "IO error: {e}"),
            ScraperError::Join(e) => write!(f, "Join error: {e}"),
            ScraperError::Kafka(e) => write!(f, "Kafka error: {e}"),
            ScraperError::Reqwest(e) => write!(f, "Reqwest error: {e}"),
            ScraperError::Send((e, _)) => write!(f, "Kafka send error: {e}"),
        }
    }
}

impl Error for ScraperError {}


impl From<std::io::Error> for ScraperError {
    fn from(err: std::io::Error) -> ScraperError {
        ScraperError::Io(err)
    }
}

impl From<JoinError> for ScraperError {
    fn from(err: JoinError) -> ScraperError {
        ScraperError::Join(err)
    }
}

impl From<KafkaError> for ScraperError {
    fn from(err: KafkaError) -> ScraperError {
        ScraperError::Kafka(err)
    }
}

impl From<ReqwestError> for ScraperError {
    fn from(err: ReqwestError) -> ScraperError {
        ScraperError::Reqwest(err)
    }
}

impl From<(KafkaError, rdkafka::message::OwnedMessage)> for ScraperError {
    fn from(err: (KafkaError, rdkafka::message::OwnedMessage)) -> ScraperError {
        ScraperError::Send(err)
    }
}
