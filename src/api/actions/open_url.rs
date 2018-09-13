use super::constants::OPEN_URL_ACTION;

/// Opens URL in the default browser
#[derive(Serialize, Deserialize, Debug)]
pub struct OpenUrlAction {
    action: String,
    url: String,
}

impl OpenUrlAction {

    pub fn new() -> Self {

        OpenUrlAction {
            action: OPEN_URL_ACTION.to_string(),
            url: String::new(),
        }    
    }

    pub fn with_url(url: &str) -> Self {

        OpenUrlAction {
            action: OPEN_URL_ACTION.to_string(),
            url: url.to_owned(),
        }
    }

    /// Allows you to define the url to open.
    pub fn url(&mut self, url: &str) {

        self.url = url.to_owned();
    }
}


use super::base_action::BaseAction;
impl BaseAction for OpenUrlAction {

    fn run(self) -> Result<(), ()> {
        use webbrowser;

        // TODO: Use correct Err type here!
        if let Ok(_) = webbrowser::open(&self.url) {
            return Ok( () )
        } else {
            return Err( () )
        }
    }
}