use std::fmt;
use std::io::Cursor;
use std::time::Duration;
use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::get_rdkafka_version;

struct Row {
    name: String,
    rate: String,
    variation: String,
    high: String,
    opening: String,
    low: String,
    volume: String,
}

//impl display for Row
impl fmt::Display for Row {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "name: {}, rate: {}, variation: {}, high: {}, opening: {}, low: {}, volume: {}",
               self.name, self.rate, self.variation, self.high, self.opening, self.low, self.volume)
    }
}


async fn scrape(producer: &FutureProducer) -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.boursier.com/indices/composition/cac-40-FR0003500008,FR.html").await?;
    let body = response.text().await?;
    let cursor = Cursor::new(body);
    let document = select::document::Document::from_read(cursor).unwrap();
    let table = document.find(select::predicate::Name("table")).next();
    if let Some(table) = table {
        for row in table.find(select::predicate::Name("tr")) {
            let cells: Vec<_> = row.find(select::predicate::Name("td"))
                .map(|cell| cell.text().trim().to_string())
                .collect();
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
                // Send row_data to Kafka
                let payload = format!("{row_data}");
                producer.send(
                    FutureRecord::to("YOUR_TOPIC_NAME")
                        .payload(&payload)
                        .key("SomeKey"), // Modify as needed
                    Duration::from_secs(0),
                ).await.unwrap();
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // ... [Kafka setup code] ...
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    loop {
        scrape(&producer).await?;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}


