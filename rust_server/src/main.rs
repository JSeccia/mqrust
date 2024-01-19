#[macro_use]
extern crate rocket;

use rocket::response::Redirect;
use rocket::fs::{FileServer, relative};
use rocket::futures::{SinkExt, StreamExt};
use rocket::serde::{json::Json, Serialize};
use rocket_ws::{Config, WebSocket, Stream, Channel};

// Route to handle the index (React app entry point)
#[get("/")]
fn index() -> Redirect {
    Redirect::permanent("/frontend/index.html")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/ws", routes![echo])
        // .mount("/frontend", FileServer::from(relative!("path-to-your-frontend-build")))
        .mount("/api", routes![api_data]) // Example API route
}

#[get("/data")]
fn api_data() -> Json<&'static str> {
    Json("{ \"message\": \"Hello from Rocket!\" }")
}

#[get("/echo")]
fn echo(ws: WebSocket) -> Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {
        while let Some(message) = stream.next().await {
            let _ = stream.send(message?).await;
        }
        Ok(())
    }))
}
