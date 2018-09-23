use std::thread;
use std::sync::{
    Arc,
    Mutex,
};
use ws::listen;

use ::api::response::Response;
use super::{
    IdType,
    extension::Extension,
    extensions::EXTENSIONS,
    extension_controller::ExtensionController,
    deferred_result_renderer::DeferredResultRenderer,
};

pub struct ExtensionServer {
    hostname: String,
    port: u16,
    extensions_dir: String,
    extensions: Vec<String>, // TODO: Change type & USE THIS
    deferred_result_renderer: Arc< Mutex<DeferredResultRenderer> >,
}

impl ExtensionServer {

    pub fn new(deferred_result_renderer: Arc< Mutex<DeferredResultRenderer> >) -> Self {
        use port_scanner::request_open_port;

        ExtensionServer {
            hostname: "127.0.0.1".to_owned(),
            port: request_open_port().expect("Could not find open port!"), // TODO: Find available port programatically
            extensions_dir: "./target/debug/examples/".to_owned(),
            extensions: vec!("client".to_owned()),
            deferred_result_renderer,
        }
    }

    pub fn start(&mut self) {

        // Generate endpoint
        let ws_endpoint = self.generate_ws_endpoint();
        
        // Get reference to deferred_result_renderer to move it to the thread
        let result_renderer = Arc::clone(&self.deferred_result_renderer);
    
        // Start thread
        thread::spawn(move || {

            // Block thread and listen for connections
			listen(ws_endpoint, |out| {
                
                ExtensionController::new(out, Arc::clone(&result_renderer))
            }).expect("Could not start WS server!");
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

    pub fn get_extension_id_by_keyword(&self, kw: &str) -> Option<IdType> {
        
        // Get extensions hashmap
        let extensions = EXTENSIONS.lock()
            .unwrap();

        // For every extension check keyword match
        for (_id, extension) in extensions.iter() {
            if extension.keyword() == kw {
                return Some(extension.id());
            }
        }

        None
    }
}