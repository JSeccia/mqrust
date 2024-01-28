mod kafka_consumer;
mod stocks;

use std::collections::HashSet;

#[macro_use]
extern crate rocket;

use rdkafka::consumer::StreamConsumer;
use rdkafka::message::Message;
use rocket::State;
use rocket::futures::{SinkExt, StreamExt};
use rocket::response::Redirect;
use rocket_ws::{WebSocket, Channel};

use crate::kafka_consumer::init_new_consumer;

#[get("/")]
fn index() -> Redirect {
	Redirect::permanent("/static/index.html")
}

#[get("/echo")]
fn echo<'a>(ws: WebSocket, consumer: &'a State<StreamConsumer>) -> Channel<'a> {
	ws.channel(move |mut stream| Box::pin(async move {
		let mut consumer_stream = consumer.stream();
		let mut stocks: HashSet<String> = HashSet::new();

		loop {
			tokio::select! {
                Some(ws_message) = stream.next() => {
                    if let Ok(msg) = &ws_message {
                        if msg.is_close() {
                            break;
                        }
                       let new_stock = msg.clone().into_text().unwrap();
						stocks.insert(new_stock);
                    }
                },
                Some(result) = consumer_stream.next() => {
                    match &result {
                        Ok(borrowed_message) => {
							println!("message");
                            if let (Some(payload), Some(key)) = (borrowed_message.payload(), borrowed_message.key()) {
                                let key_str = std::str::from_utf8(key).unwrap_or_default();
								println!("key string: {key_str}");
                                if stocks.contains(key_str) {
                                    let text = std::str::from_utf8(payload).unwrap_or("toto not working");
                                    println!("Received message: {text}");
                                    if stream.send(rocket_ws::Message::text(text)).await.is_err() {
                                        break;
                                    }
                                }
                            }
                        },
                        Err(e) => eprintln!("Kafka error: {e}"),
                    }
                }
            }
		}
		Ok(())
	}))
}

#[get("/kafka_info")]
fn kafka_info(kafka_host: &State<String>, kafka_port: &State<u16>) -> String {
	format!("Kafka Host: {}, Port: {}", kafka_host.inner(), kafka_port.inner())
}

#[rocket::main]
async fn main() {
	let figment = rocket::Config::figment();
	let kafka_host: String = figment.extract_inner("kafka_host").expect("kafka_host");
	let kafka_port: u16 = figment.extract_inner("kafka_port").expect("kafka_port");
	let broker = format!("{}:{}", kafka_host, kafka_port);

	let consumer = init_new_consumer("stocks", &broker).expect("Failed to create consumer");

	rocket::build()
		.manage(consumer)
		.manage(kafka_host)
		.manage(kafka_port)
		.mount("/", routes![index, echo, kafka_info])
		.launch()
		.await.expect("TODO: panic message");
}
