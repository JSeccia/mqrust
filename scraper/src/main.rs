use std::error::Error;
use std::fmt;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use clap::{arg, command, value_parser, ArgAction};
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;
use select::document::Document;
use select::predicate::Name;
use tokio::signal;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};


struct Row {
    name: String,
    rate: String,
    variation: String,
    high: String,
    opening: String,
    low: String,
    volume: String,
}

#[derive(Debug)]
enum ScraperError {
    Io(std::io::Error),
    Join(tokio::task::JoinError),
    Kafka(rdkafka::error::KafkaError),
    Reqwest(reqwest::Error),
    Send((rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)),
}

impl fmt::Display for ScraperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ScraperError::Io(e) => write!(f, "IO error: {e}"),
            ScraperError::Join(e) => write!(f, "Join error: {e}"),
            ScraperError::Kafka(e) => write!(f, "Kafka error: {e}"),
            ScraperError::Reqwest(e) => write!(f, "Reqwest error: {e}"),
            ScraperError::Send(e) => write!(f, "Send error: {}", e.0),
        }
    }
}


impl Error for ScraperError {}


impl From<std::io::Error> for ScraperError {
    fn from(err: std::io::Error) -> ScraperError {
        ScraperError::Io(err)
    }
}

impl From<tokio::task::JoinError> for ScraperError {
    fn from(err: tokio::task::JoinError) -> ScraperError {
        ScraperError::Join(err)
    }
}

impl From<rdkafka::error::KafkaError> for ScraperError {
    fn from(err: rdkafka::error::KafkaError) -> ScraperError {
        ScraperError::Kafka(err)
    }
}

impl From<reqwest::Error> for ScraperError {
    fn from(err: reqwest::Error) -> ScraperError {
        ScraperError::Reqwest(err)
    }
}

impl From<(rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)> for ScraperError {
    fn from(err: (rdkafka::error::KafkaError, rdkafka::message::OwnedMessage)) -> ScraperError {
        ScraperError::Send(err)
    }
}

//impl display for Row
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, rate: {}, variation: {}, high: {}, opening: {}, low: {}, volume: {}",
               self.name, self.rate, self.variation, self.high, self.opening, self.low, self.volume)
    }
}


async fn scrape(producer: &FutureProducer) -> Result<(), ScraperError> {    // Perform the async HTTP request
    let response = reqwest::get("https://www.boursier.com/indices/composition/cac-40-FR0003500008,FR.html").await?;
    let body = response.text().await?;

    // Spawn a blocking task for parsing
    let payloads = tokio::task::spawn_blocking(move || -> Result<Vec<(String, String)>, ScraperError> {
        let document = Document::from_read(Cursor::new(body))
            .map_err(ScraperError::Io)?;
        let mut payloads: Vec<(String, String)> = Vec::new();

        for row in document.find(Name("table")).next().into_iter().flat_map(|table| table.find(Name("tr"))) {
            let cells: Vec<_> = row.find(Name("td")).map(|cell| cell.text().trim().to_string()).collect();
            if cells.len() == 7 {
                let row_data = Row {
                    name: cells[0].clone(),
                    rate: cells[1].clone(),
                    variation: cells[2].clone(),
                    opening: cells[3].clone(),
                    high: cells[4].clone(),
                    low: cells[5].clone(),
                    volume: cells[6].clone(),
                };
                payloads.push((row_data.name.clone(), format!("{row_data}")));
            }
        }
        Ok(payloads)
    }).await
        .map_err(ScraperError::Join)??;

    for payload in payloads {
        println!("Sending payload: {:?}", payload);
        //
        // println!("Sending payload: {:?}", payload.0.replace([' ', '\''], ""));
        //

        let start = Instant::now();  // Start the timer

        producer.send(
            FutureRecord::to(&payload.0.replace([' ', '\''], ""))
                .payload(&payload.1)
                .key("SomeKey"),
            Duration::from_secs(5),
        )
            .await
            .map_err(ScraperError::Send)?; // Convert Kafka Error to MyError
        let duration = start.elapsed();  // Calculate the duration
        //
        println!("Time taken: {:?}", duration);
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = command!() // requires `cargo` feature
        .version("0.1")
        .author("Jayjay")
        .about("Does something.")
        .disable_help_flag(true)
        .arg(arg!(-h --host [HOST] "Sets a custom Kafka host").default_value("localhost"))
        .arg(arg!(-p --port [PORT] "Sets a custom Kafka port number").default_value("9092"))
        .arg(arg!([name] "Optional name to operate on"))
        .arg(
            arg!(-c --config <FILE> "Sets a custom config file")
                .required(false)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(arg!(-d --debug ... "Turn debugging information on"))
        .subcommand(
            command!("test")
                .about("does testing things")
                .arg(arg!(-l --list "lists test values").action(ArgAction::SetTrue)),
        )
        .get_matches();

    let host: &String = matches.get_one("host").unwrap();
    let port: &String = matches.get_one("port").unwrap();

    println!("host: {host}, port: {port}");

    let (version_n, version_s) = get_rdkafka_version();
    println!("librdkafka version: {version_s} ({version_n})");
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", format!("{host}:{port}"))
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let (shutdown_tx, mut shutdown_rx): (UnboundedSender<()>, UnboundedReceiver<()>) = unbounded_channel();

    let scrape_handle = tokio::spawn(async move {
        loop {
            // Check if a shutdown signal has been received
            if shutdown_rx.try_recv().is_ok() {
                println!("Shutdown signal received. Stopping scraping.");
                break;
            }

            if let Err(e) = scrape(&producer).await {
                println!("Scraping error: {e:?}")
            }
            tokio::select! {
            _ = tokio::time::sleep(Duration::from_secs(5)) => {
            }
            _ = shutdown_rx.recv() => {
                println!("Shutdown signal received. Stopping scraping.");
                break;
            }
        }
        }
    });



    let shutdown_signal = async {
        tokio::select! {
            _ = signal::ctrl_c() => {
                println!("SIGINT received, shutting down.");
            }
            _ = handle_sigterm() => {
                println!("SIGTERM received, shutting down.");
            }
        }
    };

    // Wait for either the shutdown signal or the scrape handle to complete
    tokio::select! {
        _ = shutdown_signal => {
            // Shutdown signal received, send shutdown message to scrape handle
            let _ = shutdown_tx.send(());
        }
        _ = scrape_handle => {
            // The scrape handle completed (which shouldn't normally happen first)
            println!("Scrape handle completed unexpectedly.");
        }
    }

    println!("Application shutdown gracefully.");
    Ok(())
}

#[cfg(unix)]
async fn handle_sigterm() {
    let mut term_signal = signal(SignalKind::terminate()).expect("Failed to set up SIGTERM handler");
    term_signal.recv().await;
    println!("SIGTERM signal received.");
}

#[cfg(not(unix))]
async fn handle_sigterm() {
    futures::future::pending::<()>().await;
    // Create a future that never resolves to keep the function running indefinitely
    println!("SIGTERM handling is not applicable in non-Unix platforms.");
}
