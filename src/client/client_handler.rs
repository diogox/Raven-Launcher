use serde_json;
use ws::{
    Sender,
    Handler,
    Message,
};

use ::api::response::Response;
use super::IdType;

#[derive(Clone)]
pub struct ClientHandler {
    extension_id: IdType,
    ws: Sender,
}

impl ClientHandler {

    pub fn new(extension_id: IdType, ws: Sender) -> Self {

        ClientHandler {
            extension_id,
            ws,
        }
    }

    pub fn send(&mut self, message: &str) {

        // TODO: handle error
        if let Err(e) = self.ws.send(message) {
            println!("{}", e);
        }
    }

    pub fn stop(&self) {
        
        // TODO: handle error
        if let Err(e) = self.ws.shutdown() {
            println!("{}", e);
        }
    }
}

use std::str;
impl Handler for ClientHandler {

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        let res: Response = serde_json::from_str(&msg.into_text().unwrap()).unwrap();

        // TODO: run requested commands
        println!("{:?}", res.action);
        Ok( () )
    }
}