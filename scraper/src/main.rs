mod scraper;
mod errors;
mod cli;
mod kafka;
mod signal_handling;

use std::error::Error;

use rdkafka::producer::FutureProducer;
use rdkafka::util::get_rdkafka_version;
use tokio::signal;

#[cfg(unix)]
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use crate::kafka::create_kafka_producer;
use crate::signal_handling::handle_sigterm;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let (host, port) = cli::parse_arguments();

    println!("host: {host}, port: {port}");

    let (version_n, version_s) = get_rdkafka_version();
    println!("librdkafka version: {version_s} ({version_n})");
    let producer: FutureProducer = create_kafka_producer(&host, &port)?;

    let (shutdown_tx, shutdown_rx): (UnboundedSender<()>, UnboundedReceiver<()>) = unbounded_channel();

    let scrape_handle = tokio::spawn(async move {
        scraper::run_scraper_loop(producer, shutdown_rx).await
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
