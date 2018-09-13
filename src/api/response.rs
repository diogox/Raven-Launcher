use serde::{
    Serialize,
    Deserialize,
};
use serde_json;

use super::events::base_event::BaseEvent;
use super::actions::base_action::BaseAction;

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    event: String,
    action: String,
}

impl<'de> Response {

    pub fn new(
        event: impl BaseEvent + Serialize + Deserialize<'de>,
        action: impl BaseAction + Serialize + Deserialize<'de>,
    ) -> Self {

        Response {
            event: serde_json::to_string(&event).unwrap(),
            action: serde_json::to_string(&action).unwrap(),
        }
    }
}