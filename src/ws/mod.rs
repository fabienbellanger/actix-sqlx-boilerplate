//! Web Socket module

use actix::{Actor, ActorContext, StreamHandler};
use actix_web_actors::ws;
use color_eyre::Result;

/// Define HTTP actor
pub struct WebSocket;

impl Actor for WebSocket {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        tracing::debug!("WS: {:?}", msg);
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => process_text_message(ctx, &text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop()
            }
            _ => ctx.stop(),
        }
    }
}

/// Process text message from the client
fn process_text_message(ctx: &mut actix_web_actors::ws::WebsocketContext<WebSocket>, msg: &str) {
    let mut s = msg.to_owned();
    s.push_str(" - from server");

    // Send message to the client
    ctx.text(&s[..])
}
