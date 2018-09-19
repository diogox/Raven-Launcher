use std::thread;
use std::sync::mpsc;
use ws::connect;

use ::api::response::Response;
use super::{
    IdType,
    client_handler::ClientHandler,
};

pub struct ExtensionClient {
    extension_id: IdType,
    ws_endpoint: String,
    ws_server: Option<ClientHandler>,
}

impl ExtensionClient {

    pub fn new() -> Self {
        use std::env;
    
        // Get WS endpoint and extension id
        let env_vars = env::vars();
        
        let mut raven_ws = None;
        let mut raven_extension_id = None;
        for (key, value) in env_vars {

            if key == "RAVEN_WS" {
                raven_ws = Some(value);
            
            } else if key == "RAVEN_EXTENSION_ID" { 
                raven_extension_id = Some(value.parse()
                    .unwrap());  
            }
        }

        ExtensionClient {
            extension_id: raven_extension_id.unwrap(),
            ws_endpoint: raven_ws.unwrap(),
            ws_server: None,
        }
    }

    pub fn start(&mut self) {

        // Return error if already running
        if self.ws_server.is_some() {
            // TODO: Return error here.
            return;
        }

        let extension_id = self.extension_id;

        // Generate endpoint
        let ws_endpoint = format!("ws://{}/{}", 
            self.ws_endpoint,
            extension_id,
        );

        // Create channel to get clone of `Sender` type
        let (tx, rx) = mpsc::channel();

        // Start thread
        thread::spawn(move || {

            // Block thread and listen to the server
			connect(ws_endpoint, |out| {

                // Send clone of `Sender` type
                tx.send(out.clone()).unwrap();
                
                // Return Handler
                ClientHandler::new(extension_id, out)
            })
		});

        // Get clone of `Sender` type
        let out_clone = rx.recv().unwrap();

        // Define new Handler
        self.ws_server = Some( ClientHandler::new(extension_id, out_clone) );
    }

    pub fn send(&mut self, response: &Response) {

        let message = serde_json::to_string(response).unwrap();
        let mut ws = self.ws_server.clone().unwrap();
        ws.send(&message);
    }

    pub fn stop(&self) {
        self.ws_server
            .clone()
            .unwrap()
            .stop();
    }
}