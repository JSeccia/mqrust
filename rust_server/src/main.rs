mod kafka_consumer;

#[macro_use]
extern crate rocket;

use futures::SinkExt;
use crate::kafka_consumer::init_new_consumer;
use rdkafka::message::Message;
use futures::stream::StreamExt;
use rocket::response::Redirect;
use rocket_ws::{WebSocket, Channel};

#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/frontend/index.html")
}

#[get("/echo")]
fn echo(ws: WebSocket) -> Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        let consumer = init_new_consumer("test-group", "localhost:9092")
            .expect("Failed to create consumer");
        let mut message_stream = consumer.stream();

        loop {
            tokio::select! {
                Some(ws_message) = stream.next() => {
                    if let Ok(msg) = & ws_message {
                        let _ = stream.send(msg.clone()).await;
                    }
                },
                Some(result) = message_stream.next() => {
                    match result {
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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, echo])
}
