use actix::prelude::*;

// https://github.com/fteychene/lottery-jug-rust/

// Actor
// =====
#[derive(Debug)]
pub struct StringList {
    pub list: Vec<String>,
}

impl Default for StringList {
    fn default() -> Self {
        Self { list: Vec::new() }
    }
}

impl Actor for StringList {
    type Context = Context<Self>;
}

// Messages
// ========
#[derive(Debug)]
pub struct StringListMessage;

impl Message for StringListMessage {
    type Result = Vec<String>;
}

impl Handler<StringListMessage> for StringList {
    type Result = MessageResult<StringListMessage>;

    fn handle(&mut self, msg: StringListMessage, _ctx: &mut Context<Self>) -> Self::Result {
        MessageResult(self.list.clone())
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddMessage(pub String);

impl Handler<AddMessage> for StringList {
    type Result = ();

    fn handle(&mut self, msg: AddMessage, _ctx: &mut Context<Self>) -> Self::Result {
        self.list.push(msg.0.clone());
    }
}
