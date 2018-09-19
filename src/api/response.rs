// Here I opted for saving the event and action 
// as Strings because it allows me to ignore the
// Box<TraitName> hell. Because `Response` is 
// always meant to be used in communication 
// between server and client, doing so is not an
// issue because they would have had to be serialized
// either way.

use serde::{
    Serialize,
    Deserialize,
};
use serde_json;

use super::events::base_event::BaseEvent;
use super::actions::base_action::BaseAction;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub event: String,
    pub action: String,
    pub extension_id: u32,
}

impl<'de> Response {

    pub fn new(
        event: impl BaseEvent + Serialize + Deserialize<'de>,
        action: impl BaseAction + Serialize + Deserialize<'de>,
    ) -> Self {

        Response {
            event: serde_json::to_string(&event).unwrap(),
            action: serde_json::to_string(&action).unwrap(),
            extension_id: 0, // TODO
        }
    }
}