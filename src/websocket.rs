use tokio::sync::mpsc;
use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::Error;
use tokio_tungstenite::tungstenite::protocol::frame::coding::FrameCode;
use tokio_tungstenite::tungstenite::protocol::frame::Frame;
use tokio_tungstenite::tungstenite::protocol::Message::Ping;
use tokio_tungstenite::tungstenite::protocol::Message::Pong;
use tokio_tungstenite::{accept_async, tungstenite};

use futures_util::stream::SplitSink;
use futures_util::sink::Sink;
use futures_util::stream::SplitStream;
use futures_util::stream::Stream;
use actix_web::{HttpRequest, HttpResponse, Error};
use futures_util::FutureExt;
use actix_web::web::Bytes;
use actix_web::HttpRequest;
use actix_web::HttpResponse;

pub async fn start_listener(mut rx: mpsc::Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Mensagem recebida: {}", msg);
    }
}

pub async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let ws = accept_async(stream)
        .await
        .expect("Falha ao conectar WebSocket");

    println!("Novo cliente WebSocket conectado");

    let (mut tx, mut rx) = ws.split();

    tokio::spawn(async move {
        loop {
            match rx.next().await {
                Some(Ok(message)) => {
                    if let Message::Text(text) = message {
                        println!("Mensagem recebida: {}", text);
                    }
                }
                Some(Err(e)) => {
                    println!("Erro no WebSocket: {}", e);
                    break;
                }
                None => break,
            }
        }
    });

    Ok(HttpResponse::Ok().body("WebSocket Conectado"))
}
