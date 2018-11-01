use relm_core::Sender;

use ::ui::windows::launcher::msg::Msg as LauncherMsg;
use super::constants::LAUNCH_WINDOW_ACTION;

/// Launches an app by the
/// given `.desktop` file path.
#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchAppAction {
    action: String,
    filename: String,
}

impl LaunchAppAction {

    pub fn new() -> Self {

        LaunchAppAction {
            action: LAUNCH_WINDOW_ACTION.to_string(),
            filename: String::new(),
        }
    }

    pub fn with_filename(name: &str) -> Self {

        LaunchAppAction {
            action: LAUNCH_WINDOW_ACTION.to_string(),
            filename: name.to_owned(),
        }
    }

    /// Allows you to define the name
    /// of the .desktop file to be
    /// launched.
    pub fn set_filename(&mut self, name: &str) {
        self.filename = name.to_owned();
    }
}

use std::sync::{
    Arc,
    Mutex,
};
use super::base_action::BaseAction;
impl BaseAction for LaunchAppAction {

    fn run(self, sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()> {

        // TODO!!
        unimplemented!();
    }
}