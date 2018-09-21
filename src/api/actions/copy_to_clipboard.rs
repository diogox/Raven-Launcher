use super::constants::COPY_TO_CLIPBOARD_ACTION;

/// Copy text to the clipboard.
#[derive(Serialize, Deserialize, Debug)]
pub struct CopyToClipboardAction {
    action: String,
    text: String,
}

impl CopyToClipboardAction {

    pub fn new() -> Self {

        CopyToClipboardAction {
            action: COPY_TO_CLIPBOARD_ACTION.to_string(),
            text: String::new(),
        }
    }

    pub fn with_text(text: &str) -> Self {

        CopyToClipboardAction {
            action: COPY_TO_CLIPBOARD_ACTION.to_string(),
            text: text.to_owned(),
        }
    }

    /// Allows you to define the text
    /// to be copied to the clipboard.
    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
    }
}


use super::base_action::BaseAction;
impl BaseAction for CopyToClipboardAction {

    fn run(self) -> Result<(), ()> {

        // TODO!!
        unimplemented!();
    }
}