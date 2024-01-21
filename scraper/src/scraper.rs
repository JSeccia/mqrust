use std::io::Cursor;
use std::fmt;
use std::time::{Duration, Instant};

use rdkafka::producer::{FutureProducer, FutureRecord};
use select::document::Document;
use select::predicate::Name;
use tokio::sync::mpsc::UnboundedReceiver;

use crate::errors::ScraperError;

pub async fn run_scraper_loop(producer: FutureProducer, mut shutdown_rx: UnboundedReceiver<()>) -> Result<(), ScraperError> {
    loop {
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
    Ok(())
}

async fn scrape(producer: &FutureProducer) -> Result<(), ScraperError> {
    let response = reqwest::get("https://www.boursier.com/indices/composition/cac-40-FR0003500008,FR.html").await?;
    let body = response.text().await?;

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

#[derive(Clone, Debug)]
pub struct Row {
    pub name: String,
    pub rate: String,
    pub variation: String,
    pub high: String,
    pub opening: String,
    pub low: String,
    pub volume: String,
}

impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "name: {}, rate: {}, variation: {}, high: {}, opening: {}, low: {}, volume: {}",
            self.name, self.rate, self.variation, self.high, self.opening, self.low, self.volume
        )
    }
}
