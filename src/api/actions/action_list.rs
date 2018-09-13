use serde;
use serde_json;

use super::constants;

/// Used to run multiple
/// actions at once.
#[derive(Serialize, Deserialize, Debug)]
pub struct ActionList {
    action: String,
    actions: Vec<String>,
    keep_app_open: bool,
}

impl ActionList {

    pub fn new() -> Self {

        ActionList {
            action: constants::ACTION_LIST.to_string(),
            actions: Vec::new(),
            keep_app_open: false,
        }
    }

    pub fn with_actions(actions: Vec<String>) -> Self {

        ActionList {
            action: constants::ACTION_LIST.to_string(),
            actions,
            keep_app_open: false,
        }
    }

    /// Allows you to define the 
    /// actions to be executed.
    pub fn actions(&mut self, actions: Vec<String>) {
        self.actions = actions;
    }

    pub fn push<T>(&mut self, action: T)
        where T: BaseAction + serde::Serialize {

        // Should the app stay open after running the action?
        let keep_open = action.keep_app_open();
        if self.keep_app_open != true && keep_open {
            self.keep_app_open = keep_open;
        }

        // Save the JSON for the action
        let json_action = serde_json::to_string(&action).unwrap();
        println!("{}", json_action); // TODO: Remove this!
        self.actions.push( json_action );
    }

    // ? Implement other methods to mimic a `Vec` ?
}


macro_rules! deserialize_and_execute_if (
    ($struct_name:ident, $action_json:expr) => ({
            let action: $struct_name = serde_json::from_str(&$action_json)
                .unwrap();
            action.run()?;
    })
);

use super::base_action::BaseAction;
impl BaseAction for ActionList {

    fn keep_app_open(&self) -> bool {
        self.keep_app_open
    }

    fn run(self) -> Result<(), ()> {
        use super::*;

        for action_json in self.actions {

            // Find action type
            let value: serde_json::Value = serde_json::from_str(&action_json).unwrap();
            
            // Deserialize action
            match value.get("action").unwrap().as_str().unwrap() {
                constants::ACTION_LIST => deserialize_and_execute_if!(ActionList, action_json),
                constants::COPY_TO_CLIPBOARD_ACTION => deserialize_and_execute_if!(CopyToClipboardAction, action_json),
                constants::DO_NOTHING_ACTION => deserialize_and_execute_if!(DoNothingAction, action_json),
                //constants::EXTENSION_CUSTOM_ACTION => deserialize_and_execute_if!(ExtensionCustomAction, action_json),
                constants::HIDE_WINDOW_ACTION => deserialize_and_execute_if!(HideWindowAction, action_json),
                constants::LAUNCH_WINDOW_ACTION => deserialize_and_execute_if!(LaunchAppAction, action_json),
                constants::OPEN_URL_ACTION => deserialize_and_execute_if!(OpenUrlAction, action_json),
                constants::OPEN_ACTION => deserialize_and_execute_if!(OpenAction, action_json),
                //constants::RENDER_RESULT_LIST_ACTION => deserialize_and_execute_if!(RenderResultListAction, action_json),
                //constants::RUN_SCRIPT_ACTION => deserialize_and_execute_if!(RunScriptAction, action_json),
                constants::SET_USER_QUERY_ACTION => deserialize_and_execute_if!(SetUserQueryAction, action_json),
                _ => {}
            }
        }

        Ok( () )
    }
}