use relm_core::Sender;

use ::ui::launcher_msg::LauncherMsg;
use super::constants::EXTENSION_CUSTOM_ACTION;

use serde_json;
use serde::{
    Serialize,
    Deserialize,
};

/// If initiated with `data`,
/// the same data will be returned
/// for further use when the action
/// is run.
#[derive(Serialize, Deserialize, Debug)]
pub struct ExtensionCustomAction {
    action: String,
    data: Option<String>,
    keep_app_open: bool,
}

impl<'de> ExtensionCustomAction {
    pub fn new() -> Self {

        ExtensionCustomAction {
            action: EXTENSION_CUSTOM_ACTION.to_string(),
            data: None,
            keep_app_open: false,
        }
    }

    pub fn with_data(data: impl Serialize + Deserialize<'de>) -> Self {

        // Serialize data
        let data_json = serde_json::to_string(&data).ok();

        ExtensionCustomAction {
            action: EXTENSION_CUSTOM_ACTION.to_string(),
            data: data_json,
            keep_app_open: false,
        }
    }

    pub fn set_data(&mut self, data: impl Serialize + Deserialize<'de>) {
        self.data = serde_json::to_string(&data).ok();
    }

    /// Allows us to specify if the app should close
    /// after the action is executed.
    // Has the same name as the method in the `BaseAction`
    // trait, but it's not a problem because users don't 
    // have access to the trait.
    pub fn keep_app_open(&mut self, keep_open: bool) {
        self.keep_app_open = keep_open;
    }
}

use std::sync::{
    Arc,
    Mutex,
};
use super::base_action::BaseAction;
impl BaseAction for ExtensionCustomAction {

    fn keep_app_open(&self) -> bool { self.keep_app_open }

    fn run(self, sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()> {

        // ? Is this usable?
        // `data` is supposed to be sent to `onItemEnter` Event. How?
        unimplemented!();
    }
}