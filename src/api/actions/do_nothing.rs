use relm_core::Sender;

use ::ui::windows::launcher::msg::Msg as LauncherMsg;
use super::constants::DO_NOTHING_ACTION;

/// Does nothing.
#[derive(Serialize, Deserialize, Debug)]
pub struct DoNothingAction {
    action: String,
    hide_app: bool,
}

impl DoNothingAction {

    pub fn new() -> Self {

        DoNothingAction {
            action: DO_NOTHING_ACTION.to_string(),
            hide_app: true, // by default
        }
    }

    pub fn should_hide_app(&mut self, should_hide: bool) {
        self.hide_app = should_hide;
    }
}

use std::sync::{
    Arc,
    Mutex,
};
use super::base_action::BaseAction;
impl BaseAction for DoNothingAction {

    fn keep_app_open(&self) -> bool { self.hide_app }

    fn run(self, _sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()> {
        // Does Nothing
        Ok( () )
    }
}