use std::thread;
use ws::listen;

use ::api::response::Response;
use super::{
    IdType,
    extension::Extension,
    extensions::EXTENSIONS,
    extension_controller::ExtensionController,
};

pub struct ExtensionServer {
    hostname: String,
    port: u16,
    extensions_dir: String,
    extensions: Vec<String>, // TODO: Change type & USE THIS
}

impl ExtensionServer {

    pub fn new() -> Self {
        use port_scanner::request_open_port;

        ExtensionServer {
            hostname: "127.0.0.1".to_owned(),
            port: request_open_port().expect("Could not find open port!"), // TODO: Find available port programatically
            extensions_dir: "./target/debug/examples/".to_owned(),
            extensions: vec!("client".to_owned()),
        }
    }

    pub fn start(&mut self) {

        // Generate endpoint
        let ws_endpoint = self.generate_ws_endpoint();
        
        // Start thread
        thread::spawn(move || {

            // Block thread and listen for connections
			listen(ws_endpoint, ExtensionController::new)
                .expect("Could not start WS server!");
		});

        // Start Extensions
        self.start_extensions();

        // ! Find a way not to need this
        thread::sleep_ms(5000);
    }

    pub fn send(&mut self, response: &Response) {

        // Get target extension's ID
        let id = response.extension_id as IdType;

        // Find extension's handler & Send message
        let extensions = EXTENSIONS.lock()
            .unwrap();

        let extension_handler = extensions
            .get(&id)
            .unwrap();
        
        extension_handler.send(&response);
    }

    // ? Should I stop the executable's proccess as well?
    pub fn stop_extension(&self, extension_id: IdType) {
        // TODO: Check if extension_id exists

        // Lock mutex & Get extension
        let extension = EXTENSIONS.lock()
            .unwrap()
            .remove(&extension_id)
            .unwrap();

        // Stop it
        // TODO: Handle error
        extension.stop().unwrap();
    }

    fn generate_ws_endpoint(&self) -> String {
        format!("{}:{}", 
            self.hostname,
            self.port,
        )
    }

    fn start_extensions(&self) {

        let executable_paths = self.extensions.iter()
            .map(|extension_name| {
                format!("{}{}", self.extensions_dir, extension_name)
            });

        let next_id: IdType = 0;
        for executable in executable_paths {
            
            Extension::builder()
                .id(next_id)
                .keyword(next_id.to_string()) // TODO
                .exec_path(executable)
                .build()
                .start(&self.generate_ws_endpoint())
                .expect("Could not start Extension!"); // TODO: Handle error
        }
    }
}