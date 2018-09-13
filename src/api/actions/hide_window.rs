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

use super::base_action::BaseAction;
impl BaseAction for HideWindowAction {

    fn run(self) -> Result<(), ()> { 
        // Does Nothing 
        Ok( () )
    }
}