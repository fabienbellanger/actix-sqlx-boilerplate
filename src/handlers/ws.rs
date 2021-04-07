//! Web Sockets handlers

use crate::errors::AppError;
use crate::ws::{chat::client::WsChatSession, chat::server::ChatServer, WebSocket};
use actix::Addr;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use askama_actix::{Template, TemplateIntoResponse};
use color_eyre::Result;
use std::time::Instant;

#[derive(Template)]
#[template(path = "ws/ws.html")]
struct WsTemplate {}

#[derive(Template)]
#[template(path = "ws/chat.html")]
struct ChatTemplate {}

// Route: GET "/ws"
// ws://127.0.0.1:8089/ws
pub async fn index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(WebSocket {}, &req, stream);
    debug!("WS Client Response: {:?}", resp);
    resp
}

// Route: GET "/ws-chat"
// ws://127.0.0.1:8089/ws-chat
pub async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
    srv: web::Data<Addr<ChatServer>>,
) -> Result<HttpResponse, Error> {
    ws::start(
        WsChatSession {
            id: 0,
            hb: Instant::now(),
            room: "Main".to_owned(),
            name: None,
            addr: srv.get_ref().clone(),
        },
        &req,
        stream,
    )
}

// Route: GET "/ws-client"
pub async fn ws_client() -> Result<HttpResponse, AppError> {
    WsTemplate {}.into_response().map_err(|e| {
        error!("{}", e);
        AppError::InternalError {
            message: "Failed to load WsTemplate.".to_owned(),
        }
    })
}

// Route: GET "/ws-chat-client"
pub async fn ws_chat_client() -> Result<HttpResponse, AppError> {
    ChatTemplate {}.into_response().map_err(|e| {
        error!("{}", e);
        AppError::InternalError {
            message: "Failed to load ChatTemplate.".to_owned(),
        }
    })
}
