//! Web Sockets handlers

use crate::ws::WebSocket;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use color_eyre::Result;

/// Connect the client
// Route: GET "/ws"
// ws://127.0.0.1:8089/ws
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocket {}, &req, stream);
    debug!("WS Client Response: {:?}", resp);
    resp
}
