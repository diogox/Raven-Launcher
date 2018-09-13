use super::constants::RENDER_RESULT_LIST_ACTION;

/// Renders list of result items.
#[derive(Serialize, Deserialize, Debug)]
pub struct RenderResultListAction {
    action: String,
    //result_list: 
}

impl RenderResultListAction {
    
    pub fn new() -> Self {

        RenderResultListAction {
            action: RENDER_RESULT_LIST_ACTION.to_string(),
        }
    }
}


use super::base_action::BaseAction;
impl BaseAction for RenderResultListAction {

    fn keep_app_open(&self) -> bool { true }

    fn run(self) -> Result<(), ()> {
        unimplemented!();
    }
}