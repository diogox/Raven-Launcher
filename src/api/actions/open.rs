use super::constants::OPEN_ACTION;

/// Run platform a specific 
/// command to open either
/// a file or a directory.
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenAction {
    action: String,
    file_path: String, // ? Maybe Pathbuf ?
}

impl OpenAction {

    pub fn new() -> Self {

        OpenAction {
            action: OPEN_ACTION.to_string(),
            file_path: String::new(),
        }
    }

    pub fn with_path(path: &str) -> Self {

        OpenAction {
            action: OPEN_ACTION.to_string(),
            file_path: path.to_owned(),
        }
    }

    /// Allows you to define the path
    /// of the file or directory to be
    /// opened.
    pub fn set_path(&mut self, path: &str) {

        self.file_path = path.to_owned();
    }
}


use super::base_action::BaseAction;
impl BaseAction for OpenAction {

    fn run(&self) -> Result<(), ()> {

        // TODO!!
        unimplemented!();
    }
}