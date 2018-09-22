use ::api::items::ExtensionResultItem;
use super::constants::RENDER_RESULT_LIST_ACTION;

/// Renders list of result items.
#[derive(Serialize, Deserialize, Debug)]
pub struct RenderResultListAction {

    /// 
    action: String,

    // I gave up trying to make the trait version work
    /// 
    result_list: Vec< ExtensionResultItem >,
}

impl RenderResultListAction {
    
    pub fn new() -> Self {

        RenderResultListAction {
            action: RENDER_RESULT_LIST_ACTION.to_string(),
            result_list: Vec::new(),
        }
    }

    pub fn push(&mut self, result_item: ExtensionResultItem) {
        self.result_list.push(result_item);
    }
}


use super::base_action::BaseAction;
impl BaseAction for RenderResultListAction {

    fn keep_app_open(&self) -> bool { true }

    fn run(&self) -> Result<(), ()> {
        // TODO: Make this display to ui
        unimplemented!();
    }
}