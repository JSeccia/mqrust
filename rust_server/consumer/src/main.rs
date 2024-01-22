mod kafka_consumer;

#[macro_use]
extern crate rocket;

use futures::SinkExt;
use crate::kafka_consumer::init_new_consumer;
use rdkafka::message::Message;
use futures::stream::StreamExt;
use rdkafka::consumer::Consumer;
use rocket::State;
use rocket::response::Redirect;
use rocket_ws::{WebSocket, Channel};

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/static/index.html")
}

#[get("/echo")]
fn echo(ws: WebSocket, kafka_host: &State<String>, kafka_port: &State<u16>) -> Channel<'static> {
    let broker = format!("{}:{}", kafka_host.inner(), kafka_port.inner());
    ws.channel(move |mut stream| Box::pin(async move {

        let consumer = init_new_consumer("test-group", & broker)
            .expect("Failed to create consumer");
        let mut message_stream = consumer.stream();

        loop {
            tokio::select! {
                Some(ws_message) = stream.next() => {
                    if let Ok(msg) = & ws_message {
                        if msg.is_close() {
                            break;
                        }
                        if msg.clone().into_text()? == "toto" {
                            consumer.subscribe(&["LVMH"]).unwrap();
                        }
                        let _ = stream.send("ok toto".into()).await;
                    }
                },
                Some(result) = message_stream.next() => {
                    match & result {
                        Ok(borrowed_message) => {
                            if let Some(payload) = borrowed_message.payload() {
                                let text = std::str::from_utf8(payload).unwrap_or("");
                                if stream.send(rocket_ws::Message::text(text)).await.is_err() {
                                    break;
                                }
                            }
                        },
                        Err(e) => eprintln!("Kafka error: {}", e),
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

#[launch]
fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();

    // Extract the entire config into a `Config` struct
    // let config: Config = figment.extract().expect("config");

    // Extract a specific part of the config
    let kafka_host: String = figment.extract_inner("kafka_host").expect("kafka_host");
    let kafka_port: u16 = figment.extract_inner("kafka_port").expect("kafka_port");
    println!("Kafka Host: {}, Port: {}", kafka_host, kafka_port);
    rocket.mount("/", routes![index, echo, kafka_info])
        .manage(kafka_host)
        .manage(kafka_port)
}
