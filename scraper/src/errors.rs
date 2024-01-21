use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

use rdkafka::error::KafkaError;
use rdkafka::message::OwnedMessage;
use reqwest::Error as ReqwestError;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum ScraperError {
    Io(IoError),
    Join(JoinError),
    Kafka(KafkaError),
    Reqwest(ReqwestError),
    Send((KafkaError, OwnedMessage)),
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


impl From<IoError> for ScraperError {
    fn from(err: IoError) -> ScraperError {
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

impl From<(KafkaError, OwnedMessage)> for ScraperError {
    fn from(err: (KafkaError, OwnedMessage)) -> ScraperError {
        ScraperError::Send(err)
    }
}
