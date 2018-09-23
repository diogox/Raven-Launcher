use std::sync::{
    Arc,
    Mutex,
};

use serde_json;
use ws::{
    Sender,
    Handler,
    Message,
};

use ::api::response::Response;
use super::deferred_result_renderer::DeferredResultRenderer;
use super::IdType;

#[derive(Clone)]
pub struct ExtensionController {
    extension_id: Option<IdType>,
    ws: Sender,
    deferred_result_renderer: Arc< Mutex<DeferredResultRenderer> >,
}

impl ExtensionController {

    pub fn new(ws: Sender, deferred_result_renderer: Arc< Mutex<DeferredResultRenderer> >) -> Self {
        ExtensionController {
            extension_id: None,
            ws,
            deferred_result_renderer,
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
            .expect("Could not lock extension 'on_open'");

        println!("Received 'on_open' from extension id: {}", &self.extension_id.unwrap());
        let extension = extensions.get(&self.extension_id.unwrap())
            .expect("Could not get id from 'on_open'");
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

        // ? Is it needed to check ID? Sender is unique to the client, right?
        // Ignore message if not extension controller of that id
        let extension_id = self.extension_id.unwrap();
        if extension_id == msg_sender {

            println!("Token: {:?} -> {:?}", extension_id, res.action);

            // Handle response
            self.deferred_result_renderer.lock()
                .expect("Could not get lock on result renderer!")
                .handle_response(res, extension_id);
        }
        
        Ok( () )
    }
}