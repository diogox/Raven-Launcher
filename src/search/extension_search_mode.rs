use ::server::extension_server::ExtensionServer;
use ::server::deferred_result_renderer::DeferredResultRenderer;
use super::base_search_mode::BaseSearchMode;
use super::query::Query;

pub struct ExtensionSearchMode {
    server: ExtensionServer,
    deferred_result_renderer: DeferredResultRenderer,
}

impl ExtensionSearchMode {

    pub fn new() -> Self {

        let mut server = ExtensionServer::new();
        server.start();
        
        let deferred_result_renderer = DeferredResultRenderer::new();

        ExtensionSearchMode {
            server,
            deferred_result_renderer,
        }
    }
}


use ::api::actions::{
    base_action::BaseAction,
    do_nothing::DoNothingAction,
    set_user_query::SetUserQueryAction,
};
use ::api::items::{
    result_item::ResultItem,
    extension_result_item::ExtensionResultItem,
};

impl BaseSearchMode for ExtensionSearchMode {

    fn is_enabled(&self, query: &str) -> bool { 
        let query = String::from(query);
        
        let is_mode_active = query.is_mode_active();

        if is_mode_active {
            let has_extension = self.server.get_extension_id_by_keyword(query.get_keyword().unwrap())
                .is_some();
            
            return is_mode_active && has_extension;
        }

        false
    }

    /// Triggered when user changes a search query.
    fn on_query(&mut self, _query: &str) {
        self.deferred_result_renderer.on_query_change();
    }

    fn handle_query(&self, query: &str) -> Box<dyn BaseAction> { 
        let query = String::from(query);
        let extension = self.server.get_extension_id_by_keyword(query.get_keyword().unwrap());
        if extension.is_none() {
            panic!("Extension not found to handle the query!");
        }

        // TODO: extension.handle_query(&query)
        Box::new(DoNothingAction::new()) as Box<BaseAction> // ! PLACEHOLDER
    }

    fn get_searchable_items(&self) -> Vec<Box<ResultItem>> {
        let mut items = Vec::new();

        // TODO: Implement after implementing 'preferences' functionality.
        let mut item = ExtensionResultItem::new();
        item.set_keyword("hn".to_string());
        item.set_name("HackerNews".to_string());
        item.set_description("A HN api wrapper.".to_string());
        item.set_icon("".to_string());
        item.set_on_enter(SetUserQueryAction::with_query("hn "));
        items.push(Box::new(item) as Box<ResultItem>);
        
        items
    }
}
