use super::constants::RUN_SCRIPT_ACTION;

#[derive(Serialize, Deserialize, Debug)]
pub struct RunScriptAction {
    action: String,
}

impl RunScriptAction {

    pub fn new() -> Self {

        RunScriptAction {
            action: RUN_SCRIPT_ACTION.to_string(),
        }
    }
}


use super::base_action::BaseAction;
impl BaseAction for RunScriptAction {
    
}