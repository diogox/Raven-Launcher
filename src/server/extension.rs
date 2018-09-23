use std::process;
use std::cell::RefCell;

use ::api::response::Response;

use super::IdType;
use super::extension_controller::ExtensionController;
use super::extensions::EXTENSIONS;
use super::manifest::ExtensionManifest;

#[derive(TypedBuilder)]
pub struct Extension {

    /// The id assigned to the extension.
    /// This is also the value the extension uses
    /// as a path in the websocket so the server
    /// can recognize it.
    /// Eg. "localhost:1000/{id}"
    id: IdType,

    /// The trigger word that signals the server
    /// to use this extension when typed in by
    /// the user.
    keyword: String,

    /// The path to the extension's executable.
    /// It allows us to start the extension from here. 
    exec_path: String, // ? Pathbuf

    /// The handler responsible for managing the
    /// behaviour of the process.
    /// Necessary to kill the extension since it's
    /// independent from the server.
    // ? Can I just have the extensions close upon server disconnect ?
    #[default]
    process_handler: Option<process::Child>,
    
    /// The extension controller. It allows us to
    /// send messages and close the connection with
    /// this particular extension.
    /// It is an `Option` because its controller has
    /// to be assigned on a different thread.
    #[default]
    controller: RefCell<Option<ExtensionController>>,

    /// The extension's developer defined manifest.
    /// It holds some configurations and defines what
    /// preferences the use can set.
    #[default]
    manifest: Option<ExtensionManifest>,
}

impl Extension {
    
    // Getters
    pub fn id(&self) -> IdType { self.id }
    pub fn keyword(&self) -> &str { &self.keyword }
    pub fn exec_path(&self) -> &str { &self.exec_path }
    pub fn manifest(&self) -> &Option<ExtensionManifest> { &self.manifest }

    // Setters
    pub fn set_controller(&self, controller: ExtensionController) {
        self.controller.replace(Some(controller));
    }

    // Methods
    // TODO: Handle errors
    pub fn start(mut self, ws_endpoint: &str) -> Result<(), ()> {
        use std::process::Command;

        // Is it running already?
        if self.process_handler.is_some() {
            return Err( () )
        }

        // Is the manifest valid?
        match self.validate_manifest() {
            Ok(manifest) => {
                self.manifest = Some(manifest);
            },
            Err(_) => println!("MANIFEST INVALID")
        }

        
        // Start it
        let process_handler = Command::new(&self.exec_path)
            .env("RAVEN_WS", ws_endpoint)
            .env("RAVEN_EXTENSION_ID", self.id.to_string())
            .spawn()
            .expect("Failed to start extension!"); //TODO: Handle error

        // Save the handler
        self.process_handler = Some(process_handler);

        // Add self to list of running extensions
        println!("Inserting Extension(id: {})", self.id);
        EXTENSIONS.lock()
            .unwrap()
            .insert(self.id, self);
        
        Ok( () )
    }

    pub fn send(&self, response: &Response) {
        
        // Serialize message
        let message = serde_json::to_string(response).unwrap();

        // TODO: Handle error
        self.controller
            .borrow_mut()
            .as_mut()
            .unwrap()
            .send(&message);
    }

    pub fn stop(mut self) -> Result<(), ()> {
        
        // Is it running?
        if self.process_handler.is_none() {
            return Err( () )
        }

        // Stop it
        self.process_handler
            .unwrap()
            .kill()
            .unwrap(); // TODO: Handle error

        // Remove it from instance
        self.process_handler = None;

        // Remove self from list of running extensions
        EXTENSIONS.lock()
            .unwrap()
            .remove(&self.id);

        Ok( () )
    }

    // TODO: Refactor/Clean-up code
    fn validate_manifest(&self) -> Result<ExtensionManifest, ()> {
        use std::path::PathBuf;
        use std::fs::File;
        use std::io::Read;
        
        // Get extension directory
        let mut extension_dir = PathBuf::new();
        extension_dir.push(self.exec_path.clone());
        extension_dir.pop();
        extension_dir.push("manifest.json");

        // Open manifest file
        let mut manifest_file = File::open(extension_dir)
            .expect("file not found");

        // Read manifest
        let mut manifest = String::new();
        manifest_file.read_to_string(&mut manifest)
            .expect("something went wrong reading the file");

        // Deserialize manifest
        if let Ok(manifest) = serde_json::from_str(&manifest) {
            let manifest: ExtensionManifest = manifest;
            
            // Check manifest fields
            ExtensionManifest::validate(&manifest)
                .expect("Manifest not valid!");

            return Ok( manifest );
        } else {
            return Err( () );
        }
    }
}