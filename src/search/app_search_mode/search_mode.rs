use super::super::base_search_mode::BaseSearchMode;
use super::app_db::AppDb;

pub struct AppSearchMode {
    apps: AppDb,
}

impl AppSearchMode {

    pub fn new() -> Self {


        AppSearchMode {
            apps: AppDb::new(),
        }
    }
}

use ::api::actions::base_action::BaseAction;
impl BaseSearchMode for AppSearchMode {

    /// AppSearchMode is a default search mode and is always enabled.
    fn is_enabled(&self, _query: &str) -> bool { 
        true 
    }

    fn on_query(&mut self, _query: &str) {
        return;
    }

    fn handle_query(&self, query: &str) -> Box<dyn BaseAction> { 
        use ::api::actions::RenderResultListAction;
        let results = self.apps.search(query);

        // TODO: Missing functionality reliant on other, unimplemented, search modes
        println!("{:?}", results);
        Box::new(
            RenderResultListAction::with_list(results)
        )
    }
}