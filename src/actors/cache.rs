use actix::prelude::*;

// https://github.com/fteychene/lottery-jug-rust/

// Actor
// =====
#[derive(Debug)]
pub struct Cache {
    pub list: Vec<String>,
}

impl Default for Cache {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl Actor for Cache {
    type Context = Context<Self>;
}

// Messages
// ========
#[derive(Debug)]
pub struct CacheMessage;

impl Message for CacheMessage {
    type Result = Vec<String>;
}

impl Handler<CacheMessage> for Cache {
    type Result = MessageResult<CacheMessage>;

    fn handle(&mut self, _msg: CacheMessage, _ctx: &mut Context<Self>) -> Self::Result {
        MessageResult(self.list.clone())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddMessage(pub String);

impl Handler<AddMessage> for Cache {
    type Result = ();

    fn handle(&mut self, msg: AddMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.list.push(msg.0.clone());
    }
}
