use serde_json;
use ws::{
    Sender,
    Handler,
    Message,
};

use ::api::response::Response;
use super::IdType;

#[derive(Clone)]
pub struct ExtensionController {
    extension_id: Option<IdType>,
    ws: Sender,
}

impl ExtensionController {

    pub fn new(ws: Sender) -> Self {
        ExtensionController {
            extension_id: None,
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

impl Handler for ExtensionController {

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        use super::extensions::EXTENSIONS;

        // Getting path used by extension (eg. "/12")
        let extension_id = shake.request.resource();

        // Get id for the extension from the path
        self.extension_id = extension_id[1..].parse().ok();

        // Add extension id and controller (self) to the list
        // TODO: Handle Errors
        let extensions = EXTENSIONS.lock()
            .unwrap();

        let extension = extensions.get(&self.extension_id.unwrap())
            .unwrap();
        extension.set_controller(self.clone());

        Ok( () )
    }

    fn on_message(&mut self, msg: Message) -> ws::Result<()> {

        // Get JSON from message type
        let json = msg.into_text().unwrap();

        // Deserialize JSON
        let res: Response = serde_json::from_str(&json)
            .unwrap();

        // Get sender of the message
        let msg_sender = res.extension_id as IdType;

        // Ignore message if not extension controller of that id
        let extension_id = self.extension_id.unwrap();
        if extension_id == msg_sender {

            // TODO: Implement logic to handle message here
            println!("Token: {:?} -> {:?}", extension_id, res.action);
        }
        
        Ok( () )
    }
}