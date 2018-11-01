use relm_core::Sender;

use ::ui::windows::launcher::msg::Msg as LauncherMsg;
use super::constants::HIDE_WINDOW_ACTION;

/// Hides the app. 
/// Essentialy the same as `DoNothingAction`
/// with a `keep_app_open` that always returns
/// false.
#[derive(Serialize, Deserialize, Debug)]
pub struct HideWindowAction {
    action: String,
}

impl HideWindowAction {

    pub fn new() -> Self { 
        
        HideWindowAction {
            action: HIDE_WINDOW_ACTION.to_string(),
        }
    }
}

use std::sync::{
    Arc,
    Mutex,
};
use super::base_action::BaseAction;
impl BaseAction for HideWindowAction {

    fn run(self, sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()> { 
        // Does Nothing 
        sender.lock()
            .unwrap()
            .send(LauncherMsg::Hide)
            .unwrap();
        Ok( () )
    }
}