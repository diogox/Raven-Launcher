use relm_core::Sender;

use ::ui::windows::launcher::msg::Msg as LauncherMsg;
use super::constants::SET_USER_QUERY_ACTION;

/// Changes query string to a 
/// new one.
#[derive(Serialize, Deserialize, Debug)]
pub struct SetUserQueryAction {
    action: String,
    new_query: String,
}

impl SetUserQueryAction {

    pub fn new() -> Self {

        SetUserQueryAction {
            action: SET_USER_QUERY_ACTION.to_string(),
            new_query: String::new(),
        }
    }

    pub fn with_query(query: &str) -> Self {

        SetUserQueryAction {
            action: SET_USER_QUERY_ACTION.to_string(),
            new_query: query.to_owned(),
        }
    }

    /// Allows you to define the 
    /// new query to be set.
    pub fn set_query(&mut self, query: &str) {
        self.new_query = query.to_owned();
    }
}

use std::sync::{
    Arc,
    Mutex,
};
use super::base_action::BaseAction;
impl BaseAction for SetUserQueryAction {

    // Keeps the app open so that 
    // the new query can be processed. 
    fn keep_app_open(&self) -> bool { true }

    fn run(self, sender: &Arc< Mutex<Sender<LauncherMsg>> >) -> Result<(), ()> {
        unimplemented!();
    }
}